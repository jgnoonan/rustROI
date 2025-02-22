use vosk::{Model, Recognizer, LogLevel};
use std::sync::Once;

static INIT: Once = Once::new();

pub struct AudioDetector {
    recognizer: Recognizer,
}

impl AudioDetector {
    pub fn new() -> Self {
        // Initialize Vosk logging only once
        INIT.call_once(|| {
            vosk::set_log_level(LogLevel::Info);
        });

        println!("Loading Vosk model...");
        let model = Model::new("model").expect("Failed to load model");
        println!("Model loaded successfully");

        let mut recognizer = Recognizer::new(&model, 48000.0)
            .expect("Failed to create recognizer");
            
        // Configure recognizer
        recognizer.set_words(true);
        recognizer.set_partial_words(true);

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
        if let Ok(state) = self.recognizer.accept_waveform(&i16_data) {
            match state {
                vosk::DecodingState::Running => {
                    // Still processing audio
                    let partial = self.recognizer.partial_result();
                    if !partial.partial_result.is_empty() {
                        println!("Partial: {:?}", partial.partial_result);
                    }
                }
                vosk::DecodingState::Finalized => {
                    // Got a final result
                    let result = self.recognizer.final_result();
                    if let Some(result) = result.single() {
                        let text = result.text;
                        println!("Vosk result: {:?}", text);
                        
                        // Map detected words to commands
                        match text.to_lowercase().as_str() {
                            text if text.contains("power") => return Some("power"),
                            text if text.contains("start") || text.contains("go") => return Some("start"),
                            text if text.contains("stop") || text.contains("halt") => return Some("stop"),
                            _ => {
                                if !text.is_empty() {
                                    println!("No matching command found for: {}", text);
                                }
                                return None
                            }
                        }
                    }
                }
                vosk::DecodingState::Failed => {
                    println!("Error: Speech recognition failed");
                }
            }
        }

        None
    }
}
