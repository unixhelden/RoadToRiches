use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)] // Added this line to fix CSV errors
pub struct Body {
    pub body_name: String,
    pub body_subtype: String,
    pub distance: f32,
    pub fss_scanned: bool,
    pub dss_mapped: bool,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemGroup { // Added this back so models/mod.rs finds it
    pub name: String,
    pub jumps: i32,
    pub bodies: Vec<Body>,
}

impl Body {
    pub fn is_star(&self) -> bool {
        let s = self.body_subtype.to_lowercase();
        s.contains("star") || s.contains("zwerg") || s.contains("sun")
    }

    pub fn is_completed(&self) -> bool {
        if self.is_star() { self.fss_scanned } 
        else { self.fss_scanned && self.dss_mapped }
    }

    pub fn mark_fss_done(&mut self) { self.fss_scanned = true; }
    pub fn mark_dss_done(&mut self) { self.dss_mapped = true; }

    pub fn mark_completed(&mut self) {
        self.fss_scanned = true;
        self.dss_mapped = true;
        self.status = "erledigt".to_string();
    }

    pub fn mark_incomplete(&mut self) {
        self.fss_scanned = false;
        self.dss_mapped = false;
        self.status = crate::constants::constants::STATUS_INCOMPLETE.to_string();
    }
}