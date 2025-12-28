use eframe::egui;
use crate::models::{AppSettings, SystemGroup};
use std::time::Instant;

/// Die zentrale Datenstruktur deiner Anwendung.
pub struct EliteApp {
    /// Die geladenen Einstellungen
    pub settings: AppSettings,
    /// Die aus der CSV gruppierten Sternensysteme
    pub groups: Vec<SystemGroup>,
    /// Speichert, welcher Tab ("route" oder "settings") aktiv ist
    pub current_tab: String,
    /// Zeitstempel für die visuelle Bestätigung beim Kopieren
    pub last_copy_time: Option<Instant>,
}

impl EliteApp {
    /// HIER IST DER FIX: Der Konstruktor akzeptiert jetzt genau das, 
    /// was du in der main.rs übergibst.
    /// Wir ignorieren 'cc' (CreationContext), da wir ihn aktuell nicht brauchen.
    pub fn new(_cc: &eframe::CreationContext<'_>, groups: Vec<SystemGroup>, settings: AppSettings) -> Self {
        Self {
            settings,
            groups,
            current_tab: "route".to_string(), // Startansicht
            last_copy_time: None,
        }
    }

    /// Speichert die Einstellungen in die settings.json
    pub fn save_settings(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.settings) {
            let _ = std::fs::write("settings.json", json);
        }
    }
}

impl eframe::App for EliteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Globales Elite-Theme anwenden
        crate::ui::apply_elite_theme(ctx, self.settings.dark_mode);

        egui::CentralPanel::default().show(ctx, |ui| {
            // Menü-Logik (Tabs) rendern
            crate::ui::render_menu(self, ui);
        });
    }
}