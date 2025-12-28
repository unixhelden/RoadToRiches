use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;
use std::thread;

// Kann nun die Lautstärke empfangen.
pub fn play_scan_sound(volume: f32) {
    thread::spawn(move || { // 'move' ist wichtig, um 'volume' in den Thread zu übertragen
        let audio_data = include_bytes!("../assets/scan_success.mp3");
        
        if let Ok((_stream, handle)) = OutputStream::try_default() {
            if let Ok(sink) = Sink::try_new(&handle) {
                // Hier wird die Lautstärke gesetzt
                sink.set_volume(volume);
                
                let cursor = Cursor::new(audio_data);
                if let Ok(source) = Decoder::new(cursor) {
                    sink.append(source);
                    sink.sleep_until_end(); 
                }
            }
        }
    });
}