use eframe::egui;
use rfd::FileDialog;
use crate::app::EliteApp;
use crate::constants::constants::{SOUND_FSS, SOUND_DSS, COPY_FEEDBACK_DURATION_MS};

pub fn render(app: &mut EliteApp, ui: &mut egui::Ui) {
    ui.add_space(5.0);

    // 1. Index suchen (nach oben verschoben für Label-Anzeige)
    let group_index = app.groups.iter().position(|g| {
        g.bodies.iter().any(|b| !b.is_completed())
    });

    // --- CSV LOAD BUTTON ---
    ui.horizontal(|ui| {
        let load_btn_text = app.translations.get("load_csv");
        if ui.button(load_btn_text).clicked() {
            if let Some(path) = FileDialog::new().add_filter("CSV", &["csv"]).pick_file() {
                let path_str = path.display().to_string();
                match crate::csv_logic::load_and_group(&path_str) {
                    Ok(new_groups) => {
                        app.settings.csv_path = path_str;
                        app.groups = new_groups;
                        app.save_settings();
                    }
                    Err(e) => eprintln!("Fehler beim Laden der CSV: {}", e),
                }
            }
        }
        
        // FIX 3: Zielsystem anzeigen statt Dateiname
        if !app.groups.is_empty() {
            let final_name = &app.groups.last().unwrap().name;
            let total = app.groups.len();
            let current_idx = group_index.unwrap_or(total);
            let percent = if total > 0 { (current_idx as f32 / total as f32) * 100.0 } else { 0.0 };
            
            // Destination Name: Größer (24.0) und in Elite-Orange
            ui.label(egui::RichText::new(format!("🏁 {}", final_name))
                .size(24.0).strong().color(egui::Color32::from_rgb(255, 125, 0)));
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let route_label = app.translations.get("route_progress_label");
                ui.label(egui::RichText::new(format!("{} {:.2}%", route_label, percent).replace('.', ",")).size(14.0).weak());
            });
        } else if !app.settings.csv_path.is_empty() {
            let file_name = std::path::Path::new(&app.settings.csv_path).file_name().unwrap_or_default().to_string_lossy();
            ui.label(egui::RichText::new(file_name).small().weak());
        }
    });
    ui.separator();

    let mut needs_save = false;

    if let Some(idx) = group_index {
        
        // 2. WICHTIG: Daten KOPIEREN
        let (system_name, system_jumps) = {
            let g = &app.groups[idx];
            (g.name.clone(), g.jumps)
        };

        // FIX 2: Header fixiert oben (außerhalb der ScrollArea)
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            
            ui.horizontal(|ui| {
                let header_text = format!("SYSTEM: {} ({} JUMPS)", system_name.to_uppercase(), system_jumps);
                
                let header_btn = ui.selectable_label(false, egui::RichText::new(header_text)
                    .size(16.0).strong().color(egui::Color32::from_rgb(255, 125, 0)));
                
                if header_btn.clicked() {
                    app.copy_to_clipboard(&system_name, ui.ctx());
                }

                if let Some(last_copy) = app.last_copy_time {
                    if last_copy.elapsed().as_millis() < COPY_FEEDBACK_DURATION_MS as u128 {
                        ui.label(egui::RichText::new(" 📋 Copied!").color(egui::Color32::GREEN).italics());
                        ui.ctx().request_repaint();
                    }
                }
            });
        });

        ui.add_space(5.0);

        // FIX 2: Scrollbare Liste der Planeten
        egui::ScrollArea::vertical().show(ui, |ui| {
            // 3. Jetzt holen wir uns die Bodies. 
            let bodies = &mut app.groups[idx].bodies;
            
            // Wir packen die Liste in eine Group für den visuellen Rahmen
            ui.group(|ui| {
                ui.set_min_width(ui.available_width());

                for body in bodies {
                    let is_done = body.is_completed();
                    
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            let elite_orange = egui::Color32::from_rgb(255, 125, 0);

                            // FIX 1: Fortschrittsanzeige (Orange für FSS, Grün für DSS)
                            let (p, c) = if body.dss_mapped { 
                                (1.0, egui::Color32::GREEN) 
                            } else if body.fss_scanned { 
                                (0.5, elite_orange) // Einheitliches Elite-Orange
                            } else { 
                                (0.0, egui::Color32::GRAY) 
                            };

                            ui.add(egui::ProgressBar::new(p).fill(c).desired_width(40.0));

                            // Name einfärben je nach Status
                            let name_color = if body.dss_mapped {
                                egui::Color32::GREEN
                            } else if body.fss_scanned {
                                elite_orange
                            } else {
                                ui.visuals().text_color()
                            };

                            ui.add_sized([160.0, 20.0], egui::Label::new(egui::RichText::new(&body.body_name).size(14.0).strong().color(name_color)));

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.add_sized([85.0, 28.0], egui::Button::new("DONE")).clicked() {
                                    if is_done { 
                                        body.mark_incomplete(); 
                                    } else { 
                                        body.mark_completed(); 
                                        // Hier nutzen wir app.settings.volume direkt über den Pfad
                                        let custom_path = if app.settings.sound_dss_path.is_empty() { None } else { Some(app.settings.sound_dss_path.clone()) };
                                        crate::audio::play_sound(app.settings.volume, custom_path, SOUND_DSS);
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
                                            let custom_path = if app.settings.sound_dss_path.is_empty() { None } else { Some(app.settings.sound_dss_path.clone()) };
                                            crate::audio::play_sound(app.settings.volume, custom_path, SOUND_DSS); 
                                        }
                                        needs_save = true;
                                    }
                                }

                                ui.add_space(8.0);

                                let fss_label = if body.fss_scanned { "✅ FSS" } else { "⚪ FSS" };
                                if ui.button(egui::RichText::new(fss_label).size(12.0)).clicked() {
                                    body.fss_scanned = !body.fss_scanned;
                                    if body.fss_scanned { 
                                        let custom_path = if app.settings.sound_fss_path.is_empty() { None } else { Some(app.settings.sound_fss_path.clone()) };
                                        crate::audio::play_sound(app.settings.volume, custom_path, SOUND_FSS); 
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
        });
    } else {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            if app.groups.is_empty() {
                let msg = app.translations.get("no_route_message");
                ui.label(msg);
            } else {
                let all_done_msg = app.translations.get("all_targets_reached");
                ui.label(egui::RichText::new(all_done_msg).size(20.0).color(egui::Color32::GREEN));
            }
        });
    }

    if needs_save {
        let _ = crate::csv_logic::save_all(&app.settings.csv_path, &app.groups);
    }
}