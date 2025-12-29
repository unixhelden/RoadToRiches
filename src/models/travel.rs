use serde::{Deserialize, Serialize};

/// Repräsentiert einen einzelnen Himmelskörper (Planeten/Mond).
/// Die Aliase helfen dabei, verschiedene CSV-Formate von Spansh zu unterstützen.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Body {
    /// Name des Sternensystems (wird für die Gruppierung benötigt)
    #[serde(alias = "system_name", alias = "System Name", alias = "SystemName")]
    pub system_name: String,
    
    /// Name des Planeten/Mondes
    #[serde(alias = "body_name", alias = "Body Name", alias = "BodyName")]
    pub body_name: String,
    
    /// Typ des Körpers (z.B. "High metal content world")
    #[serde(alias = "body_subtype", alias = "Body Subtype", alias = "BodySubtype")]
    pub body_subtype: String,
    
    /// Entfernung zum Ankunftspunkt im System in Lichtsekunden
    #[serde(alias = "distance", alias = "Distance", default)]
    pub distance: f64,
    
    /// Anzahl der Sprünge, die noch bis zu diesem Ziel nötig sind
    #[serde(alias = "jumps", alias = "Jumps", default)]
    pub jumps: i32,
    
    /// Gibt an, ob der Planet terraformierbar ist ("Yes" oder "No")
    #[serde(alias = "terraformable", alias = "Terraformable", default)]
    pub terraformable: String,
    
    /// Interner Status: "erledigt" oder leer. Wird nicht von Spansh geliefert, 
    /// sondern von unserer App verwaltet.
    #[serde(default)] 
    pub status: String,
}

impl Body {
    /// Check if body is completed
    pub fn is_completed(&self) -> bool {
        self.status == crate::constants::constants::STATUS_COMPLETED
    }
    
    /// Mark body as completed
    pub fn mark_completed(&mut self) {
        self.status = crate::constants::constants::STATUS_COMPLETED.to_string();
    }
    
    /// Mark body as incomplete
    pub fn mark_incomplete(&mut self) {
        self.status = crate::constants::constants::STATUS_INCOMPLETE.to_string();
    }
}

/// Gruppiert alle Planeten, die sich im selben Sternensystem befinden.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemGroup {
    pub name: String,
    pub jumps: i32,
    pub bodies: Vec<Body>,
}