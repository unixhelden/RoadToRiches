use std::env;
use std::path::PathBuf;

fn main() {
    // Nur ausführen, wenn wir für Windows bauen
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        
        // Holt den Pfad zum Projektordner auf dem GitHub-Server
        let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let mut icon_path = PathBuf::from(project_dir);
        
        // Zeigt auf assets/icon.ico (Stelle sicher, dass der Ordner 'assets' klein geschrieben ist!)
        icon_path.push("assets");
        icon_path.push("icon.ico");

        if icon_path.exists() {
            res.set_icon(icon_path.to_str().unwrap());
        } else {
            panic!("Icon nicht gefunden unter: {:?}", icon_path);
        }

        res.compile().unwrap();
    }
}