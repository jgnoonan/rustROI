use vosk::{Model, Recognizer};
use std::sync::Once;

static INIT: Once = Once::new();

pub struct AudioDetector {
    recognizer: Recognizer,
}

impl AudioDetector {
    pub fn new() -> Self {
        // Initialize Vosk logging only once
        INIT.call_once(|| {
            // Note: vosk 0.2 doesn't have LogLevel enum, using default logging
        });

        println!("Loading Vosk model...");
        let model = Model::new("model").expect("Failed to load model");
        println!("Model loaded successfully");

        let recognizer = Recognizer::new(&model, 48000.0)
            .expect("Failed to create recognizer");

        Self { recognizer }
    }

    pub fn process_audio(&mut self, data: &[f32]) -> Option<&'static str> {
        // Convert f32 samples to i16
        let mut i16_data = Vec::with_capacity(data.len());
        for &sample in data {
            let scaled = (sample * 32768.0) as i16;
            i16_data.push(scaled);
        }

        // Process audio data
        if self.recognizer.accept_waveform(&i16_data) {
            // Got a final result
            let result = self.recognizer.final_result();
            if let Some(text) = result.text {
                if !text.is_empty() {
                    println!("Final: {}", text);
                    // Simple command detection
                    if text.to_lowercase().contains("click") {
                        return Some("click");
                    } else if text.to_lowercase().contains("double") {
                        return Some("double");
                    }
                }
            }
        } else {
            // Still processing audio
            let partial = self.recognizer.partial_result();
            if let Some(partial_text) = partial.partial {
                if !partial_text.is_empty() {
                    println!("Partial: {}", partial_text);
                }
            }
        }
        None
    }
}
