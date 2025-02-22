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

    let err_fn = move |err| {
        eprintln!("Error in audio stream: {:?}", err);
    };

    let data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        if let Ok(mut detector) = detector.lock() {
            if let Some(command) = detector.process_audio(data) {
                let _ = tx.send(gui::AppCommand::Click(command));
            }
        }
    };

    Ok(input_device.build_input_stream(
        config,
        data_fn,
        err_fn,
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
        .ok_or_else(|| anyhow!("Could not find Insta360 microphone"))?;
    
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

    // Create a channel for sending commands from the audio thread to the GUI
    let (tx, rx) = std::sync::mpsc::channel();

    // Build the input stream
    let stream = build_input_stream(&input_device, &config, tx)?;
    stream.play()?;

    // Start the GUI
    gui::run(rx, roi_controller)?;

    Ok(())
}
