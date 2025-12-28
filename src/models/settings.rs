use serde::{Deserialize, Serialize};

/// Speichert die Benutzereinstellungen und Pfade.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    /// Pfad zur aktuell geladenen Road-to-Riches CSV-Datei.
    pub csv_path: String,
    /// Manueller Pfad zu den Elite-Journal-Logs (wird bei "Custom" genutzt).
    pub log_dir: String,
    /// Schalter für die akustische Benachrichtigung bei Scans.
    pub sound_enabled: bool,
    /// Pfad zur Audio-Datei (.wav oder .mp3).
    pub sound_file: String,
    /// Betriebsmodus für die Log-Suche ("Windows", "Linux" oder "Custom").
    pub os_mode: String,
    /// Flag für das dunkle Design (wird in der App-Schleife erzwungen).
    pub dark_mode: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            csv_path: String::new(),
            log_dir: String::new(),
            sound_enabled: true,
            sound_file: String::new(),
            // Setzt den Standardwert basierend auf dem Betriebssystem beim ersten Start.
            os_mode: if cfg!(windows) { "Windows".into() } else { "Linux".into() },
            dark_mode: true,
        }
    }
}