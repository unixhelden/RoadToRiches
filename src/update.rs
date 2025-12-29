use std::sync::mpsc::{channel, Receiver};
use std::thread;
use self_update::backends::github::Update;

pub struct UpdateInfo {
    pub version: String,
    pub body: String,
}

/// Startet den Update-Check in einem Hintergrund-Thread.
/// Gibt einen Receiver zurück, über den die App später das Ergebnis abholen kann.
pub fn spawn_update_check() -> Receiver<Option<UpdateInfo>> {
    let (tx, rx) = channel();

    thread::spawn(move || {
        let result = fetch_latest_release();
        let _ = tx.send(result.ok().flatten());
    });

    rx
}

fn fetch_latest_release() -> Result<Option<UpdateInfo>, Box<dyn std::error::Error>> {
    let releases = Update::configure()
        .repo_owner("unixhelden")
        .repo_name("RoadToRiches")
        .bin_name("roadtoriches")
        .current_version(env!("CARGO_PKG_VERSION"))
        .build()?;

    let latest = releases.get_latest_release()?;
    
    if self_update::version::bump_is_greater(env!("CARGO_PKG_VERSION"), &latest.version)? {
        return Ok(Some(UpdateInfo {
            version: latest.version,
            body: latest.body.unwrap_or_default(),
        }));
    }
    Ok(None)
}

/// Führt den eigentlichen Download und Austausch der EXE aus
pub fn execute_update() -> Result<(), Box<dyn std::error::Error>> {
    Update::configure()
        .repo_owner("unixhelden")
        .repo_name("RoadToRiches")
        .bin_name("roadtoriches")
        .current_version(env!("CARGO_PKG_VERSION"))
        .build()?
        .update()?;
    Ok(())
}
