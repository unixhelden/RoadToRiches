use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;
use std::thread;

/// Plays audio from embedded byte data.
/// - `volume`: Range from 0.0 to 1.0.
/// - `audio_data`: The static byte array (e.g., from constants.rs).
pub fn play_sound(volume: f32, audio_data: &'static [u8]) {
    // We spawn a new thread so the audio playback doesn't block the UI thread.
    thread::spawn(move || {
        // Initialize the default audio output device.
        if let Ok((_stream, handle)) = OutputStream::try_default() {
            // Create a Sink which manages playback on the output device.
            if let Ok(sink) = Sink::try_new(&handle) {
                sink.set_volume(volume);
                
                // Wrap the bytes in a Cursor so rodio can read it like a file stream.
                let cursor = Cursor::new(audio_data);
                
                // Decode the audio format (MP3).
                if let Ok(source) = Decoder::new(cursor) {
                    sink.append(source);
                    // Keep this background thread alive until the sound has finished playing.
                    sink.sleep_until_end();
                }
            }
        }
    });
}