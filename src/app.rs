use eframe::egui;
use crate::models::{AppSettings, SystemGroup};
use std::time::Instant;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use crate::update::UpdateInfo;
use crate::log_watcher::ScanEvent;
use crate::constants::constants::{SOUND_FSS, SOUND_DSS};

pub struct EliteApp {
    pub settings: AppSettings,
    pub groups: Vec<SystemGroup>,
    pub current_tab: String,
    pub last_copy_time: Option<Instant>,
    pub last_log_check: Instant,
    pub log_file_positions: HashMap<String, u64>,
    pub translations: crate::i18n::Translations,
    
    // --- Update fields ---
    pub update_receiver: Option<Receiver<Option<UpdateInfo>>>,
    pub update_info: Option<UpdateInfo>,
    pub current_log_name: String,
}

impl EliteApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, groups: Vec<SystemGroup>, settings: AppSettings) -> Self {
        let translations = crate::i18n::Translations::load(settings.language);
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
            current_log_name: String::from("Searching Logs..."),
        }
    }

    /// Copies text to clipboard and updates the visual feedback timer
    pub fn copy_to_clipboard(&mut self, text: &str, ctx: &egui::Context) {
        ctx.copy_text(text.to_string()); 
        self.last_copy_time = Some(Instant::now());
    }

    pub fn reload_translations(&mut self) {
        self.translations = crate::i18n::Translations::load(self.settings.language);
    }

    pub fn save_settings(&mut self) {
        self.settings.validate_volume();
        let settings_path = self.get_settings_path();
        if let Ok(json) = serde_json::to_string_pretty(&self.settings) {
            if let Err(e) = std::fs::write(&settings_path, json) {
                eprintln!("Error saving settings: {}", e);
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

    /// Monitors game logs and updates scan status in real-time
    fn process_log_updates(&mut self) {
        use crate::constants::constants::LOG_CHECK_INTERVAL_SECS;
        
        if self.last_log_check.elapsed().as_secs() < LOG_CHECK_INTERVAL_SECS {
            return;
        }
        self.last_log_check = Instant::now();

        let (found_events, log_path) = crate::log_watcher::check_for_scans(
            self.settings.os_mode, 
            &self.settings.log_dir,
            &mut self.log_file_positions
        );

        if let Some(path) = log_path {
            self.current_log_name = path.file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| "Unknown".to_string());
        }

        if !found_events.is_empty() {
            let mut changed = false;
            // BORROW CHECKER FIX: Store sounds to play after the mutable loop
            let mut sounds_to_play: Vec<bool> = Vec::new(); 

            for event in found_events {
                let (target_name, is_dss_event) = match event {
                    ScanEvent::FSS(name) => (name, false),
                    ScanEvent::DSS(name) => (name, true),
                };

                for group in &mut self.groups {
                    for body in &mut group.bodies {
                        if body.body_name == target_name {
                            if is_dss_event {
                                if !body.dss_mapped {
                                    body.mark_dss_done();
                                    changed = true;
                                    sounds_to_play.push(true); // Queue DSS sound
                                }
                            } else {
                                if !body.fss_scanned {
                                    body.mark_fss_done();
                                    changed = true;
                                    sounds_to_play.push(false); // Queue FSS sound
                                }
                            }
                        }
                    }
                }
            }

            // Play sounds after the mutable borrow of self.groups is over
            for is_dss in sounds_to_play {
                self.play_feedback_sound(is_dss);
            }

            if changed {
                let _ = crate::csv_logic::save_all(&self.settings.csv_path, &self.groups);
            }
        }
    }

    /// Plays the embedded audio bytes (no external files needed)
    pub fn play_feedback_sound(&self, is_dss: bool) {
        if self.settings.sound_enabled {
            let (data, custom_path) = if is_dss {
                (SOUND_DSS, if self.settings.sound_dss_path.is_empty() { None } else { Some(self.settings.sound_dss_path.clone()) })
            } else {
                (SOUND_FSS, if self.settings.sound_fss_path.is_empty() { None } else { Some(self.settings.sound_fss_path.clone()) })
            };
            crate::audio::play_sound(self.settings.volume, custom_path, data);
        }
    }
}

impl eframe::App for EliteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.process_log_updates();

        if let Some(rx) = &self.update_receiver {
            if let Ok(result) = rx.try_recv() {
                self.update_info = result;
                self.update_receiver = None;
            }
        }

        crate::ui::apply_elite_theme(ctx, self.settings.dark_mode);

        egui::CentralPanel::default().show(ctx, |ui| {
            crate::ui::render_menu(self, ui);
        });

        crate::ui::modals::draw_update_modal(self, ctx);

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

        // Fast repaint to ensure "Copied!" feedback and logs feel snappy
        ctx.request_repaint_after(std::time::Duration::from_millis(200));
    }
}