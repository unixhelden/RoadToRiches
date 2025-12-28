use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use serde_json::Value;

/// Sucht nach dem neuesten 'Scan' Event in der aktuellsten Log-Datei.
/// Gibt den Namen des gescannten Objekts zurück, falls eines gefunden wurde.
pub fn check_for_scans(os_mode: &str, manual_dir: &str) -> Vec<String> {
    let mut found_bodies = Vec::new();

    // Pfad bestimmen: Entweder manuell gesetzt oder automatisch ermittelt
    let log_path = if !manual_dir.is_empty() {
        find_latest_in_dir(PathBuf::from(manual_dir))
    } else {
        auto_find_logs(os_mode)
    };

    if let Some(path) = log_path {
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            
            // Wir gehen durch jede Zeile der Log-Datei
            for line in reader.lines().filter_map(|l| l.ok()) {
                if let Ok(v) = serde_json::from_str::<Value>(&line) {
                    // Wir suchen nach dem Event "Scan"
                    if v["event"] == "Scan" {
                        if let Some(body_name) = v["BodyName"].as_str() {
                            found_bodies.push(body_name.to_string());
                        }
                    }
                }
            }
        }
    }
    found_bodies
}

/// Ermittelt den Standard-Log-Pfad basierend auf dem Betriebssystem.
fn auto_find_logs(os_mode: &str) -> Option<PathBuf> {
    let path = dirs::home_dir()?;
    let sub_path = if os_mode == "Windows" {
        "Saved Games/Frontier Developments/Elite Dangerous"
    } else {
        ".steam/steam/steamapps/compatdata/359320/pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous"
    };
    find_latest_in_dir(path.join(sub_path))
}

/// Hilfsfunktion, die die zeitlich letzte .log Datei in einem Ordner findet.
fn find_latest_in_dir(dir: PathBuf) -> Option<PathBuf> {
    let entries = std::fs::read_dir(dir).ok()?;
    let mut logs: Vec<_> = entries.filter_map(|e| e.ok()).map(|e| e.path())
        .filter(|p| p.extension().map_or(false, |ext| ext == "log")).collect();
    
    // Sortieren nach Änderungsdatum (Neueste zuerst)
    logs.sort_by(|a, b| {
        b.metadata().and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::now())
        .cmp(&a.metadata().and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::UNIX_EPOCH))
    });
    
    logs.into_iter().next()
}
