// Dummy implementation of AudioDetector for development without audio dependencies
use std::time::{Duration, Instant};

pub struct AudioDetector {
    last_command_time: Instant,
    command_interval: Duration,
}

impl AudioDetector {
    pub fn new() -> Self {
        println!("Creating dummy audio detector for development...");
        Self {
            last_command_time: Instant::now(),
            command_interval: Duration::from_secs(5), // Send a command every 5 seconds for testing
        }
    }

    pub fn process_audio(&mut self, _data: &[f32]) -> Option<&'static str> {
        // For development, we'll just return a command periodically
        // This allows testing the UI without actual audio processing
        
        if self.last_command_time.elapsed() >= self.command_interval {
            self.last_command_time = Instant::now();
            
            // Alternate between different commands for testing
            let current_secs = self.last_command_time.elapsed().as_secs() % 3;
            match current_secs {
                0 => {
                    println!("Dummy audio detector: Simulating 'power' command");
                    return Some("power");
                },
                1 => {
                    println!("Dummy audio detector: Simulating 'start' command");
                    return Some("start");
                },
                _ => {
                    println!("Dummy audio detector: Simulating 'stop' command");
                    return Some("stop");
                }
            }
        }
        
        None
    }
}
