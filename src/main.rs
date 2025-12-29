#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Definition der Module
mod models;
mod csv_logic;
mod log_watcher;
mod app;
mod audio;
mod ui;
mod constants;
mod i18n;

use app::EliteApp;
use models::AppSettings;
use std::path::Path;
use eframe::egui;

fn main() -> eframe::Result<()> {
    // 1. Einstellungen laden
    let settings_path = get_settings_path();
    let settings: AppSettings = std::fs::read_to_string(&settings_path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_else(|| {
            eprintln!("Konnte Einstellungen nicht laden, verwende Standardwerte.");
            AppSettings::default()
        });

    // 2. Daten laden
    let groups = if !settings.csv_path.is_empty() && Path::new(&settings.csv_path).exists() {
        match csv_logic::load_and_group(&settings.csv_path) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Fehler beim Laden der CSV-Datei: {}", e);
                Vec::new()
            }
        }
    } else {
        Vec::new()
    };

    // 3. Icon vorbereiten
    // Wir betten das Icon direkt in die Binary ein.
    let icon = load_app_icon();

    // 4. Fenster-Optionen konfigurieren
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([550.0, 550.0])
            .with_min_inner_size([400.0, 400.0])
            .with_drag_and_drop(true)
            .with_icon(icon), // Hier wird das Icon dem Fenster zugewiesen
        ..Default::default()
    };
    
    // 5. Die App starten
    eframe::run_native(
        "Spansh Road To Riches Companion",
        options,
        Box::new(|cc| {
            Ok(Box::new(EliteApp::new(cc, groups, settings)))
        }),
    )
}

/// Gets the path to the settings file, using OS config directory if available
fn get_settings_path() -> String {
    if let Some(config_dir) = dirs::config_dir() {
        let app_config_dir = config_dir.join("roadtoriches");
        if let Err(e) = std::fs::create_dir_all(&app_config_dir) {
            eprintln!("Konnte Config-Verzeichnis nicht erstellen: {}. Verwende aktuelles Verzeichnis.", e);
            return crate::constants::constants::SETTINGS_FILENAME.to_string();
        }
        app_config_dir.join(crate::constants::constants::SETTINGS_FILENAME)
            .to_string_lossy()
            .to_string()
    } else {
        crate::constants::constants::SETTINGS_FILENAME.to_string()
    }
}

/// Hilfsfunktion zum Laden des eingebetteten Icons
fn load_app_icon() -> egui::IconData {
    let icon_bytes = include_bytes!("../assets/icon.png");
    
    // decode_from_read ist oft stabiler
    let image = image::load_from_memory(icon_bytes)
        .unwrap_or_else(|e| {
            eprintln!("Fehler: icon.png konnte nicht dekodiert werden: {}", e);
            // Fallback: Create a minimal 1x1 image
            image::DynamicImage::ImageRgba8(image::RgbaImage::new(1, 1))
        })
        .to_rgba8(); // WICHTIG: Explizit nach RGBA8 wandeln
    
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    
    egui::IconData { rgba, width, height }
}