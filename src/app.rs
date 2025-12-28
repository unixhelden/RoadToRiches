use eframe::egui;
use crate::models::{AppSettings, SystemGroup};
use std::time::Instant;

pub struct EliteApp {
    pub settings: AppSettings,
    pub groups: Vec<SystemGroup>,
    pub current_tab: String,
    pub last_copy_time: Option<Instant>,
    // Wir speichern den Zeitpunkt des letzten Log-Checks, um die CPU zu schonen
    pub last_log_check: Instant, 
}

impl EliteApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, groups: Vec<SystemGroup>, settings: AppSettings) -> Self {
        Self {
            settings,
            groups,
            current_tab: "route".to_string(),
            last_copy_time: None,
            last_log_check: Instant::now(),
        }
    }

    pub fn save_settings(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.settings) {
            let _ = std::fs::write("settings.json", json);
        }
    }

    /// Neue Logik: Gleicht gefundene Scans aus den Journal-Logs mit der Route ab
    fn process_log_updates(&mut self) {
        // Nur alle 2 Sekunden prüfen, um die Festplatte nicht zu stressen
        if self.last_log_check.elapsed().as_secs() < 2 {
            return;
        }
        self.last_log_check = Instant::now();

        // Rufe die (bisher ungenutzten) Funktionen aus log_watcher.rs auf
        let found_scans = crate::log_watcher::check_for_scans(
            &self.settings.os_mode, 
            &self.settings.log_dir
        );

        if !found_scans.is_empty() {
            let mut changed = false;
            for scanned_body in found_scans {
                // Suche in allen Gruppen nach dem gescannten Planeten
                for group in &mut self.groups {
                    for body in &mut group.bodies {
                        // Wenn der Name übereinstimmt und noch nicht erledigt ist
                        if body.body_name == scanned_body && body.status != "erledigt" {
                            body.status = "erledigt".to_string();
                            changed = true;
                        }
                    }
                }
            }
            // Wenn etwas gefunden wurde, direkt in der CSV speichern
            if changed {
                let _ = crate::csv_logic::save_all(&self.settings.csv_path, &self.groups);
            }
        }
    }
}

impl eframe::App for EliteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Log-Check ausführen
        self.process_log_updates();

        // Elite-Theme anwenden
        crate::ui::apply_elite_theme(ctx, self.settings.dark_mode);

        egui::CentralPanel::default().show(ctx, |ui| {
            crate::ui::render_menu(self, ui);
        });

        // Kontinuierliches Repaint anfordern, damit der Log-Check läuft
        ctx.request_repaint_after(std::time::Duration::from_secs(2));
    }
}