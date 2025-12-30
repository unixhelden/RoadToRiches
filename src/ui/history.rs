use eframe::egui;
use crate::app::EliteApp;

pub fn render(app: &mut EliteApp, ui: &mut egui::Ui) {
    let history_label = app.translations.get("history_label");
    ui.heading(history_label);
    ui.add_space(10.0);

    // Finde heraus, wie viele Systeme bereits vollständig sind
    let first_incomplete = app.groups.iter().position(|g| {
        g.bodies.iter().any(|b| !b.is_completed())
    });
    
    let completed_count = first_incomplete.unwrap_or(app.groups.len());

    if completed_count == 0 {
        let msg = app.translations.get("no_history_yet");
        ui.label(egui::RichText::new(msg).italics().weak());
        return;
    }

    egui::ScrollArea::vertical().show(ui, |ui| {
        for i in 0..completed_count {
            let group = &app.groups[i];
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("✅").color(egui::Color32::GREEN));
                    ui.label(egui::RichText::new(&group.name).strong());
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let jumps_label = app.translations.get("jumps_label");
                        ui.label(egui::RichText::new(format!("{} {}", group.jumps, jumps_label)).weak());
                        
                        // Schätzung der Credits anzeigen
                        let value = calculate_system_value(&group.bodies);
                        let val_str = format_credits(value);
                        ui.label(egui::RichText::new(format!("💰 {}", val_str))
                            .color(egui::Color32::GOLD)
                            .strong());
                        ui.add_space(10.0);
                    });
                });
            });
        }
    });
}

/// Berechnet den geschätzten Wert eines Systems basierend auf den Scans.
/// Werte sind Annäherungen für "Road to Riches" (d.h. oft terraformierbar).
fn calculate_system_value(bodies: &[crate::models::Body]) -> u64 {
    let mut total = 0;
    for body in bodies {
        let base_value = match body.body_subtype.to_lowercase().as_str() {
            s if s.contains("earth") => 3_200_000,
            s if s.contains("ammonia") => 1_700_000,
            s if s.contains("water") => 2_000_000, // Annahme: Terraformable
            s if s.contains("high metal") => 1_500_000, // Annahme: Terraformable
            s if s.contains("rocky") => 1_000_000, // Annahme: Terraformable
            _ => 500_000,
        };

        // DSS bringt den vollen Wert (inkl. Effizienz-Bonus Annahme), FSS nur ca. 1/3
        if body.dss_mapped {
            total += base_value;
        } else if body.fss_scanned {
            total += base_value / 3;
        }
    }
    total
}

/// Formatiert große Zahlen lesbar (k, M).
fn format_credits(amount: u64) -> String {
    if amount >= 1_000_000 {
        format!("{:.1} M", amount as f64 / 1_000_000.0)
    } else if amount >= 1_000 {
        format!("{:.0} k", amount as f64 / 1_000.0)
    } else {
        format!("{}", amount)
    }
}