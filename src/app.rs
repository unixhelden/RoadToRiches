use eframe::egui;
use crate::models::{AppSettings, SystemGroup};
use std::time::Instant;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use crate::update::UpdateInfo;

pub struct EliteApp {
    pub settings: AppSettings,
    pub groups: Vec<SystemGroup>,
    pub current_tab: String,
    pub last_copy_time: Option<Instant>,
    pub last_log_check: Instant,
    /// Tracks the last read byte position for each log file
    pub log_file_positions: HashMap<String, u64>,
    /// Current translations
    pub translations: crate::i18n::Translations,
    
    // --- Update Felder ---
    pub update_receiver: Option<Receiver<Option<UpdateInfo>>>,
    pub update_info: Option<UpdateInfo>,
    pub current_log_name: String,
}

impl EliteApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, groups: Vec<SystemGroup>, settings: AppSettings) -> Self {
        let translations = crate::i18n::Translations::load(settings.language);
        
        // Hier wird der Update-Thread gestartet, der in update.rs definiert ist
        let rx = crate::update::spawn_update_check();

        Self {
            settings,
            groups,
            current_tab: "route".to_string(),
            last_copy_time: None,
            last_log_check: Instant::now(),
            log_file_positions: HashMap::new(),
            translations,
            update_receiver: Some(rx),
            update_info: None,
            current_log_name: String::from("Suche Logs..."),
        }
    }
    
    pub fn reload_translations(&mut self) {
        self.translations = crate::i18n::Translations::load(self.settings.language);
    }

    pub fn save_settings(&mut self) {
        self.settings.validate_volume();
        let settings_path = self.get_settings_path();
        if let Ok(json) = serde_json::to_string_pretty(&self.settings) {
            if let Err(e) = std::fs::write(&settings_path, json) {
                eprintln!("Fehler beim Speichern der Einstellungen: {}", e);
            }
        }
    }
    
    fn get_settings_path(&self) -> String {
        if let Some(config_dir) = dirs::config_dir() {
            let app_config_dir = config_dir.join("roadtoriches");
            let _ = std::fs::create_dir_all(&app_config_dir);
            app_config_dir.join(crate::constants::constants::SETTINGS_FILENAME)
                .to_string_lossy()
                .to_string()
        } else {
            crate::constants::constants::SETTINGS_FILENAME.to_string()
        }
    }

    fn process_log_updates(&mut self) {
        use crate::constants::constants::LOG_CHECK_INTERVAL_SECS;
        
        if self.last_log_check.elapsed().as_secs() < LOG_CHECK_INTERVAL_SECS {
            return;
        }
        self.last_log_check = Instant::now();

        // Wir holen uns die Scans UND den Pfad der aktuellen Datei
        let (found_scans, log_path) = crate::log_watcher::check_for_scans(
            self.settings.os_mode, 
            &self.settings.log_dir,
            &mut self.log_file_positions
        );

        // Update den Namen der Log-Datei für die Statusleiste
        if let Some(path) = log_path {
            self.current_log_name = path.file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| "Unbekannt".to_string());
        }

        if !found_scans.is_empty() {
            use std::collections::HashSet;
            let scanned_set: HashSet<&str> = found_scans.iter().map(|s| s.as_str()).collect();
            
            let mut changed = false;
            for group in &mut self.groups {
                for body in &mut group.bodies {
                    if scanned_set.contains(body.body_name.as_str()) && !body.is_completed() {
                        body.mark_completed();
                        changed = true;
                        
                        if self.settings.sound_enabled {
                            let sound_file = if self.settings.sound_file.is_empty() {
                                None
                            } else {
                                Some(self.settings.sound_file.as_str())
                            };
                            crate::audio::play_scan_sound(self.settings.volume, sound_file);
                        }
                    }
                }
            }
            
            if changed {
                let _ = crate::csv_logic::save_all(&self.settings.csv_path, &self.groups);
            }
        }
    }
}

impl eframe::App for EliteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. Hintergrund-Checks (Logs & Updates)
        self.process_log_updates();

        // Prüfen, ob der Update-Thread eine Antwort geschickt hat
        if let Some(rx) = &self.update_receiver {
            if let Ok(result) = rx.try_recv() {
                self.update_info = result;
                self.update_receiver = None; // Kanal schließen, wir haben die Info
            }
        }

        // 2. Styling
        crate::ui::apply_elite_theme(ctx, self.settings.dark_mode);

        // 3. UI Hauptinhalt
        egui::CentralPanel::default().show(ctx, |ui| {
            crate::ui::render_menu(self, ui);
        });

        // 4. Popups / Modals (z.B. Update-Hinweis)
        crate::ui::modals::draw_update_modal(self, ctx);

        // 5. Statusleiste unten
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("📄 Journal: {}", self.current_log_name))
                    .size(14.0));
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.settings.sound_enabled {
                        let vol = (self.settings.volume * 100.0) as i32;
                        ui.label(egui::RichText::new(format!("🔊 {}%", vol)).size(14.0));
                    }
                });
            });
        });

        // Alle 2 Sekunden neu zeichnen für Log-Checks
        ctx.request_repaint_after(std::time::Duration::from_secs(2));
    }
}