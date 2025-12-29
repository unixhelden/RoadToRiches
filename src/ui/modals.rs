use eframe::egui;
use egui::Context;
use crate::app::EliteApp;

pub fn draw_update_modal(app: &mut EliteApp, ctx: &Context) {
    // Falls ein Update da ist, kopieren wir die Daten kurz raus
    let update_data = if let Some(info) = &app.update_info {
        Some((info.version.clone(), info.body.clone()))
    } else {
        None
    };

    // Jetzt arbeiten wir mit den Kopien (version, body), nicht mehr mit dem Borrow von 'app'
    if let Some((version, body)) = update_data {
        egui::Window::new("🚀 Update verfügbar")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .fixed_size([400.0, 200.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.heading(format!("Version {} ist da!", version));
                    ui.add_space(10.0);
                });

                ui.separator();
                
                egui::ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
                    ui.label(&body);
                });

                ui.add_space(15.0);
                ui.separator();
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    let install_btn = ui.add_sized(
                        [120.0, 30.0], 
                        egui::Button::new("Jetzt Installieren").fill(egui::Color32::from_rgb(200, 100, 0))
                    );

                    if install_btn.clicked() {
                        if let Err(e) = crate::update::execute_update() {
                            eprintln!("Fehler beim Update: {}", e);
                        } else {
                            std::process::exit(0);
                        }
                    }

                    ui.add_space(20.0);

                    if ui.add_sized([100.0, 30.0], egui::Button::new("Vielleicht später")).clicked() {
                        // Da wir oben die Leihgabe beendet haben, dürfen wir 'app' hier jetzt verändern!
                        app.update_info = None;
                    }
                });
            });
    }
}