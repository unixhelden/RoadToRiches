use serde::{Deserialize, Serialize};

/// Repräsentiert einen einzelnen Eintrag (Zeile) aus der Spansh CSV.
/// Die Felder werden über 'rename' exakt auf die Spaltennamen der CSV gemappt.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawTravelStop {
    #[serde(rename = "System Name")]
    pub system_name: String,
    
    #[serde(rename = "Body Name")]
    pub body_name: String,
    
    #[serde(rename = "Body Subtype")]
    pub body_subtype: String,
    
    #[serde(rename = "Is Terraformable")]
    pub terraformable: String,
    
    #[serde(rename = "Distance To Arrival")]
    pub distance: String,
    
    #[serde(rename = "Estimated Mapping Value")]
    pub value: String,
    
    #[serde(rename = "Jumps")]
    pub jumps: String,

    /// Der Status ("erledigt" oder leer), um den Fortschritt zu speichern.
    #[serde(default)] 
    pub status: String,
}

/// Gruppiert alle Planeten (Bodies) eines Sternensystems für die Anzeige in der GUI.
#[derive(Clone)]
pub struct SystemGroup {
    pub name: String,
    pub jumps: String,
    pub bodies: Vec<RawTravelStop>,
}
