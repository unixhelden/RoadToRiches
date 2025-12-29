use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};

/// Supported languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    German,
    English,
}

impl Default for Language {
    fn default() -> Self {
        Language::German
    }
}

impl Language {
    pub fn to_str(&self) -> &'static str {
        match self {
            Language::German => "german",
            Language::English => "english",
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        match s {
            "english" | "English" => Language::English,
            "german" | "German" | "Deutsch" => Language::German,
            _ => Language::default(),
        }
    }
    
    pub fn filename(&self) -> &'static str {
        match self {
            Language::German => "german.json",
            Language::English => "english.json",
        }
    }
}

/// Translation manager that loads and provides translations
pub struct Translations {
    translations: HashMap<String, String>,
}

impl Translations {
    /// Load translations for the specified language
    pub fn load(language: Language) -> Self {
        let filename = format!("assets/{}", language.filename());
        let translations = if let Ok(content) = fs::read_to_string(&filename) {
            serde_json::from_str::<HashMap<String, String>>(&content)
                .unwrap_or_else(|e| {
                    eprintln!("Fehler beim Laden der Übersetzungen aus {}: {}", filename, e);
                    HashMap::new()
                })
        } else {
            eprintln!("Übersetzungsdatei {} nicht gefunden, verwende leere Übersetzungen", filename);
            HashMap::new()
        };
        
        Self { translations }
    }
    
    /// Get a translated string by key, or return empty string if not found
    /// Note: In practice, all keys should be in the HashMap loaded from JSON files.
    pub fn get(&self, key: &str) -> &str {
        // Due to lifetime constraints, we can't return 'key' directly when not found.
        // We return a reference from the HashMap, which should always exist in practice.
        self.translations.get(key).map(|s| s.as_str()).unwrap_or("")
    }
}

