use eframe::egui;
use crate::models::{AppSettings, SystemGroup};
use std::time::Instant;
use std::collections::HashMap;

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
}

impl EliteApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, groups: Vec<SystemGroup>, settings: AppSettings) -> Self {
        let translations = crate::i18n::Translations::load(settings.language);
        Self {
            settings,
            groups,
            current_tab: "route".to_string(),
            last_copy_time: None,
            last_log_check: Instant::now(),
            log_file_positions: HashMap::new(),
            translations,
        }
    }
    
    /// Reload translations when language changes
    pub fn reload_translations(&mut self) {
        self.translations = crate::i18n::Translations::load(self.settings.language);
    }

    pub fn save_settings(&mut self) {
        // Validate settings before saving
        self.settings.validate_volume();
        
        // Reload translations if language changed
        let old_language = self.settings.language;
        
        let settings_path = self.get_settings_path();
        if let Ok(json) = serde_json::to_string_pretty(&self.settings) {
            if let Err(e) = std::fs::write(&settings_path, json) {
                eprintln!("Fehler beim Speichern der Einstellungen nach {}: {}", settings_path, e);
            } else {
                // Reload translations if language changed
                if self.settings.language != old_language {
                    self.reload_translations();
                }
            }
        } else {
            eprintln!("Fehler beim Serialisieren der Einstellungen");
        }
    }
    
    fn get_settings_path(&self) -> String {
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

    fn process_log_updates(&mut self) {
        use crate::constants::constants::LOG_CHECK_INTERVAL_SECS;
        
        if self.last_log_check.elapsed().as_secs() < LOG_CHECK_INTERVAL_SECS {
            return;
        }
        self.last_log_check = Instant::now();

        let found_scans = crate::log_watcher::check_for_scans(
            self.settings.os_mode, 
            &self.settings.log_dir,
            &mut self.log_file_positions
        );

        if !found_scans.is_empty() {
            // Create a HashSet for O(1) lookup instead of O(n) nested loops
            use std::collections::HashSet;
            let scanned_set: HashSet<&str> = found_scans.iter().map(|s| s.as_str()).collect();
            
            let mut changed = false;
            for group in &mut self.groups {
                for body in &mut group.bodies {
                    if scanned_set.contains(body.body_name.as_str()) && !body.is_completed() {
                        body.mark_completed();
                        changed = true;
                        
                        // Play sound if enabled
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
                if let Err(e) = crate::csv_logic::save_all(&self.settings.csv_path, &self.groups) {
                    eprintln!("Fehler beim Speichern der CSV: {}", e);
                }
            }
        }
    }
}

impl eframe::App for EliteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.process_log_updates();
        crate::ui::apply_elite_theme(ctx, self.settings.dark_mode);

        egui::CentralPanel::default().show(ctx, |ui| {
            crate::ui::render_menu(self, ui);
        });

        ctx.request_repaint_after(std::time::Duration::from_secs(2));
    }
}