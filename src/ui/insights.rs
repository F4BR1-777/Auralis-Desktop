//! Auralis Insights: on-device listening estimates, no external service.
use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::Ordering::Relaxed;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use egui::{Color32, RichText, Sense, Ui, vec2, pos2};
use serde::{Deserialize, Serialize};
use crate::{model::Track, theme};
use super::{App, Action, Page, widgets::pill};

const KEEP_DAYS: u64 = 366;
const MIN_PLAY_MS: u64 = 30_000;

#[derive(Default, Clone, Serialize, Deserialize)]
struct Entry {
    title: String,
    artists: Vec<String>,
    ms: u64,
    plays: u64,
}
#[derive(Serialize, Deserialize)]
struct Ledger {
    enabled: bool,
    days: BTreeMap<u64, BTreeMap<String, Entry>>,
}
impl Default for Ledger {
    fn default() -> Self { Self { enabled: false, days: BTreeMap::new() } }
}
#[derive(Clone)]
struct Sample {
    at: Instant,
    pos: u64,
}
#[derive(Default)]
struct Meter {
    key: Option<(u64, String)>,
    last: Option<Sample>,
    heard: u64,
    counted: bool,
}
impl Meter {
    fn step(&mut self, key: (u64, String), sample: Option<Sample>) -> (u64, u64) {
        if self.key.as_ref() != Some(&key) {
            *self = Self { key: Some(key), ..Self::default() };
        }
        let Some(s) = sample else { self.last = None; return (0, 0) };
        let Some(last) = self.last.as_ref() else { self.last = Some(s); return (0, 0) };
        let elapsed = s.at.saturating_duration_since(last.at).as_millis() as u64;
        if elapsed < 1000 { return (0, 0); }
        let delta = s.pos.checked_sub(last.pos);
        self.last = Some(s);
        // Do not charge suspended time, backwards seeks or large forward jumps.
        // Small timing jitter is tolerated but never adds more than wall time.
        let ms = match delta {
            Some(d) if elapsed <= 5000 && d <= elapsed + 450 => d.min(elapsed),
            _ => 0,
        };
        self.heard = self.heard.saturating_add(ms);
        let play = !self.counted && self.heard >= MIN_PLAY_MS;
        if play { self.counted = true; }
        (ms, u64::from(play))
    }
}
#[derive(Default)]
struct Summary {
    ms: u64,
    plays: u64,
    tracks: Vec<(String, Entry)>,
    artists: Vec<(String, u64)>,
    daily: Vec<u64>,
}
pub(super) struct Insights {
    ledger: Ledger,
    meter: Meter,
    range: u64,
    confirm_clear: bool,
    // Invalidate on every data change, not every rendered frame.
    cache: Option<(u64, u64, Summary)>,
}
fn utc_day() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() / 86_400
}
impl Insights {
    pub(super) fn load(raw: Option<&str>) -> Self {
        let mut ledger: Ledger = raw.and_then(|s| serde_json::from_str(s).ok()).unwrap_or_default();
        let start = utc_day().saturating_sub(KEEP_DAYS - 1);
        ledger.days.retain(|d, _| *d >= start);
        Self { ledger, meter: Meter::default(), range: 7, confirm_clear: false, cache: None }
    }
    pub(super) fn save(&self, storage: &mut dyn eframe::Storage) {
        if let Ok(data) = serde_json::to_string(&self.ledger) {
            storage.set_string("auralis_insights_v1", data);
        }
    }
    fn record(&mut self, day: u64, track: &Track, ms: u64, plays: u64) {
        if ms == 0 { return; }
        self.ledger.days.retain(|d, _| *d >= day.saturating_sub(KEEP_DAYS - 1));
        // Keep the local history bounded, including unexpected provider metadata.
        // This affects Insights only; it never modifies the queue or track itself.
        let id: String = track.id.chars().take(128).collect();
        let day_entries = self.ledger.days.entry(day).or_default();
        if !day_entries.contains_key(&id) && day_entries.len() >= 2_000 { return; }
        let e = day_entries.entry(id).or_default();
        e.title = track.title.chars().take(240).collect();
        e.artists = track.artists.iter().take(12)
            .map(|a| a.name.trim().chars().take(120).collect())
            .filter(|n: &String| !n.is_empty()).collect();
        e.ms = e.ms.saturating_add(ms);
        e.plays = e.plays.saturating_add(plays);
        self.cache = None;
    }
    fn summary(&mut self, day: u64) -> &Summary {
        if !matches!(self.cache.as_ref(), Some((d, r, _)) if *d == day && *r == self.range) {
            self.cache = Some((day, self.range, summarize(&self.ledger, day, self.range)));
        }
        &self.cache.as_ref().expect("summary initialized").2
    }
}
fn summarize(ledger: &Ledger, day: u64, range: u64) -> Summary {
    let start = day.saturating_sub(range - 1);
    let mut s = Summary { daily: vec![0; range as usize], ..Summary::default() };
    let mut tracks: BTreeMap<String, Entry> = BTreeMap::new();
    let mut artists: BTreeMap<String, u64> = BTreeMap::new();
    for (&d, entries) in ledger.days.range(start..=day) {
        for (id, entry) in entries {
            s.ms = s.ms.saturating_add(entry.ms);
            s.plays = s.plays.saturating_add(entry.plays);
            s.daily[(d-start) as usize] += entry.ms;
            let t = tracks.entry(id.clone()).or_default();
            t.title = entry.title.clone(); t.artists = entry.artists.clone();
            t.ms += entry.ms; t.plays += entry.plays;
            // Each credited artist gets the track's time; co-credits are not additive.
            for name in entry.artists.iter().collect::<BTreeSet<_>>() {
                *artists.entry((*name).clone()).or_default() += entry.ms;
            }
        }
    }
    s.tracks = tracks.into_iter().collect();
    s.tracks.sort_by(|a,b| b.1.ms.cmp(&a.1.ms).then_with(|| a.1.title.cmp(&b.1.title)));
    s.artists = artists.into_iter().collect();
    s.artists.sort_by(|a,b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    s
}
pub(super) fn tick(app: &mut App) {
    if !app.insights.ledger.enabled { return; }
    let Some(track) = app.playing_track() else { app.insights.meter.last = None; return };
    let key = (app.generation, track.id.clone());
    let status = &app.audio.status;
    let active = status.playing.load(Relaxed) && status.playing_gen.load(Relaxed) == app.generation && !app.buffering;
    let sample = active.then(|| Sample { at: Instant::now(), pos: status.position().as_millis() as u64 });
    // Clone metadata only for a valid sample, after updating the small meter.
    let (ms, plays) = app.insights.meter.step(key, sample);
    if ms > 0 {
        if let Some(track) = app.playing_track().cloned() { app.insights.record(utc_day(), &track, ms, plays); }
    }
}
fn duration(ms: u64) -> String {
    let sec = ms / 1000;
    if sec >= 3600 { format!("{} h {:02} min", sec/3600, (sec/60)%60) }
    else if sec >= 60 { format!("{} min {:02} s", sec/60, sec%60) }
    else { format!("{sec} s") }
}
fn frame() -> egui::Frame {
    egui::Frame::new().fill(theme::panel(0.48)).stroke(egui::Stroke::new(1.0, theme::glass(0.18))).corner_radius(19).inner_margin(15)
}
fn metric(ui: &mut Ui, label: &str, value: &str, w: f32, accent: Color32) {
    frame().show(ui, |ui| {
        ui.set_width((w - 30.0).max(48.0));
        ui.label(RichText::new(label).font(theme::regular(12.0)).color(theme::text_dim()));
        ui.add_space(6.0);
        ui.label(RichText::new(value).font(theme::bold(23.0)).color(accent));
    });
}
fn graph(ui: &mut Ui, daily: &[u64], accent: Color32) {
    let n = daily.len();
    let bins = if n <= 7 { n } else if n <= 30 { 6 } else { 12 };
    let values: Vec<u64> = (0..bins).map(|i| daily[i*n/bins..(i+1)*n/bins].iter().sum()).collect();
    let peak = values.iter().copied().max().unwrap_or(0);
    ui.label(RichText::new(format!("Máximo por bloque: {}", duration(peak))).font(theme::regular(12.0)).color(theme::text_dim()));
    let max = peak.max(1) as f32;
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width().max(80.0), 150.0), Sense::hover());
    let step = rect.width()/bins as f32;
    let baseline = rect.bottom()-24.0;
    for (i, &v) in values.iter().enumerate() {
        let x = rect.left()+step*(i as f32+0.5);
        let w = (step*0.63).clamp(3.0, 46.0);
        let height = 112.0 * v as f32 / max;
        let bar = egui::Rect::from_min_max(pos2(x-w/2.0, baseline-height.max(2.0)),pos2(x+w/2.0,baseline));
        ui.painter().rect_filled(bar, 4.0, if v==0 {theme::glass(0.10)} else {accent});
        let hit = egui::Rect::from_min_max(pos2(x-step/2.0,rect.top()),pos2(x+step/2.0,baseline));
        ui.interact(hit,ui.id().with(("insights-bar",i)),Sense::hover()).on_hover_text(duration(v));
        if i==0 || i+1==bins || (bins<=7 && step>=35.0) {
            let ago = n-((i+1)*n/bins);
            let label = if ago==0 {"Hoy".into()} else {format!("-{ago} d")};
            ui.painter().text(pos2(x,baseline+11.0),egui::Align2::CENTER_CENTER,label,theme::regular(10.0),theme::text_dim());
        }
    }
}
pub(super) fn page(app: &mut App, ui: &mut Ui) {
    super::pages::page_title(ui, "Auralis Insights");
    ui.label(RichText::new("Tu música en cifras · actividad de este dispositivo").color(theme::text_dim()));
    ui.add_space(12.0);
    ui.horizontal_wrapped(|ui| {
        for (range,label) in [(7,"7 días"),(30,"30 días"),(365,"365 días")] {
            if pill(ui,label,None,app.insights.range==range).clicked() { app.insights.range=range; }
        }
        if ui.checkbox(&mut app.insights.ledger.enabled,"Registrar escuchas").changed() { app.insights.meter=Meter::default(); }
        if pill(ui,"Borrar estadísticas",None,false).clicked() { app.insights.confirm_clear=true; }
    });
    if app.insights.confirm_clear {
        ui.label("¿Eliminar toda la actividad registrada en este equipo?");
        ui.horizontal_wrapped(|ui| {
            if pill(ui,"Eliminar",None,true).clicked() {
                app.insights.ledger.days.clear(); app.insights.meter=Meter::default(); app.insights.cache=None; app.insights.confirm_clear=false;
            }
            if pill(ui,"Cancelar",None,false).clicked() { app.insights.confirm_clear=false; }
        });
    }
    let accent=app.accent_color();
    let enabled=app.insights.ledger.enabled;
    let s=app.insights.summary(utc_day());
    ui.add_space(12.0);
    let w=((ui.available_width()-30.0)/4.0).max(138.0);
    ui.horizontal_wrapped(|ui| {
        metric(ui,"Tiempo escuchado",&duration(s.ms),w,accent);
        metric(ui,"Escuchas (30 s+)",&s.plays.to_string(),w,accent);
        metric(ui,"Canciones distintas",&s.tracks.len().to_string(),w,accent);
        metric(ui,"Artistas distintos",&s.artists.len().to_string(),w,accent);
    });
    ui.add_space(16.0);
    frame().show(ui,|ui| {
        ui.label(RichText::new("Actividad musical").font(theme::bold(20.0)).color(theme::text()));
        ui.label(RichText::new("Tiempo por bloque · días UTC · periodos móviles").font(theme::regular(12.0)).color(theme::text_dim()));
        graph(ui,&s.daily,accent);
    });
    ui.add_space(16.0);
    if s.ms==0 {
        ui.label(RichText::new(if enabled {"Todavía no hay actividad. Reproduce música para comenzar tu resumen."} else {"Registro desactivado. Puedes activarlo cuando quieras."}).color(theme::text_dim()));
    } else {
        frame().show(ui,|ui| {
            ui.label(RichText::new("Tus canciones más escuchadas").font(theme::bold(20.0)).color(theme::text()));
            for (i,(_,e)) in s.tracks.iter().take(10).enumerate() {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new(format!("{:02}",i+1)).color(accent));
                    ui.label(RichText::new(e.title.as_str()).font(theme::semibold(15.0)).color(theme::text()));
                    ui.label(RichText::new(e.artists.join(", ")).color(theme::text_dim()));
                    ui.label(RichText::new(format!("{} · {} escuchas",duration(e.ms),e.plays)).color(theme::text_dim()));
                });
                ui.separator();
            }
        });
        ui.add_space(12.0);
        frame().show(ui,|ui| {
            ui.label(RichText::new("Tus artistas más escuchados").font(theme::bold(20.0)).color(theme::text()));
            let peak=s.artists.first().map(|a|a.1).unwrap_or(1).max(1);
            for (i,(name,ms)) in s.artists.iter().take(10).enumerate() {
                ui.label(RichText::new(format!("{:02}  {name}  ·  {}",i+1,duration(*ms))).color(theme::text()));
                let (r,_)=ui.allocate_exact_size(vec2(ui.available_width().max(1.0),5.0),Sense::hover());
                ui.painter().rect_filled(r,3.0,theme::glass(0.12));
                ui.painter().rect_filled(egui::Rect::from_min_size(r.min,vec2(r.width()*(*ms as f32/peak as f32),r.height())),3.0,accent);
                ui.add_space(7.0);
            }
            ui.label(RichText::new("El tiempo de una colaboración se atribuye a cada artista acreditado.").font(theme::regular(11.0)).color(theme::text_dim()));
        });
    }
    ui.add_space(12.0);
    ui.label(RichText::new("Registro local aproximado desde la fase 4. No importa el historial de YouTube; comparte estadísticas entre cuentas en este perfil de Windows. Se guardan hasta 366 días; en perfiles nuevos el registro se activa manualmente. No se envían datos de Insights a servicios externos.").font(theme::regular(12.0)).color(theme::text_dim()));
    if pill(ui,"Volver al inicio",None,false).clicked() { app.actions.push(Action::Go(Page::Home)); }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    #[test]
    fn pause_seek_threshold_and_new_generation() {
        let mut m=Meter::default(); let base=Instant::now();
        let key=(1,"x".to_string());
        let sample=|t:u64,pos:u64| Some(Sample {at:base+Duration::from_secs(t),pos});
        assert_eq!(m.step(key.clone(),sample(0,0)),(0,0));
        for i in 1..30 {assert_eq!(m.step(key.clone(),sample(i,i*1000)),(1000,0));}
        assert_eq!(m.step(key.clone(),sample(30,30000)),(1000,1));
        assert_eq!(m.step(key.clone(),None),(0,0));
        assert_eq!(m.step(key.clone(),sample(35,30000)),(0,0));
        assert_eq!(m.step(key.clone(),sample(36,120000)),(0,0));
        assert_eq!(m.step(key.clone(),sample(37,121000)),(1000,0));
        assert_eq!(m.step((2,"x".into()),sample(38,0)),(0,0));
        assert!(!m.counted);
    }
    #[test]
    fn period_totals_and_artist_dedup() {
        let mut l=Ledger::default();
        for (day,ms) in [(90,2000),(99,3000),(100,4000)] {
            l.days.entry(day).or_default().insert("x".into(),Entry {title:"X".into(),artists:vec!["A".into(),"A".into(),"B".into()],ms,plays:1});
        }
        let s=summarize(&l,100,7); assert_eq!(s.ms,7000); assert_eq!(s.plays,2); assert_eq!(s.tracks.len(),1); assert_eq!(s.artists,vec![("A".into(),7000),("B".into(),7000)]); assert_eq!(s.daily.iter().sum::<u64>(),7000);
    }
}
