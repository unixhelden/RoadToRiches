pub mod settings;
pub mod route;

use eframe::egui;
use crate::app::EliteApp;

/// Zeichnet die obere Navigationsleiste der App.
// In src/ui/mod.rs

pub fn render_menu(app: &mut EliteApp, ui: &mut egui::Ui) {
    // Ersetze egui::menu::bar durch egui::menu::nav_bar oder direkt:
    ui.horizontal_top(|ui| {
        ui.selectable_value(&mut app.show_settings, false, "📊 Route");
        ui.selectable_value(&mut app.show_settings, true, "⚙ Einstellungen");
    });
}