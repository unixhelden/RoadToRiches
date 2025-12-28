use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;
use std::thread;

pub fn play_scan_sound() {
    thread::spawn(|| {
        println!("DEBUG: Audio-Thread gestartet...");
        let audio_data = include_bytes!("../assets/scan_success.mp3");
        
        match OutputStream::try_default() {
            Ok((_stream, handle)) => {
                println!("DEBUG: Audio-Device erfolgreich geöffnet.");
                if let Ok(sink) = Sink::try_new(&handle) {
                    let cursor = Cursor::new(audio_data);
                    if let Ok(source) = Decoder::new(cursor) {
                        sink.append(source);
                        println!("DEBUG: Sound wird abgespielt...");
                        sink.sleep_until_end();
                        println!("DEBUG: Sound fertig.");
                    }
                }
            }
            Err(e) => println!("DEBUG: Fehler beim Öffnen des Audio-Devices: {:?}", e),
        }
    });
}