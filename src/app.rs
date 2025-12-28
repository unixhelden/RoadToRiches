use eframe::egui;
use crate::models::{SystemGroup, AppSettings};
use crate::log_watcher::check_for_scans;
use std::fs::File;
use std::io::BufReader;
use std::time::{Instant, Duration};
use rodio::{Decoder, OutputStream, Sink};
use crate::ui; 

pub struct EliteApp {
    pub groups: Vec<SystemGroup>,
    pub settings: AppSettings,
    pub show_settings: bool,
    pub last_log_check: Instant,
    pub last_copy_time: Option<Instant>,
}

impl EliteApp {
    pub fn new(groups: Vec<SystemGroup>, settings: AppSettings) -> Self {
        Self {
            groups,
            settings,
            show_settings: false,
            last_log_check: Instant::now(),
            last_copy_time: None,
        }
    }

    pub fn save_settings(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.settings) {
            let _ = std::fs::write("settings.json", json);
        }
    }

    pub fn play_scan_sound(&self) {
        if !self.settings.sound_enabled || self.settings.sound_file.is_empty() { return; }
        let sound_path = self.settings.sound_file.clone();
        std::thread::spawn(move || {
            if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
                if let Ok(file) = File::open(sound_path) {
                    let reader = BufReader::new(file);
                    if let Ok(source) = Decoder::new(reader) {
                        if let Ok(sink) = Sink::try_new(&stream_handle) {
                            sink.append(source);
                            sink.sleep_until_end();
                        }
                    }
                }
            }
        });
    }

    fn run_log_check(&mut self) {
        let found_scans = check_for_scans(&self.settings.os_mode, &self.settings.log_dir);
        let mut changed = false;
        let mut play_sound = false;

        for body_name in found_scans {
            for group in &mut self.groups {
                for body in &mut group.bodies {
                    if body.body_name == body_name && body.status != "erledigt" {
                        body.status = "erledigt".to_string();
                        changed = true;
                        play_sound = true;
                    }
                }
            }
        }

        if play_sound { self.play_scan_sound(); }
        if changed { let _ = crate::csv_logic::save_all(&self.settings.csv_path, &self.groups); }
    }
}

impl eframe::App for EliteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Theme-Einstellung anwenden
        if self.settings.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        if self.last_log_check.elapsed() > Duration::from_secs(2) {
            self.run_log_check();
            self.last_log_check = Instant::now();
        }

        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui::render_menu(self, ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.show_settings {
                ui::settings::render(self, ui);
            } else {
                ui::route::render(self, ui);
            }
        });

        if self.last_copy_time.map_or(false, |t| t.elapsed() < Duration::from_millis(800)) {
            ctx.request_repaint();
        }
    }
}