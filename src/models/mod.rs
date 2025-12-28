// Deklaration der Untermodule
pub mod travel;
pub mod settings;

// Re-Export für einfacheren Zugriff (z.B. crate::models::Body statt crate::models::travel::Body)
pub use travel::{Body, SystemGroup};
pub use settings::AppSettings;