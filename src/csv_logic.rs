use crate::models::{Body, SystemGroup};
use std::error::Error;
use std::fs;

/// Lädt die CSV-Datei und gruppiert die einzelnen Planeten nach Systemnamen.
pub fn load_and_group(path: &str) -> Result<Vec<SystemGroup>, Box<dyn Error>> {
    let file = fs::File::open(path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    
    let mut groups: Vec<SystemGroup> = Vec::new();

    for (index, result) in rdr.deserialize().enumerate() {
        // Falls eine Zeile nicht gelesen werden kann, gibt uns das Terminal jetzt Bescheid
        let stop: Body = match result {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Fehler in CSV-Zeile {}: {}", index + 1, e);
                continue; // Überspringt die kaputte Zeile, statt abzubrechen
            }
        };
        
        // Gruppierungs-Logik
        if let Some(group) = groups.iter_mut().find(|g| g.name == stop.system_name) {
            group.bodies.push(stop);
        } else {
            groups.push(SystemGroup {
                name: stop.system_name.clone(),
                jumps: stop.jumps,
                bodies: vec![stop],
            });
        }
    }

    // Diagnose-Check: Wie viele Systeme wurden geladen?
    println!("Ladevorgang abgeschlossen. {} Systeme gefunden.", groups.iter().count());
    
    Ok(groups)
}

/// Speichert die Daten sicher mit Backup.
pub fn save_all(path: &str, groups: &[SystemGroup]) -> Result<(), Box<dyn Error>> {
    if groups.is_empty() {
        return Err("Speichern abgebrochen: Liste ist leer.".into());
    }

    let temp_path = format!("{}.tmp", path);
    let backup_path = format!("{}.bak", path);

    {
        let mut wtr = csv::Writer::from_path(&temp_path)?;
        for group in groups {
            for body in &group.bodies {
                wtr.serialize(body)?;
            }
        }
        wtr.flush()?;
    }

    if fs::metadata(path).is_ok() {
        fs::copy(path, &backup_path)?;
    }

    fs::rename(&temp_path, path)?;
    Ok(())
}