use anyhow::{Result, anyhow};
use cpal;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

mod gui;
mod roi_controller;
mod audio_detector;

use audio_detector::AudioDetector;

const SAMPLE_RATE: u32 = 48000;

fn build_input_stream(
    input_device: &cpal::Device,
    config: &cpal::StreamConfig,
    tx: std::sync::mpsc::Sender<gui::AppCommand>,
) -> Result<cpal::Stream> {
    let detector = Arc::new(Mutex::new(AudioDetector::new()));

    Ok(input_device.build_input_stream(
        config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            if let Ok(mut detector) = detector.lock() {
                if let Some(command) = detector.process_audio(data) {
                    let _ = tx.send(gui::AppCommand::Click(command));
                }
            }
        },
        move |err| {
            eprintln!("Error in audio stream: {:?}", err);
        },
        None,
    )?)
}

fn main() -> Result<()> {
    // Initialize ROI Controller
    let mut roi_controller = roi_controller::ROIController::new();
    roi_controller.load_regions_from_file("regions.json")?;
    let roi_controller = Arc::new(Mutex::new(roi_controller));

    // Initialize audio
    let host = cpal::default_host();
    
    // Find camera microphone
    let input_device = host.input_devices()?
        .find(|device| {
            if let Ok(name) = device.name() {
                name.to_lowercase().contains("insta360")
            } else {
                false
            }
        })
        .ok_or_else(|| anyhow!("Failed to find camera microphone. Please ensure it is connected."))?;
    
    println!("Audio host: {}", host.id().name());
    println!("Using input device: {}", input_device.name()?);

    // Get supported configs
    let supported_configs = input_device.supported_input_configs()?;
    println!("Supported configs:");
    for config in supported_configs {
        println!("  {:?}", config);
    }

    let config = cpal::StreamConfig {
        channels: 1,
        sample_rate: cpal::SampleRate(SAMPLE_RATE),
        buffer_size: cpal::BufferSize::Default,
    };

    println!("Using audio config: {:?}", config);

    // Create channels for communication between audio and GUI threads
    let (tx, rx) = std::sync::mpsc::channel();
    let (command_tx, command_rx) = std::sync::mpsc::channel();

    // Start audio processing in a separate thread
    let audio_thread = std::thread::spawn(move || {
        println!("Building input stream...");
        match build_input_stream(&input_device, &config, tx) {
            Ok(stream) => {
                println!("Starting audio stream...");
                stream.play().unwrap();
                println!("Audio stream started successfully");

                // Keep the stream alive
                loop {
                    if let Ok(_) = command_rx.try_recv() {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
            Err(err) => {
                println!("Error building input stream: {}", err);
            }
        }
    });

    // Run GUI on the main thread
    if let Err(err) = gui::run_gui(rx, command_tx, roi_controller) {
        println!("Error: GUI thread error: {}", err);
    }

    // Wait for audio thread to finish
    if let Err(err) = audio_thread.join() {
        println!("Error: Audio thread panicked: {:?}", err);
    }

    Ok(())
}
