use eframe::egui;
use rfd::FileDialog;
use crate::app::EliteApp;
use crate::constants::constants::{SOUND_FSS, SOUND_DSS};

/// Rendert die Konfigurationsseite der Anwendung.
pub fn render(app: &mut EliteApp, ui: &mut egui::Ui) {
    let settings_heading = app.translations.get("settings_heading");
    ui.heading(settings_heading);
    ui.add_space(10.0);

    // --- LANGUAGE ---
    ui.group(|ui| {
        let language_label = app.translations.get("language_label");
        ui.label(egui::RichText::new(language_label).strong());
        ui.horizontal(|ui| {
            use crate::i18n::Language;
            let german_label = app.translations.get("language_german").to_string();
            let english_label = app.translations.get("language_english").to_string();
            
            if ui.selectable_value(&mut app.settings.language, Language::German, german_label).changed() {
                app.reload_translations();
                app.save_settings();
            }
            if ui.selectable_value(&mut app.settings.language, Language::English, english_label).changed() {
                app.reload_translations();
                app.save_settings();
            }
        });
    });

    ui.add_space(10.0);

    // --- DESIGN ---
    ui.group(|ui| {
        let appearance_label = app.translations.get("appearance_label");
        ui.label(egui::RichText::new(appearance_label).strong());
        let force_dark = app.translations.get("force_dark_design");
        if ui.checkbox(&mut app.settings.dark_mode, force_dark).changed() {
            app.save_settings();
        }
    });

    ui.add_space(10.0);

    // --- AUDIO ---
    ui.group(|ui| {
        let audio_settings = app.translations.get("audio_settings");
        ui.label(egui::RichText::new(audio_settings).strong());
        ui.add_space(5.0);

        let play_sound = app.translations.get("play_sound_on_scan");
        if ui.checkbox(&mut app.settings.sound_enabled, play_sound).changed() {
            app.save_settings();
        }

        ui.add_enabled_ui(app.settings.sound_enabled, |ui| {
            ui.add_space(5.0);
            
            // Lautstärke
            ui.horizontal(|ui| {
                let volume_label = app.translations.get("volume_label");
                ui.label(volume_label); 
                let res = ui.add(egui::Slider::new(&mut app.settings.volume, 0.0..=1.0).show_value(true));
                if res.changed() {
                    app.settings.validate_volume();
                    app.save_settings();
                }
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // "Eigene Sound-Dateien (WAV/MP3):"
            ui.label(app.translations.get("custom_sounds_hint"));
            ui.add_space(5.0);

            // --- FSS ROW ---
            ui.horizontal(|ui| {
                let fss_label = app.translations.get("fss_label");
                ui.add_sized([100.0, 20.0], egui::Label::new(fss_label));
    
                let choose_file_tooltip = app.translations.get("choose_file_tooltip");
                if ui.button("📁").on_hover_text(choose_file_tooltip).clicked() {
                    if let Some(path) = FileDialog::new().add_filter("Audio", &["wav", "mp3"]).pick_file() {
                        app.settings.sound_fss_path = path.display().to_string();
                        app.save_settings();
                    }
                }
            
                let test_tooltip = app.translations.get("test_sound_tooltip");
                if ui.button("▶").on_hover_text(test_tooltip).clicked() {
                    // KORREKTUR: 3 Parameter übergeben
                    crate::audio::play_sound(
                        app.settings.volume, 
                        Some(app.settings.sound_fss_path.clone()), 
                        SOUND_FSS
                    );
                }

                if !app.settings.sound_fss_path.is_empty() {
                    if ui.button("🗑").clicked() { 
                        app.settings.sound_fss_path.clear(); 
                        app.save_settings(); 
                    }
                    ui.label(egui::RichText::new(&app.settings.sound_fss_path).small().weak());
                } else {
                    let default_active = app.translations.get("default_sound_label");
                    ui.label(egui::RichText::new(default_active).small().italics().color(egui::Color32::GRAY));
                }
            });

            ui.add_space(4.0);

            // --- DSS ROW ---
            ui.horizontal(|ui| {
                let dss_label = app.translations.get("dss_label");
                ui.add_sized([100.0, 20.0], egui::Label::new(dss_label));
    
                let choose_file_tooltip = app.translations.get("choose_file_tooltip");
                if ui.button("📁").on_hover_text(choose_file_tooltip).clicked() {
                    if let Some(path) = FileDialog::new().add_filter("Audio", &["wav", "mp3"]).pick_file() {
                        app.settings.sound_dss_path = path.display().to_string();
                        app.save_settings();
                    }
                }

                let test_tooltip = app.translations.get("test_sound_tooltip");
                if ui.button("▶").on_hover_text(test_tooltip).clicked() {
                    // KORREKTUR: 3 Parameter übergeben
                    crate::audio::play_sound(
                        app.settings.volume, 
                        Some(app.settings.sound_dss_path.clone()), 
                        SOUND_DSS
                    );
                }

                if !app.settings.sound_dss_path.is_empty() {
                    if ui.button("🗑").clicked() { 
                        app.settings.sound_dss_path.clear(); 
                        app.save_settings(); 
                    }
                    ui.label(egui::RichText::new(&app.settings.sound_dss_path).small().weak());
                } else {
                    let default_active = app.translations.get("default_sound_label");
                    ui.label(egui::RichText::new(default_active).small().italics().color(egui::Color32::GRAY));
                }
            });
        });
    });

    ui.add_space(10.0);

    // --- LOG PATHS ---
    ui.group(|ui| {
        let journal_dir = app.translations.get("journal_directory");
        ui.label(egui::RichText::new(journal_dir).strong());
        ui.horizontal(|ui| {
            use crate::models::OsMode;
            ui.selectable_value(&mut app.settings.os_mode, OsMode::Windows, "Windows");
            ui.selectable_value(&mut app.settings.os_mode, OsMode::Linux, "Linux");
            ui.selectable_value(&mut app.settings.os_mode, OsMode::Custom, "Custom");
        });

        use crate::models::OsMode;
        if app.settings.os_mode == OsMode::Custom {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                let choose_folder = app.translations.get("choose_folder");
                if ui.button(choose_folder).clicked() {
                    if let Some(path) = FileDialog::new().pick_folder() {
                        app.settings.log_dir = path.display().to_string();
                        app.save_settings();
                    }
                }
                ui.label(egui::RichText::new(&app.settings.log_dir).small());
            });
        }
    });
}