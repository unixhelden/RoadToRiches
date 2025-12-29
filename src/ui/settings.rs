use eframe::egui;
use rfd::FileDialog;
use crate::app::EliteApp;

/// Rendert die Konfigurationsseite der Anwendung.
pub fn render(app: &mut EliteApp, ui: &mut egui::Ui) {
    let settings_heading = app.translations.get("settings_heading");
    ui.heading(settings_heading);
    ui.add_space(10.0);

    // --- LANGUAGE ---
    ui.group(|ui| {
        let language_label = app.translations.get("language_label").to_string();
        ui.label(language_label);
        ui.horizontal(|ui| {
            use crate::i18n::Language;
            // Get translation strings before mutable borrow
            let german_label = app.translations.get("language_german").to_string();
            let english_label = app.translations.get("language_english").to_string();
            
            if ui.selectable_value(&mut app.settings.language, Language::German, &german_label).changed() {
                // Reload translations immediately when language changes
                app.reload_translations();
                ui.ctx().request_repaint(); // Request immediate UI update
                app.save_settings();
            }
            if ui.selectable_value(&mut app.settings.language, Language::English, &english_label).changed() {
                // Reload translations immediately when language changes
                app.reload_translations();
                ui.ctx().request_repaint(); // Request immediate UI update
                app.save_settings();
            }
        });
    });

    ui.add_space(10.0);

    // --- DESIGN ---
    ui.group(|ui| {
        let appearance_label = app.translations.get("appearance_label");
        ui.label(appearance_label);
        let force_dark = app.translations.get("force_dark_design");
        if ui.checkbox(&mut app.settings.dark_mode, force_dark).changed() {
            app.save_settings();
        }
    });

    ui.add_space(10.0);

    // --- LOG PFADE ---
    ui.group(|ui| {
        let journal_dir = app.translations.get("journal_directory");
        ui.label(journal_dir);
        ui.horizontal(|ui| {
            use crate::models::OsMode;
            ui.selectable_value(&mut app.settings.os_mode, OsMode::Windows, "Windows");
            ui.selectable_value(&mut app.settings.os_mode, OsMode::Linux, "Linux");
            ui.selectable_value(&mut app.settings.os_mode, OsMode::Custom, "Custom");
        });

        // Nur wenn Custom gewählt ist, darf der Nutzer einen Pfad wählen
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

    ui.add_space(10.0);

    // --- AUDIO ---
    ui.vertical(|ui| {
        let audio_settings = app.translations.get("audio_settings");
        ui.heading(audio_settings);
        ui.add_space(8.0); // Etwas Abstand unter der Überschrift

        ui.horizontal(|ui| {
            let play_sound = app.translations.get("play_sound_on_scan");
            if ui.checkbox(&mut app.settings.sound_enabled, play_sound).changed() {
                app.save_settings();
            }
        ui.add_space(20.0); // Raum zwischen CheckBox und Button

        ui.add_enabled_ui(app.settings.sound_enabled, |ui| {
            let test_sound = app.translations.get("test_sound");
            let test_tooltip = app.translations.get("test_sound_tooltip");
            if ui.button(test_sound).on_hover_text(test_tooltip).clicked() {
                let sound_file = if app.settings.sound_file.is_empty() {
                    None
                } else {
                    Some(app.settings.sound_file.as_str())
                };
                crate::audio::play_scan_sound(app.settings.volume, sound_file);
            }
        });
    });

    ui.add_space(12.0);
    
    ui.add_enabled_ui(app.settings.sound_enabled, |ui| {
        ui.horizontal(|ui| {
            let volume_label = app.translations.get("volume_label");
            ui.label(volume_label); 
            // Slider von 0% bis 100% (interne Werte 0.0 bis 1.0)
            let res = ui.add(egui::Slider::new(&mut app.settings.volume, 0.0..=1.0)
                .show_value(true)
                .text(volume_label));

            if res.changed() {
                app.settings.validate_volume();
                app.save_settings();
            }
        });
    });
    
    ui.horizontal(|ui| {
        let choose_sound = app.translations.get("choose_sound_file");
        if ui.button(choose_sound).clicked() {
            if let Some(path) = FileDialog::new().add_filter("Audio", &["wav", "mp3"]).pick_file() {
                app.settings.sound_file = path.display().to_string();
                app.save_settings();
            }
        }
        ui.label(egui::RichText::new(&app.settings.sound_file).small());
        });
    });
}