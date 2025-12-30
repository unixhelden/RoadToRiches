# Developer Guide

This guide explains how to build the **Road to Riches Companion** from source.

## 🛠 Prerequisites

1.  **Rust & Cargo:** Install via [rustup.rs](https://rustup.rs/).
2.  **Git:** To clone the repository.

## 🐧 Linux Dependencies

On Linux, you need development libraries for audio (ALSA) and graphics (GTK3/X11) to compile `rodio` and `eframe`.

**Ubuntu / Debian:**
```bash
sudo apt update
sudo apt install libasound2-dev libudev-dev pkg-config libgtk-3-dev
```

**Fedora:**
```bash
sudo dnf install alsa-lib-devel systemd-devel gtk3-devel
```

## 🏗 Building

1.  **Clone the repository:**
    ```bash
    git clone <REPOSITORY_URL>
    cd RoadToRiches
    ```

2.  **Build in Release mode:**
    ```bash
    cargo build --release
    ```
    The binary will be located at `target/release/roadtoriches` (or `.exe` on Windows).

3.  **Run directly:**
    ```bash
    cargo run --release
    ```

## 🧪 Testing

To run the unit tests (e.g. for CSV logic):
```bash
cargo test
```

---

# Entwickler-Handbuch

Diese Anleitung erklärt, wie man den **Road to Riches Companion** aus dem Quellcode kompiliert.

## 🛠 Voraussetzungen

1.  **Rust & Cargo:** Installation über rustup.rs.
2.  **Git:** Zum Klonen des Repositories.

## 🐧 Linux Abhängigkeiten

Unter Linux werden Entwickler-Bibliotheken für Audio (ALSA) und Grafik (GTK3/X11) benötigt. Siehe Befehle oben im englischen Abschnitt.

## 🏗 Kompilieren & Ausführen

Nutze `cargo`, um das Projekt zu bauen und zu starten:

```bash
cargo run --release
```