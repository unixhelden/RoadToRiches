use rodio::{Decoder, OutputStream, Sink};
use std::io::{Cursor, Read};
use std::fs::File;
use std::thread;

/// Plays a scan sound with the specified volume.
/// If sound_file is provided and exists, uses that file; otherwise uses the embedded default.
pub fn play_scan_sound(volume: f32, sound_file: Option<&str>) {
    // Convert to owned String before moving into thread
    let sound_file = sound_file.map(|s| s.to_string());
    
    thread::spawn(move || {
        // 'move' ist wichtig, um 'volume' und 'sound_file' in den Thread zu übertragen
        
        if let Ok((_stream, handle)) = OutputStream::try_default() {
            if let Ok(sink) = Sink::try_new(&handle) {
                sink.set_volume(volume);
                
                // Try to use custom sound file if provided and exists
                let source_result = if let Some(ref path) = sound_file {
                    if std::path::Path::new(path).exists() {
                        File::open(path)
                            .and_then(|mut file| {
                                let mut buffer = Vec::new();
                                file.read_to_end(&mut buffer)?;
                                Ok(buffer)
                            })
                            .ok()
                            .and_then(|data| {
                                Decoder::new(Cursor::new(data)).ok()
                            })
                    } else {
                        None
                    }
                } else {
                    None
                };
                
                // Fallback to embedded sound if custom file not available
                let source = source_result.unwrap_or_else(|| {
                    let audio_data = include_bytes!("../assets/scan_success.mp3");
                    Decoder::new(Cursor::new(audio_data.to_vec())).expect("Embedded sound file should be valid")
                });
                
                sink.append(source);
                sink.sleep_until_end();
            }
        }
    });
}