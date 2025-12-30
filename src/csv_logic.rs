use crate::models::{Body, SystemGroup};
use std::error::Error;
use std::fs;
use serde::{Deserialize, Serialize};

/// Internal helper struct to match the EXACT columns in your CSV file.
/// This acts as a bridge between the raw CSV and your App-Models.
#[derive(Debug, Deserialize, Serialize)]
struct CsvRow {
    pub system_name: String,
    pub jumps: i32,
    pub body_name: String,
    pub body_subtype: String,
    pub distance: f32,
    pub fss_scanned: bool,
    pub dss_mapped: bool,
    pub status: String,
}

/// Loads the CSV and groups bodies by their system name.
pub fn load_and_group(path: &str) -> Result<Vec<SystemGroup>, Box<dyn Error>> {
    let file = fs::File::open(path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    
    let mut groups: Vec<SystemGroup> = Vec::new();

    for (index, result) in rdr.deserialize::<CsvRow>().enumerate() {
        let row: CsvRow = match result {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Error in CSV line {}: {}", index + 1, e);
                continue; 
            }
        };

        // Create a Body object from the CSV row data
        let body = Body {
            body_name: row.body_name,
            body_subtype: row.body_subtype,
            distance: row.distance,
            fss_scanned: row.fss_scanned,
            dss_mapped: row.dss_mapped,
            status: row.status,
        };
        
        // Grouping Logic: Check if system group already exists
        if let Some(group) = groups.iter_mut().find(|g| g.name == row.system_name) {
            group.bodies.push(body);
        } else {
            // Create a new group if it's the first time we see this system
            groups.push(SystemGroup {
                name: row.system_name,
                jumps: row.jumps,
                bodies: vec![body],
            });
        }
    }

    println!("Load complete. Found {} systems.", groups.len());
    Ok(groups)
}

/// Saves the current state back to CSV by flattening the groups.
pub fn save_all(path: &str, groups: &[SystemGroup]) -> Result<(), Box<dyn Error>> {
    if groups.is_empty() {
        return Err("Save aborted: List is empty.".into());
    }

    let temp_path = format!("{}.tmp", path);
    let backup_path = format!("{}.bak", path);

    {
        let mut wtr = csv::Writer::from_path(&temp_path)?;
        for group in groups {
            for body in &group.bodies {
                // We convert our nested model back into the flat CSV row format
                let row = CsvRow {
                    system_name: group.name.clone(),
                    jumps: group.jumps,
                    body_name: body.body_name.clone(),
                    body_subtype: body.body_subtype.clone(),
                    distance: body.distance,
                    fss_scanned: body.fss_scanned,
                    dss_mapped: body.dss_mapped,
                    status: body.status.clone(),
                };
                wtr.serialize(row)?;
            }
        }
        wtr.flush()?;
    }

    // Safety: Create backup before overwriting
    if fs::metadata(path).is_ok() {
        fs::copy(path, &backup_path)?;
    }

    fs::rename(&temp_path, path)?;
    Ok(())
}