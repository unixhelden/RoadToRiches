use eframe::egui;
use rfd::FileDialog;
use std::time::{Instant, Duration};
use crate::app::EliteApp;
use crate::models::Body;

/// Haupt-Render-Funktion für die Routen-Ansicht
pub fn render(app: &mut EliteApp, ui: &mut egui::Ui) {
    let is_dark = ui.visuals().dark_mode;
    let orange = egui::Color32::from_rgb(255, 120, 0);
    let highlight = if is_dark { orange } else { ui.visuals().selection.bg_fill };

    // --- NAVIGATION BAR (Obere Zeile) ---
    ui.horizontal(|ui| {
        // Button zum Laden einer neuen Spansh-CSV
        if ui.button("📁 CSV laden").clicked() {
            if let Some(path) = FileDialog::new().add_filter("CSV", &["csv"]).pick_file() {
                app.settings.csv_path = path.display().to_string();
                if let Ok(data) = crate::csv_logic::load_and_group(&app.settings.csv_path) {
                    app.groups = data;
                }
                app.save_settings();
            }
        }

        ui.add_space(15.0);

        // --- VERGRÖSSERTE DESTINATION ANZEIGE ---
        if !app.groups.is_empty() {
            if let Some(last_system) = app.groups.last() {
                ui.label(egui::RichText::new("ZIEL:").small().weak());
                // Hier die deutliche Vergrößerung auf 18.0 und fett
                ui.label(egui::RichText::new(&last_system.name)
                    .size(18.0) 
                    .strong()
                    .color(highlight));
            }
        } else {
            ui.label(egui::RichText::new("Keine Route aktiv").small().italics().weak());
        }
    });
    
    ui.add_space(8.0);
    ui.separator();
    ui.add_space(10.0);

    // Suche das aktuelle System
    let current_idx = app.groups.iter().position(|g| {
        g.bodies.iter().any(|b| b.status != "erledigt")
    });

    if let Some(idx) = current_idx {
        let mut needs_save = false;
        let system = &mut app.groups[idx];
        
        // --- ZENTRIERTER CONTENT-BEREICH (Fixiert auf 500px) ---
        ui.vertical_centered(|ui| {
            let content_width = 500.0;
            ui.set_max_width(content_width);

            // AKTUELLER STANDORT (📍 System Name)
            let is_recently_copied = app.last_copy_time.map_or(false, |t| t.elapsed() < Duration::from_millis(800));
            let title_color = if is_recently_copied { egui::Color32::GREEN } else { highlight };

            let resp = ui.add(egui::Label::new(
                egui::RichText::new(format!("📍 {}", system.name))
                    .size(26.0) // Sogar noch einen Tick größer als das Ziel
                    .strong()
                    .color(title_color)
            ).sense(egui::Sense::click()));

            if resp.clicked() {
                ui.ctx().copy_text(system.name.clone());
                app.last_copy_time = Some(Instant::now());
            }

            ui.label(egui::RichText::new(format!("Sprünge bis Ziel: {}", system.jumps)).weak());

            ui.add_space(10.0);

            // SCROLL-BEREICH
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        for body in &mut system.bodies {
                            render_body_card(ui, body, &mut needs_save, highlight, content_width);
                            ui.add_space(8.0);
                        }
                    });
                });
        });

        if needs_save { 
            let _ = crate::csv_logic::save_all(&app.settings.csv_path, &app.groups); 
        }
    } else {
        render_empty_state(ui);
    }
}

/// Rendert eine einzelne Planeten-Karte im EDMC-Stil
fn render_body_card(ui: &mut egui::Ui, body: &mut Body, needs_save: &mut bool, highlight: egui::Color32, width: f32) {
    let frame = egui::Frame::group(ui.style())
        .fill(ui.visuals().panel_fill)
        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
        .inner_margin(10.0)
        .corner_radius(egui::CornerRadius::same(2));

    frame.show(ui, |ui| {
        ui.set_width(width - 15.0); 

        ui.horizontal(|ui| {
            let mut is_done = body.status == "erledigt";
            if ui.checkbox(&mut is_done, "").changed() {
                body.status = if is_done { "erledigt".to_string() } else { "".to_string() };
                *needs_save = true;
            }

            ui.vertical(|ui| {
                ui.label(egui::RichText::new(&body.body_name).strong().size(15.0));
                
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(&body.body_subtype).small().color(highlight));
                    ui.label(egui::RichText::new("|").weak());
                    ui.label(egui::RichText::new(format!("{:.0} Ls", body.distance)).small().weak());
                });
            });

            if body.terraformable == "Yes" {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new("✨ TERRAFORMABLE")
                        .color(highlight)
                        .small()
                        .strong());
                });
            }
        });
    });
}

fn render_empty_state(ui: &mut egui::Ui) {
    ui.centered_and_justified(|ui| {
        ui.label(egui::RichText::new("Keine Route aktiv.\nBitte lade eine Spansh-CSV.").weak());
    });
}