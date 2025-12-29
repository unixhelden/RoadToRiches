use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::PathBuf;
use std::collections::HashMap;
use serde_json::Value;

/// Sucht nach dem neuesten 'Scan' Event in der aktuellsten Log-Datei.
/// Gibt den Namen des gescannten Objekts zurück, falls eines gefunden wurde.
/// Tracks file position to avoid re-reading the same lines.
pub fn check_for_scans(
    os_mode: crate::models::OsMode, 
    manual_dir: &str,
    file_positions: &mut HashMap<String, u64>
) -> Vec<String> {
    let mut found_bodies = Vec::new();

    // Pfad bestimmen: Entweder manuell gesetzt oder automatisch ermittelt
    let log_path = if !manual_dir.is_empty() {
        find_latest_in_dir(PathBuf::from(manual_dir))
    } else {
        auto_find_logs(os_mode)
    };

        if let Some(path) = log_path {
            let path_str = path.to_string_lossy().to_string();
            
            if let Ok(mut file) = File::open(&path) {
                // Get the last known position for this file, or start from beginning
                let start_pos = file_positions.get(&path_str).copied().unwrap_or(0);
                
                // Get current file size to detect if file was truncated
                let file_size = file.metadata()
                    .map(|m| m.len())
                    .unwrap_or(0);
                
                // If file is smaller than our last position, it was likely rotated/truncated
                let actual_start_pos = if start_pos > file_size {
                    0
                } else {
                    start_pos
                };
                
                // Seek to the last known position
                if let Err(e) = file.seek(SeekFrom::Start(actual_start_pos)) {
                    eprintln!("Fehler beim Positionieren in Log-Datei {}: {}", path_str, e);
                    // If seek fails, reset position tracking
                    file_positions.remove(&path_str);
                } else {
                    // Use BufReader for efficient line reading
                    let reader = BufReader::new(&mut file);
                    
                    // Track current position (start from where we seeked)
                    let mut current_pos = actual_start_pos;
                    
                    // Wir gehen durch jede neue Zeile der Log-Datei
                    for line in reader.lines() {
                        match line {
                            Ok(line) => {
                                // Update position: line length + newline character
                                current_pos += line.len() as u64 + 1;
                                
                                if let Ok(v) = serde_json::from_str::<Value>(&line) {
                                    // Wir suchen nach dem Event "Scan"
                                    if v["event"] == "Scan" {
                                        if let Some(body_name) = v["BodyName"].as_str() {
                                            found_bodies.push(body_name.to_string());
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Fehler beim Lesen der Log-Zeile: {}", e);
                                break;
                            }
                        }
                    }
                    
                    // Update the position for next time
                    // Use file_size if we've read past it (shouldn't happen, but safety check)
                    let final_pos = current_pos.min(file_size);
                    file_positions.insert(path_str, final_pos);
                }
            }
        }
    found_bodies
}

/// Ermittelt den Standard-Log-Pfad basierend auf dem Betriebssystem.
fn auto_find_logs(os_mode: crate::models::OsMode) -> Option<PathBuf> {
    let path = dirs::home_dir()?;
    let sub_path = match os_mode {
        crate::models::OsMode::Windows => "Saved Games/Frontier Developments/Elite Dangerous",
        crate::models::OsMode::Linux => ".steam/steam/steamapps/compatdata/359320/pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous",
        crate::models::OsMode::Custom => return None, // Custom mode should use manual_dir
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
