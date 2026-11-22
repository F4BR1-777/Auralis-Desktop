<div align="center">

# 🎵 Auralis Desktop

### Feel Every Note.

**Un reproductor musical moderno y nativo para Windows.**

Escucha, descubre y organiza tu música con una interfaz elegante, letras sincronizadas y una experiencia de reproducción inmersiva.

[**Descargar Auralis**](https://github.com/F4BR1-777/Auralis-Desktop/releases) · [Características](#-características) · [Instalación](#-instalación) · [Compilar](#-compilar-desde-el-código-fuente)

![Windows](https://img.shields.io/badge/Windows-10%2F11-0078D4?style=flat-square)
![Rust](https://img.shields.io/badge/Rust-Native-CE422B?style=flat-square)
![License](https://img.shields.io/badge/License-GPL--3.0-blue?style=flat-square)

</div>

---

## ✨ Características

### 🎧 Reproducción musical

- Buscar canciones, artistas, álbumes y playlists mediante YouTube Music.
- Reproductor integrado con cola de reproducción, aleatorio, repetición y control de volumen.
- Biblioteca, favoritos, historial y listas de reproducción.
- Modo de reproducción inmersivo con carátula y controles dedicados.

### 🎤 Smart Karaoke

- Letras sincronizadas por línea y, cuando la fuente lo permite, por palabra.
- Iluminación progresiva durante la reproducción.
- Diseño con letras destacadas, desenfoque suave en líneas no activas y márgenes interiores.
- Tamaño de letra predeterminado equivalente al 140 % del diseño anterior.
- Respeto por el texto original de la fuente: sin añadir paréntesis ni reescribir automáticamente las frases.
- Ajustes de tiempo para compensar desfases de sincronización.

> La precisión depende de los datos disponibles para cada canción. No todas las pistas cuentan con sincronización por palabra.

### ✨ Interfaz y personalización

- Interfaz nativa desarrollada en Rust.
- Diseño visual de estilo **Liquid Glass**, con fondos dinámicos y paneles translúcidos.
- Navegación por Inicio, Buscar, Descubrir, Biblioteca, Favoritos, Historial e Insights.
- Controles de reproducción y experiencia optimizados para escritorio.

### 🔎 Descubrir e Insights

- Secciones para explorar lanzamientos y música relacionada.
- Recomendaciones basadas en las canciones escuchadas y preferencias guardadas localmente, cuando la personalización está habilitada.
- Visualización de información y estadísticas de escucha en **Auralis Insights**.

> Las recomendaciones dependen de la disponibilidad de los servicios musicales y no implican un modelo de IA generativa entrenado con tu biblioteca.

## 💻 Instalación

1. Abre la sección [**Releases**](https://github.com/F4BR1-777/Auralis-Desktop/releases).
2. Descarga el instalador más reciente para Windows, por ejemplo `Auralis-Setup-3.1.0.exe`.
3. Ejecuta el instalador y sigue los pasos.
4. Abre **Auralis** desde el acceso directo del escritorio o desde el menú Inicio.

**Requisitos:** Windows de 64 bits y las dependencias del sistema necesarias para la aplicación. El instalador puede no estar firmado digitalmente.

## 🛠️ Tecnologías

- **Rust** — aplicación nativa.
- **egui** — interfaz gráfica.
- **rodio** — reproducción de audio.
- **YouTube Music** — búsqueda y catálogo mediante integraciones no oficiales.
- **Inno Setup** — instalador para Windows.

## 🧑‍💻 Compilar desde el código fuente

Instala [Rust](https://rustup.rs/) y las herramientas de compilación de C++ para Windows. Luego ejecuta:

```powershell
git clone https://github.com/F4BR1-777/Auralis-Desktop.git
cd Auralis-Desktop
cargo build --release --locked --target x86_64-pc-windows-msvc
```

La compilación se genera en `target/x86_64-pc-windows-msvc/release/`.

**Nota:** para distribuir una compilación funcional pueden ser necesarios archivos auxiliares y dependencias de Windows. Verifica también las licencias de los recursos incluidos.

## 📌 Estado del proyecto

Auralis es un proyecto en desarrollo. Las funciones y su disponibilidad pueden cambiar entre versiones. Consulta [Releases](https://github.com/F4BR1-777/Auralis-Desktop/releases) para descargar versiones publicadas.

## ⚖️ Licencia y atribuciones

Auralis es una modificación de software de código abierto derivado de **[BitChord PC — Continued](https://github.com/jackychan4891/BitChord-PC-Continued)**. Se conservan las atribuciones y avisos de copyright correspondientes al proyecto original y sus dependencias.

Este repositorio se distribuye bajo la licencia **[GNU GPL-3.0](LICENSE)**. Consulta el archivo de licencia y los avisos de terceros para conocer las condiciones aplicables.

Auralis **no está afiliado, patrocinado ni respaldado por YouTube, Google, Apple o Spotify**. El funcionamiento de las integraciones depende de servicios externos y de sus condiciones de uso.
