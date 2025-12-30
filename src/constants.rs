/// Application-wide constants
pub mod constants {
    /// String used in CSV for unfinished bodies
    pub const STATUS_INCOMPLETE: &str = "";
    
    /// How often to poll the Elite Journal files (in seconds)
    pub const LOG_CHECK_INTERVAL_SECS: u64 = 2;
    
    /// How long the "Copied!" message stays visible (in milliseconds)
    pub const COPY_FEEDBACK_DURATION_MS: u64 = 800;
    
    /// Filename for the app configuration
    pub const SETTINGS_FILENAME: &str = "settings.json";

    // --- AUDIO ASSETS ---
    // include_bytes! embeds the files into your .exe during compilation.
    // The path is relative to this file's location.
    
    /// Sound data for FSS (System-wide Discovery Scan)
    pub const SOUND_FSS: &[u8] = include_bytes!("../assets/fss_scanned.mp3");

    /// Sound data for DSS (Planetary Surface Mapping)
    pub const SOUND_DSS: &[u8] = include_bytes!("../assets/dss_mapped.mp3");
}