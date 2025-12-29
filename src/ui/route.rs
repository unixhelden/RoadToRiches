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
        let load_csv = app.translations.get("load_csv");
        if ui.button(load_csv).clicked() {
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
                let target_label = app.translations.get("target_label");
                ui.label(egui::RichText::new(target_label).small().weak());
                // Hier die deutliche Vergrößerung auf 18.0 und fett
                ui.label(egui::RichText::new(&last_system.name)
                    .size(18.0) 
                    .strong()
                    .color(highlight));
            }
        } else {
            let no_route = app.translations.get("no_route_active");
            ui.label(egui::RichText::new(no_route).small().italics().weak());
        }
    });
    
    ui.add_space(8.0);
    ui.separator();
    ui.add_space(10.0);

    // Suche das aktuelle System
    let current_idx = app.groups.iter().position(|g| {
        g.bodies.iter().any(|b| !b.is_completed())
    });

    if let Some(idx) = current_idx {
        let mut needs_save = false;
        
        // Extract values we need before borrowing system
        use crate::constants::constants::COPY_FEEDBACK_DURATION_MS;
        let is_recently_copied = app.last_copy_time.map_or(false, |t| t.elapsed() < Duration::from_millis(COPY_FEEDBACK_DURATION_MS));
        let title_color = if is_recently_copied { egui::Color32::GREEN } else { highlight };
        let jumps_label = app.translations.get("jumps_to_target").to_string();
        let terraformable_label = app.translations.get("terraformable").to_string();
        
        // --- ZENTRIERTER CONTENT-BEREICH (Fixiert auf 500px) ---
        ui.vertical_centered(|ui| {
            use crate::constants::constants::ROUTE_CONTENT_WIDTH;
            let content_width = ROUTE_CONTENT_WIDTH;
            ui.set_max_width(content_width);

            // AKTUELLER STANDORT (📍 System Name)
            let system_name = app.groups[idx].name.clone();
            let system_jumps = app.groups[idx].jumps;

            let resp = ui.add(egui::Label::new(
                egui::RichText::new(format!("📍 {}", system_name))
                    .size(26.0) // Sogar noch einen Tick größer als das Ziel
                    .strong()
                    .color(title_color)
            ).sense(egui::Sense::click()));

            if resp.clicked() {
                ui.ctx().copy_text(system_name.clone());
                app.last_copy_time = Some(Instant::now());
            }

            ui.label(egui::RichText::new(format!("{} {}", jumps_label, system_jumps)).weak());

            ui.add_space(10.0);

            // SCROLL-BEREICH
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        for body in &mut app.groups[idx].bodies {
                            render_body_card(ui, body, &mut needs_save, highlight, content_width, &terraformable_label);
                            ui.add_space(8.0);
                        }
                    });
                });
        });

        if needs_save { 
            let _ = crate::csv_logic::save_all(&app.settings.csv_path, &app.groups); 
        }
    } else {
        render_empty_state(app, ui);
    }
}

/// Rendert eine einzelne Planeten-Karte im EDMC-Stil
fn render_body_card(ui: &mut egui::Ui, body: &mut Body, needs_save: &mut bool, highlight: egui::Color32, width: f32, terraformable_label: &str) {
    let frame = egui::Frame::group(ui.style())
        .fill(ui.visuals().panel_fill)
        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
        .inner_margin(10.0)
        .corner_radius(egui::CornerRadius::same(2));

    frame.show(ui, |ui| {
        ui.set_width(width - 15.0); 

        ui.horizontal(|ui| {
            let mut is_done = body.is_completed();
            if ui.checkbox(&mut is_done, "").changed() {
                if is_done {
                    body.mark_completed();
                } else {
                    body.mark_incomplete();
                }
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
                    ui.label(egui::RichText::new(terraformable_label)
                        .color(highlight)
                        .small()
                        .strong());
                });
            }
        });
    });
}

fn render_empty_state(app: &EliteApp, ui: &mut egui::Ui) {
    ui.centered_and_justified(|ui| {
        let message = app.translations.get("no_route_message");
        ui.label(egui::RichText::new(message).weak());
    });
}