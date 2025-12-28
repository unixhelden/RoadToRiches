use crate::models::{RawTravelStop, SystemGroup};
use std::error::Error;
use std::fs;

/// Lädt die CSV-Datei und gruppiert die einzelnen Planeten nach Systemnamen.
pub fn load_and_group(path: &str) -> Result<Vec<SystemGroup>, Box<dyn Error>> {
    let file = fs::File::open(path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    
    let mut groups: Vec<SystemGroup> = Vec::new();

    for result in rdr.deserialize() {
        let stop: RawTravelStop = result?;
        
        // Suchen, ob das System bereits in unserer Liste existiert
        if let Some(group) = groups.iter_mut().find(|g| g.name == stop.system_name) {
            group.bodies.push(stop);
        } else {
            // Neues System anlegen
            groups.push(SystemGroup {
                name: stop.system_name.clone(),
                jumps: stop.jumps.clone(),
                bodies: vec![stop],
            });
        }
    }
    Ok(groups)
}

/// Speichert die Daten sicher: Schreibt erst eine temporäre Datei und erstellt ein Backup.
pub fn save_all(path: &str, groups: &[SystemGroup]) -> Result<(), Box<dyn Error>> {
    // Sicherheits-Check: Wenn die Liste leer ist, wurde evtl. falsch geladen.
    // Wir überschreiben dann nicht, um Datenverlust zu vermeiden.
    if groups.is_empty() {
        return Err("Speichern abgebrochen: Keine Daten vorhanden (Schutz vor Leeren der Datei).".into());
    }

    let temp_path = format!("{}.tmp", path);
    let backup_path = format!("{}.bak", path);

    // 1. Daten in eine temporäre Datei schreiben
    {
        let mut wtr = csv::Writer::from_path(&temp_path)?;
        for group in groups {
            for body in &group.bodies {
                wtr.serialize(body)?;
            }
        }
        wtr.flush()?;
    }

    // 2. Bestehende Datei als Backup sichern
    if fs::metadata(path).is_ok() {
        fs::copy(path, &backup_path)?;
    }

    // 3. Temporäre Datei zur echten Datei machen (Atomares Ersetzen)
    fs::rename(&temp_path, path)?;

    Ok(())
}
