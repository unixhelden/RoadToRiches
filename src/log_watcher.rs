use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::PathBuf;
use std::collections::HashMap;
use serde_json::Value;

/// Definiert die verschiedenen Arten von Scans, die wir tracken wollen.
#[derive(Debug, Clone)]
pub enum ScanEvent {
    FSS(String), // Full Spectrum Scanner / System Scan
    DSS(String), // Detailed Surface Scanner / Mapping
}

/// Sucht nach 'Scan' (FSS) und 'SAAScanComplete' (DSS) Events.
/// Tracks file position to avoid re-reading the same lines.
pub fn check_for_scans(
    os_mode: crate::models::OsMode, 
    manual_dir: &str,
    file_positions: &mut HashMap<String, u64>
) -> (Vec<ScanEvent>, Option<PathBuf>) {
    let mut found_events = Vec::new();

    // Pfad bestimmen
    let log_path = if !manual_dir.is_empty() {
        find_latest_in_dir(PathBuf::from(manual_dir))
    } else {
        auto_find_logs(os_mode)
    };

    if let Some(ref path) = log_path {
        let path_str = path.to_string_lossy().to_string();

        if let Ok(mut file) = File::open(&path) {
            let start_pos = file_positions.get(&path_str).copied().unwrap_or(0);
            let file_size = file.metadata().map(|m| m.len()).unwrap_or(0);
            
            let actual_start_pos = if start_pos > file_size { 0 } else { start_pos };
            
            if let Err(e) = file.seek(SeekFrom::Start(actual_start_pos)) {
                eprintln!("Fehler beim Positionieren in Log-Datei {}: {}", path_str, e);
                file_positions.remove(&path_str);
            } else {
                let reader = BufReader::new(&mut file);
                let mut current_pos = actual_start_pos;
                
                for line in reader.lines() {
                    match line {
                        Ok(line) => {
                            current_pos += line.len() as u64 + 1;
                            
                            if let Ok(v) = serde_json::from_str::<Value>(&line) {
                                let event_type = v["event"].as_str().unwrap_or("");

                                match event_type {
                                    // FSS Scan (oder passiv im Vorbeiflug)
                                    "Scan" => {
                                        if let Some(body_name) = v["BodyName"].as_str() {
                                            // Wir ignorieren Gürtel-Cluster (Belt Clusters), da diese nicht relevant sind
                                            if !body_name.contains("Belt Cluster") {
                                                found_events.push(ScanEvent::FSS(body_name.to_string()));
                                            }
                                        }
                                    },
                                    // DSS / SAA Mapping Scan (Sonden)
                                    "SAAScanComplete" => {
                                        if let Some(body_name) = v["BodyName"].as_str() {
                                            found_events.push(ScanEvent::DSS(body_name.to_string()));
                                        }
                                    },
                                    _ => {}
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Fehler beim Lesen der Log-Zeile: {}", e);
                            break;
                        }
                    }
                }
                
                let final_pos = current_pos.min(file_size);
                file_positions.insert(path_str, final_pos);
            }
        }
    }
    (found_events, log_path)
}

/// Ermittelt den Standard-Log-Pfad basierend auf dem Betriebssystem.
fn auto_find_logs(os_mode: crate::models::OsMode) -> Option<PathBuf> {
    let path = dirs::home_dir()?;
    let sub_path = match os_mode {
        crate::models::OsMode::Windows => "Saved Games/Frontier Developments/Elite Dangerous",
        crate::models::OsMode::Linux => ".steam/steam/steamapps/compatdata/359320/pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous",
        crate::models::OsMode::Custom => return None,
    };
    find_latest_in_dir(path.join(sub_path))
}

/// Hilfsfunktion, die die zeitlich letzte .log Datei in einem Ordner findet.
fn find_latest_in_dir(dir: PathBuf) -> Option<PathBuf> {
    let entries = std::fs::read_dir(dir).ok()?;
    let mut logs: Vec<_> = entries.filter_map(|e| e.ok()).map(|e| e.path())
        .filter(|p| p.extension().map_or(false, |ext| ext == "log")).collect();
    
    logs.sort_by(|a, b| {
        b.metadata().and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::now())
        .cmp(&a.metadata().and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::UNIX_EPOCH))
    });
    
    logs.into_iter().next()
}