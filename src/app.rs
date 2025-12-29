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
    // Current File
    pub current_log_name: String,
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
            current_log_name: String::new(),
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
        
        let settings_path = self.get_settings_path();
        if let Ok(json) = serde_json::to_string_pretty(&self.settings) {
            if let Err(e) = std::fs::write(&settings_path, json) {
                eprintln!("Fehler beim Speichern der Einstellungen nach {}: {}", settings_path, e);
            }
        } else {
            eprintln!("Fehler beim Serialisieren der Einstellungen");
        }
        // Note: Translations are now reloaded immediately when language changes in the UI,
        // not here, to ensure the UI updates right away.
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

        let (found_scans, current_log_name) = crate::log_watcher::check_for_scans(
            self.settings.os_mode, 
            &self.settings.log_dir,
            &mut self.log_file_positions
        );

        // Jetzt wandeln wir den PathBuf in einen String für die UI um
        if let Some(path) = current_log_name {
            self.current_log_name = path.file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| "Unbekannt".to_string());
        } // Speichern für die UI

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

        // --- NEU: Die Statusleiste ---
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
            // Hier nutzen wir RichText für die Größe
            // .size(14.0) ist ein guter Mittelwert. Standard ist meist ~12.0
            ui.label(egui::RichText::new(format!("📄 Journal: {}", self.current_log_name))
                .size(14.0)
                .color(ui.visuals().widgets.active.text_color())); // Optional: etwas hellere Farbe
        
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if self.settings.sound_enabled {
                    let vol = (self.settings.volume * 100.0) as i32;
                    ui.label(egui::RichText::new(format!("🔊 {}%", vol)).size(14.0));
                }
            });
        });
    });

        ctx.request_repaint_after(std::time::Duration::from_secs(2));
    }
}