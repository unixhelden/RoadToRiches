use eframe::egui;
use rfd::FileDialog;
use std::time::{Instant, Duration};
use crate::app::EliteApp;
use crate::models::Body;

/// Haupt-Render-Loop für die Routen-Ansicht.
pub fn render(app: &mut EliteApp, ui: &mut egui::Ui) {
    let is_dark = ui.visuals().dark_mode;
    let orange = egui::Color32::from_rgb(255, 120, 0);
    // Highlight-Farbe: Orange im Dark Mode, sonst System-Blau
    let highlight = if is_dark { orange } else { ui.visuals().selection.bg_fill };

    // --- OBERE NAVIGATION (Dateiauswahl) ---
    ui.horizontal(|ui| {
        if ui.button("📁 CSV laden").clicked() {
            // Öffnet den nativen Datei-Dialog (funktioniert auf Fedora/Linux super)
            if let Some(path) = FileDialog::new().add_filter("CSV", &["csv"]).pick_file() {
                app.settings.csv_path = path.display().to_string();
                // Neue Route laden und nach Systemen gruppieren
                if let Ok(data) = crate::csv_logic::load_and_group(&app.settings.csv_path) {
                    app.groups = data;
                }
                app.save_settings(); // Pfad in settings.json merken
            }
        }
        // Gekürzten Pfad zur Info anzeigen
        ui.label(egui::RichText::new(&app.settings.csv_path).small().weak());
    });
    
    ui.add_space(5.0);
    ui.separator();

    // Finde das erste System, das noch nicht fertig gescannt wurde
    let current_idx = app.groups.iter().position(|g| {
        g.bodies.iter().any(|b| b.status != "erledigt")
    });

    if let Some(idx) = current_idx {
        let mut needs_save = false;
        let system = &mut app.groups[idx];
        
        // --- ZENTRIERTER INHALT (Max 500px) ---
        ui.vertical_centered(|ui| {
            let content_width = 500.0; // Deine Ziel-Breite für das Tool
            ui.set_max_width(content_width);

            // SYSTEM-NAME (Klickbar zum Kopieren)
            let is_recently_copied = app.last_copy_time.map_or(false, |t| t.elapsed() < Duration::from_millis(800));
            let title_color = if is_recently_copied { egui::Color32::GREEN } else { highlight };

            let resp = ui.add(egui::Label::new(
                egui::RichText::new(format!("📍 {}", system.name)).size(24.0).strong().color(title_color)
            ).sense(egui::Sense::click()));

            if resp.clicked() {
                ui.ctx().copy_text(system.name.clone());
                app.last_copy_time = Some(Instant::now());
            }
            ui.label(format!("Sprünge: {}", system.jumps));

            ui.add_space(10.0);

            // Scroll-Liste für die Planeten im System
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        for body in &mut system.bodies {
                            render_body_card(ui, body, &mut needs_save, highlight);
                            ui.add_space(8.0);
                        }
                    });
                });
        });

        // Automatisch speichern, wenn eine Checkbox angeklickt wurde
        if needs_save { 
            let _ = crate::csv_logic::save_all(&app.settings.csv_path, &app.groups); 
        }
    }
}

/// Rendert eine einzelne "Planeten-Karte".
fn render_body_card(ui: &mut egui::Ui, body: &mut Body, needs_save: &mut bool, highlight: egui::Color32) {
    // Frame-Design: Schwarzer Grund, Orange Umrandung (EDMC-Look)
    let frame = egui::Frame::group(ui.style())
        .fill(ui.visuals().panel_fill)
        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
        .inner_margin(10.0)
        .corner_radius(egui::CornerRadius::same(2));

    frame.show(ui, |ui| {
        ui.set_width(480.0); // Etwas schmaler als der Container wegen Scrollbar
        ui.horizontal(|ui| {
            // Checkbox zum Markieren des Fortschritts
            let mut is_done = body.status == "erledigt";
            if ui.checkbox(&mut is_done, "").changed() {
                body.status = if is_done { "erledigt".to_string() } else { "".to_string() };
                *needs_save = true;
            }

            ui.vertical(|ui| {
                ui.label(egui::RichText::new(&body.body_name).strong());
                ui.horizontal(|ui| {
                    // Planeten-Typ in Orange hervorheben
                    ui.label(egui::RichText::new(&body.body_subtype).small().color(highlight));
                    ui.label(egui::RichText::new(format!("| {:.0} Ls", body.distance)).small().weak());
                });
            });

            // Terraformierbar-Tag am rechten Rand
            if body.terraformable == "Yes" {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new("✨ TERRAFORMABLE").color(highlight).small().strong());
                });
            }
        });
    });
}