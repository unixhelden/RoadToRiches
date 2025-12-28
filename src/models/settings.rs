use serde::{Deserialize, Serialize};

/// Speichert alle Benutzereinstellungen der App.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    /// Pfad zur geladenen Road-to-Riches CSV.
    pub csv_path: String,
    /// Manueller Pfad, falls 'Custom' Modus gewählt wurde.
    pub log_dir: String,
    /// Ob ein Sound bei einem erfolgreichen Scan abgespielt wird.
    pub sound_enabled: bool,
    /// Pfad zur gewählten Sounddatei (.wav / .mp3).
    pub sound_file: String,
    /// Modus für die Log-Suche: "Windows", "Linux" oder "Custom".
    pub os_mode: String,
    /// Erzwingt das dunkle Design (True) oder nutzt das System-Theme (False).
    pub dark_mode: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            csv_path: String::new(),
            log_dir: String::new(),
            sound_enabled: true,
            sound_file: String::new(),
            // Erkennt das OS beim ersten Start automatisch
            os_mode: if cfg!(windows) { "Windows".into() } else { "Linux".into() },
            dark_mode: true,
        }
    }
}