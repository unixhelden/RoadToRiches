use std::fs::File;
use std::io::BufReader;
use std::thread;
use rodio::{Decoder, OutputStream, Sink};

/// Spielt einen Sound ab. 
/// - `volume`: Die Lautstärke (0.0 bis 1.0)
/// - `custom_path`: Optionaler Pfad zu einer eigenen Datei
/// - `default_bytes`: Die eingebetteten Standard-Bytes (aus constants.rs)
pub fn play_sound(volume: f32, custom_path: Option<String>, default_bytes: &'static [u8]) {
    // Wir klonen den Pfad für den Thread
    let path_clone = custom_path.clone();

    thread::spawn(move || {
        // 1. Audio-Output initialisieren
        let (_stream, stream_handle) = match OutputStream::try_default() {
            Ok(s) => s,
            Err(_) => return, // Kein Audiogerät gefunden
        };

        let sink = match Sink::try_new(&stream_handle) {
            Ok(s) => s,
            Err(_) => return,
        };

        sink.set_volume(volume);

        // 2. Sound-Quelle wählen
        let mut loaded_custom = false;

        // Versuche Custom-Pfad zu laden, wenn vorhanden
        if let Some(path) = path_clone {
            if !path.is_empty() {
                if let Ok(file) = File::open(path) {
                    let reader = BufReader::new(file);
                    if let Ok(source) = Decoder::new(reader) {
                        sink.append(source);
                        loaded_custom = true;
                    }
                }
            }
        }

        // 3. Fallback auf Standard-Bytes, wenn Custom nicht geladen wurde
        if !loaded_custom {
            let cursor = std::io::Cursor::new(default_bytes);
            if let Ok(source) = Decoder::new(cursor) {
                sink.append(source);
            }
        }

        sink.sleep_until_end();
    });
}