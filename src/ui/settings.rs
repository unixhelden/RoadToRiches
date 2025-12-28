use eframe::egui;
use rfd::FileDialog;
use crate::app::EliteApp;

/// Rendert die Konfigurationsseite der Anwendung.
pub fn render(app: &mut EliteApp, ui: &mut egui::Ui) {
    ui.heading("⚙ Einstellungen");
    ui.add_space(10.0);

    // --- DESIGN ---
    ui.group(|ui| {
        ui.label("Erscheinungsbild:");
        if ui.checkbox(&mut app.settings.dark_mode, "Dunkles Design erzwingen").changed() {
            app.save_settings();
        }
    });

    ui.add_space(10.0);

    // --- LOG PFADE ---
    ui.group(|ui| {
        ui.label("Elite Dangerous Journal-Verzeichnis:");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut app.settings.os_mode, "Windows".into(), "Windows");
            ui.selectable_value(&mut app.settings.os_mode, "Linux".into(), "Linux");
            ui.selectable_value(&mut app.settings.os_mode, "Custom".into(), "Custom");
        });

        // Nur wenn Custom gewählt ist, darf der Nutzer einen Pfad wählen
        if app.settings.os_mode == "Custom" {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                if ui.button("📁 Ordner wählen").clicked() {
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
        ui.heading("Audio-Einstellungen");
        ui.add_space(8.0); // Etwas Abstand unter der Überschrift

        ui.horizontal(|ui| {
            if ui.checkbox(&mut app.settings.sound_enabled, "Sound bei Scan abspielen").changed() {
                app.save_settings();
            }
        ui.add_space(20.0); // Raum zwischen CheckBox und Button

        ui.add_enabled_ui(app.settings.sound_enabled, |ui| {
            if ui.button("🔊 Test-Ton").on_hover_text("Spielt den Scan-Sound einmal ab").clicked() {
                // MIt Lautstärke übergeben an Funktion
            crate::audio::play_scan_sound(app.settings.volume);
            }
        });
    });

    ui.add_space(12.0);
    
    ui.add_enabled_ui(app.settings.sound_enabled, |ui| {
        ui.horizontal(|ui| {
            ui.label("Lautstärke:"); 
            // Slider von 0% bis 100% (interne Werte 0.0 bis 1.0)
            let res = ui.add(egui::Slider::new(&mut app.settings.volume, 0.0..=1.0).show_value(true));

            if res.changed() {
                app.save_settings();
            }
        });
    });
    
    ui.horizontal(|ui| {
        if ui.button("🎵 Sound-Datei wählen").clicked() {
            if let Some(path) = FileDialog::new().add_filter("Audio", &["wav", "mp3"]).pick_file() {
                app.settings.sound_file = path.display().to_string();
                app.save_settings();
            }
        }
        ui.label(egui::RichText::new(&app.settings.sound_file).small());
        });
    });
}