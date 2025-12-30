use eframe::egui;
use crate::app::EliteApp;

pub mod route;
pub mod settings;
pub mod modals;
pub mod history;

/// Wendet das Elite-typische Orange-Schwarze Design global auf die App an.
pub fn apply_elite_theme(ctx: &egui::Context, dark_mode: bool) {
    let mut visuals = if dark_mode {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };

    if dark_mode {
        let orange = egui::Color32::from_rgb(255, 120, 0);   // Elite HUD Orange
        let deep_black = egui::Color32::from_rgb(10, 10, 10); // Hintergrund
        let surface_gray = egui::Color32::from_rgb(25, 25, 25); // Karten-Hintergrund

        visuals.panel_fill = deep_black;
        visuals.window_fill = surface_gray;
        visuals.selection.bg_fill = orange;
        
        // Rahmen-Stil für den EDMC-Look
        visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, orange);
        visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, orange);
        visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.5, orange);
        visuals.widgets.active.bg_stroke = egui::Stroke::new(1.5, orange);
        
        visuals.override_text_color = Some(egui::Color32::from_rgb(230, 230, 230));
    }
    ctx.set_visuals(visuals);
}

/// Diese Funktion wurde vom Compiler vermisst.
/// Sie rendert die Tab-Leiste oben und schaltet zwischen den Modulen um.
pub fn render_menu(app: &mut EliteApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        // Tab: Route
        // Nutzt selectable_value, um den Button orange leuchten zu lassen, wenn er aktiv ist
        let route_label = app.translations.get("route_tab");
        if ui.selectable_label(app.current_tab == "route", route_label).clicked() {
            app.current_tab = "route".to_string();
        }
        
        // Tab: Einstellungen
        let settings_label = app.translations.get("settings_tab");
        if ui.selectable_label(app.current_tab == "settings", settings_label).clicked() {
            app.current_tab = "settings".to_string();
        }

        // Tab: Verlauf
        let history_label = app.translations.get("history_tab");
        if ui.selectable_label(app.current_tab == "history", history_label).clicked() {
            app.current_tab = "history".to_string();
        }
    });
    
    ui.add_space(5.0);
    ui.separator(); // Trennlinie zwischen Menü und Inhalt
    ui.add_space(10.0);

    // Je nach ausgewähltem Tab das entsprechende Modul rendern
    match app.current_tab.as_str() {
        "route" => route::render(app, ui),
        "settings" => settings::render(app, ui),
        "history" => history::render(app, ui),
        _ => { 
            let not_found = app.translations.get("tab_not_found");
            ui.label(not_found);
        }
    }
}