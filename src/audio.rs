use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;

pub fn play_scan_sound() {
    let audio_data = include_bytes!("../assets/scan_success.mp3");
    
    // In 0.17/0.19 ist try_default() die zuverlässigste Methode
    if let Ok((_stream, handle)) = OutputStream::try_default() {
        // In diesen Versionen gibt es Sink::try_new, das ein Result liefert
        if let Ok(sink) = Sink::try_new(&handle) {
            let cursor = Cursor::new(audio_data);
            if let Ok(source) = Decoder::new(cursor) {
                sink.append(source);
                sink.detach(); 
            }
        }
    }
}