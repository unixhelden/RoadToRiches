use eframe::egui;
use rfd::FileDialog;
use std::time::{Instant, Duration};
use crate::app::EliteApp;

/// Rendert die Routen-Ansicht und die CSV-Auswahl.
pub fn render(app: &mut EliteApp, ui: &mut egui::Ui) {
    // CSV-Wahl direkt oben auf der Hauptseite
    ui.horizontal(|ui| {
        if ui.button("📁 Neue Route laden").clicked() {
            if let Some(path) = FileDialog::new().add_filter("CSV", &["csv"]).pick_file() {
                app.settings.csv_path = path.display().to_string();
                if let Ok(data) = crate::csv_logic::load_and_group(&app.settings.csv_path) {
                    app.groups = data;
                }
                app.save_settings();
            }
        }
        if !app.settings.csv_path.is_empty() {
            ui.label(egui::RichText::new(&app.settings.csv_path).small().weak());
        }
    });
    ui.separator();

    // Wir suchen das erste System mit noch offenen Zielen
    let current_idx = app.groups.iter().position(|g| {
        g.bodies.iter().any(|b| b.status != "erledigt")
    });

    if let Some(idx) = current_idx {
        let mut needs_save = false;
        let system = &mut app.groups[idx];
        
        ui.vertical_centered(|ui| {
            // Kopier-Animation
            let is_recently_copied = app.last_copy_time.map_or(false, |t| t.elapsed() < Duration::from_millis(800));
            let color = if is_recently_copied { egui::Color32::GREEN } else { egui::Color32::WHITE };

            let resp = ui.add(egui::Label::new(
                egui::RichText::new(format!("📍 {}", system.name)).size(30.0).strong().color(color)
            ).sense(egui::Sense::click()));

            if resp.clicked() {
                ui.ctx().copy_text(system.name.clone());
                app.last_copy_time = Some(Instant::now());
            }
            ui.label(format!("Sprünge: {}", system.jumps));
        });

        ui.add_space(10.0);

        egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
            for body in &mut system.bodies {
                let mut is_done = body.status == "erledigt";
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        if ui.checkbox(&mut is_done, "").changed() {
                            body.status = if is_done { "erledigt".to_string() } else { "".to_string() };
                            needs_save = true;
                        }
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new(&body.body_name).strong());
                            ui.label(format!("{} | {} Ls", body.body_subtype, body.distance));
                        });
                        if body.terraformable == "Yes" {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.colored_label(egui::Color32::from_rgb(255, 165, 0), "✨ TERRAFORMABLE");
                            });
                        }
                    });
                });
            }
        });

        if needs_save { 
            let _ = crate::csv_logic::save_all(&app.settings.csv_path, &app.groups); 
        }
    } else {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            ui.heading("Bereit für den Trip? 🚀");
            ui.label("Lade eine CSV-Datei von Spansh, um zu starten.");
        });
    }
}