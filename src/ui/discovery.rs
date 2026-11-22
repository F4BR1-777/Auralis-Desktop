//! Taste-based, on-device listening profile for Auralis Discover.
//! Only songs actually heard for 25 seconds count; no audio or lyrics are analysed.
//! Stored in the application's own eframe settings; capped to 200 distinct songs.

use std::collections::HashSet;
use crate::model::Track;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Heard {
    pub track: Track,
    pub count: u16,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Taste {
    pub enabled: bool,
    pub entries: Vec<Heard>,
    #[serde(skip)]
    active: Option<String>,
    #[serde(skip)]
    counted: bool,
}

impl Default for Taste {
    fn default() -> Self {
        Self { enabled: true, entries: vec![], active: None, counted: false }
    }
}

impl Taste {
    pub fn load(raw: Option<String>) -> Self {
        raw.and_then(|s| serde_json::from_str::<Self>(&s).ok())
            .map(|mut t| { t.entries.truncate(200); t })
            .unwrap_or_default()
    }

    pub fn observe(&mut self, track: Option<&Track>, seconds: f32, playing: bool) {
        let Some(track) = track else {
            self.active = None;
            self.counted = false;
            return;
        };
        if self.active.as_deref() != Some(track.id.as_str()) {
            self.active = Some(track.id.clone());
            self.counted = false;
        }
        if !self.enabled || !playing || self.counted || seconds < 25.0 || track.id.is_empty() {
            return;
        }
        self.counted = true;
        if let Some(i) = self.entries.iter().position(|x| x.track.id == track.id) {
            let mut h = self.entries.remove(i);
            h.count = h.count.saturating_add(1);
            h.track = track.clone();
            self.entries.insert(0, h);
        } else {
            self.entries.insert(0, Heard { track: track.clone(), count: 1 });
        }
        self.entries.truncate(200);
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.active = None;
        self.counted = false;
    }

    /// Balanced between recently heard songs and repeat listens (and likes).
    pub fn seeds(&self, liked: &HashSet<String>) -> Vec<String> {
        if !self.enabled { return vec![]; }
        let mut ranked: Vec<_> = self.entries.iter().enumerate().collect();
        ranked.sort_by_key(|(i, h)| {
            let score = (h.count.min(12) as i32 * 5)
                + if liked.contains(&h.track.id) { 28 } else { 0 }
                + (36i32 - *i as i32).max(0);
            -score
        });
        ranked.into_iter().filter(|(_, h)| !h.track.id.is_empty())
            .map(|(_, h)| h.track.id.clone()).take(3).collect()
    }

    pub fn seen(&self) -> Vec<String> {
        if !self.enabled { return vec![]; }
        self.entries.iter().map(|h| h.track.id.clone()).collect()
    }

    pub fn artist_ids(&self) -> Vec<String> {
        if !self.enabled { return vec![]; }
        let mut out = Vec::new();
        let mut seen = HashSet::new();
        for h in &self.entries {
            for a in &h.track.artists {
                if let Some(id) = &a.id {
                    if seen.insert(id.clone()) { out.push(id.clone()); }
                }
            }
        }
        out.truncate(30);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recording_requires_real_listening_and_can_be_cleared() {
        let t = Track { id: "song1".into(), title: "Example".into(), artists: vec![], album: None,
            duration: Some(210), thumbs: vec![], track_nr: None, plays: None, music_video: false };
        let mut taste = Taste::default();
        taste.observe(Some(&t), 10.0, true);
        assert!(taste.entries.is_empty());
        taste.observe(Some(&t), 25.0, true);
        assert_eq!(taste.entries.len(), 1);
        taste.observe(Some(&t), 50.0, true);
        assert_eq!(taste.entries[0].count, 1);
        taste.clear();
        assert!(taste.entries.is_empty());
    }
}
