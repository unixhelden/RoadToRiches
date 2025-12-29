use serde::{Deserialize, Serialize};

/// Operating system mode for log file detection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OsMode {
    Windows,
    Linux,
    Custom,
}

impl Default for OsMode {
    fn default() -> Self {
        if cfg!(windows) {
            OsMode::Windows
        } else {
            OsMode::Linux
        }
    }
}

impl OsMode {
    /// Convert to string for backward compatibility
    pub fn to_str(&self) -> &'static str {
        match self {
            OsMode::Windows => "Windows",
            OsMode::Linux => "Linux",
            OsMode::Custom => "Custom",
        }
    }
    
    /// Convert from string (for backward compatibility)
    pub fn from_str(s: &str) -> Self {
        match s {
            "Windows" => OsMode::Windows,
            "Linux" => OsMode::Linux,
            "Custom" => OsMode::Custom,
            _ => Self::default(),
        }
    }
}

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
    /// Volume Setting (0.0 to 1.0)
    pub volume: f32,
    /// Betriebsmodus für die Log-Suche
    #[serde(default, serialize_with = "serialize_os_mode", deserialize_with = "deserialize_os_mode")]
    pub os_mode: OsMode,
    /// Flag für das dunkle Design (wird in der App-Schleife erzwungen).
    pub dark_mode: bool,
    /// Selected language for the UI
    #[serde(default, serialize_with = "serialize_language", deserialize_with = "deserialize_language")]
    pub language: crate::i18n::Language,
}

// Custom serialization for backward compatibility
fn serialize_os_mode<S>(mode: &OsMode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(mode.to_str())
}

fn deserialize_os_mode<'de, D>(deserializer: D) -> Result<OsMode, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(OsMode::from_str(&s))
}

fn serialize_language<S>(lang: &crate::i18n::Language, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(lang.to_str())
}

fn deserialize_language<'de, D>(deserializer: D) -> Result<crate::i18n::Language, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(crate::i18n::Language::from_str(&s))
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            csv_path: String::new(),
            log_dir: String::new(),
            sound_enabled: true,
            sound_file: String::new(),
            volume: 0.5,
            os_mode: OsMode::default(),
            dark_mode: true,
            language: crate::i18n::Language::default(),
        }
    }
}

impl AppSettings {
    /// Validates and clamps volume to valid range [0.0, 1.0]
    pub fn validate_volume(&mut self) {
        self.volume = self.volume.clamp(0.0, 1.0);
    }
    
    /// Validates settings and returns any errors found
    pub fn validate(&mut self) -> Vec<String> {
        let mut errors = Vec::new();
        
        // Validate volume
        self.validate_volume();
        
        // Validate CSV path if set
        if !self.csv_path.is_empty() && !std::path::Path::new(&self.csv_path).exists() {
            errors.push(format!("CSV-Datei nicht gefunden: {}", self.csv_path));
        }
        
        // Validate sound file if set
        if !self.sound_file.is_empty() && !std::path::Path::new(&self.sound_file).exists() {
            errors.push(format!("Sound-Datei nicht gefunden: {}", self.sound_file));
        }
        
        errors
    }
}