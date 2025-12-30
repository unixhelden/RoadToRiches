use eframe::egui;
use crate::app::EliteApp;
use crate::constants::constants::{SOUND_FSS, SOUND_DSS, COPY_FEEDBACK_DURATION_MS};

pub fn render(app: &mut EliteApp, ui: &mut egui::Ui) {
    ui.add_space(5.0);
    let mut needs_save = false;

    // 1. Index suchen (wie gehabt)
    let group_index = app.groups.iter().position(|g| {
        g.bodies.iter().any(|b| !b.is_completed())
    });

    egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
        if let Some(idx) = group_index {
            
            // 2. WICHTIG: Daten KOPIEREN, bevor wir in die UI-Closures gehen
            // So blockiert 'group' nicht das 'app' Objekt.
            let (system_name, system_jumps) = {
                let g = &app.groups[idx];
                (g.name.clone(), g.jumps)
            };

            ui.group(|ui| {
                ui.set_min_width(ui.available_width());
                
                ui.horizontal(|ui| {
                    // Wir nutzen hier nur die kopierten Variablen: system_name, system_jumps
                    let header_text = format!("SYSTEM: {} ({} JUMPS)", system_name.to_uppercase(), system_jumps);
                    
                    let header_btn = ui.selectable_label(false, egui::RichText::new(header_text)
                        .size(16.0).strong().color(egui::Color32::from_rgb(255, 125, 0)));
                    
                    if header_btn.clicked() {
                        // app ist hier jetzt frei verfügbar!
                        app.copy_to_clipboard(&system_name, ui.ctx());
                    }

                    if let Some(last_copy) = app.last_copy_time {
                        if last_copy.elapsed().as_millis() < COPY_FEEDBACK_DURATION_MS as u128 {
                            ui.label(egui::RichText::new(" 📋 Copied!").color(egui::Color32::GREEN).italics());
                            ui.ctx().request_repaint();
                        }
                    }
                });
                ui.separator();

                // 3. Jetzt holen wir uns die Bodies. 
                // Da wir 'system_name' oben schon fertig benutzt haben, 
                // können wir jetzt 'app.groups' wieder mutable ausleihen.
                let bodies = &mut app.groups[idx].bodies;
                
                for body in bodies {
                    let is_done = body.is_completed();
                    
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            let (p, c) = if is_done { (1.0, egui::Color32::GREEN) } else { (0.1, egui::Color32::GRAY) };
                            ui.add(egui::ProgressBar::new(p).fill(c).desired_width(40.0));

                            ui.add_sized([160.0, 20.0], egui::Label::new(egui::RichText::new(&body.body_name).size(14.0).strong()));

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.add_sized([85.0, 28.0], egui::Button::new("DONE")).clicked() {
                                    if is_done { 
                                        body.mark_incomplete(); 
                                    } else { 
                                        body.mark_completed(); 
                                        // Hier nutzen wir app.settings.volume direkt über den Pfad
                                        crate::audio::play_sound(app.settings.volume, SOUND_DSS);
                                    }
                                    needs_save = true;
                                }

                                ui.add_space(8.0);
                                ui.label(egui::RichText::new(format!("{:.0} ls", body.distance)).size(12.0).weak());
                                ui.add_space(15.0);

                                if !body.is_star() {
                                    let dss_label = if body.dss_mapped { "✅ DSS" } else { "⚪ DSS" };
                                    if ui.button(egui::RichText::new(dss_label).size(12.0)).clicked() {
                                        body.dss_mapped = !body.dss_mapped;
                                        if body.dss_mapped { 
                                            crate::audio::play_sound(app.settings.volume, SOUND_DSS); 
                                        }
                                        needs_save = true;
                                    }
                                }

                                ui.add_space(8.0);

                                let fss_label = if body.fss_scanned { "✅ FSS" } else { "⚪ FSS" };
                                if ui.button(egui::RichText::new(fss_label).size(12.0)).clicked() {
                                    body.fss_scanned = !body.fss_scanned;
                                    if body.fss_scanned { 
                                        crate::audio::play_sound(app.settings.volume, SOUND_FSS); 
                                    }
                                    needs_save = true;
                                }
                            });
                        });

                        ui.horizontal(|ui| {
                            ui.add_space(55.0); 
                            ui.label(egui::RichText::new(&body.body_subtype)
                                .size(12.0).weak().italics());
                        });

                        ui.add_space(4.0);
                        ui.separator();
                        ui.add_space(4.0);
                    });
                }
            });
        } else {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.label(egui::RichText::new("🎉 Alle Ziele erreicht!").size(20.0).color(egui::Color32::GREEN));
            });
        }
    });

    if needs_save {
        let _ = crate::csv_logic::save_all(&app.settings.csv_path, &app.groups);
    }
}