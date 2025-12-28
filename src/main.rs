#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


// Definition der Module (entspricht den Dateien im src/ Ordner)
mod models;
mod csv_logic;
mod log_watcher;
mod app;
mod ui;

use app::EliteApp;
use models::AppSettings;
use std::path::Path;

fn main() -> eframe::Result<()> {
    // 1. Einstellungen laden
    // Wir versuchen die settings.json zu lesen. Wenn sie nicht existiert 
    // oder kaputt ist, nutzen wir die Standardwerte (Default).
    let settings: AppSettings = std::fs::read_to_string("settings.json")
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();

    // 2. Daten laden
    // Wenn in den Einstellungen bereits ein Pfad zu einer CSV steht, laden wir diese.
    let groups = if !settings.csv_path.is_empty() && Path::new(&settings.csv_path).exists() {
        csv_logic::load_and_group(&settings.csv_path).unwrap_or_default()
    } else {
        Vec::new()
    };

    // 3. Fenster-Optionen konfigurieren
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([550.0, 750.0])
            .with_min_inner_size([400.0, 500.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };
    
    // 4. Die App starten
    // Wir übergeben die geladenen Gruppen und Einstellungen an die EliteApp::new
    eframe::run_native(
        "Elite Voyager - Professional Explorer Tool",
        options,
        Box::new(|_cc| {
            Ok(Box::new(EliteApp::new(groups, settings)))
        }),
    )
}