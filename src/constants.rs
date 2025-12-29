/// Application-wide constants
pub mod constants {
    /// Status string for completed bodies
    pub const STATUS_COMPLETED: &str = "erledigt";
    
    /// Status string for incomplete bodies
    pub const STATUS_INCOMPLETE: &str = "";
    
    /// Interval for checking log files (seconds)
    pub const LOG_CHECK_INTERVAL_SECS: u64 = 2;
    
    /// Duration for copy feedback (milliseconds)
    pub const COPY_FEEDBACK_DURATION_MS: u64 = 800;
    
    /// Default content width for route view (pixels)
    pub const ROUTE_CONTENT_WIDTH: f32 = 500.0;
    
    /// Default settings filename
    pub const SETTINGS_FILENAME: &str = "settings.json";
}

