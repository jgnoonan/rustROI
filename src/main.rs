use anyhow::Result;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

mod egui_gui;
mod roi_controller;
mod audio_detector;

// Use the egui_gui module for AppCommand
use egui_gui::AppCommand;
use roi_controller::ROIController;

// Simulate audio commands with a simple timer thread
fn start_command_simulator(tx: mpsc::Sender<AppCommand>) {
    thread::spawn(move || {
        let mut detector = audio_detector::AudioDetector::new();
        let dummy_data = vec![0.0f32; 1024]; // Dummy audio data

        loop {
            // Process dummy audio data to get simulated commands
            if let Some(command) = detector.process_audio(&dummy_data) {
                let _ = tx.send(AppCommand::Click(command));
            }

            // Sleep to avoid consuming too much CPU
            thread::sleep(Duration::from_millis(100));
        }
    });
}

fn main() -> Result<()> {
    // Initialize ROI Controller
    let roi_controller = Arc::new(Mutex::new(ROIController::new()));

    // Load regions from file (if available)
    {
        let mut controller = roi_controller.lock().unwrap();
        let regions_result = controller.load_regions_from_file("regions.json");
        if let Err(e) = regions_result {
            println!("Warning: Could not load regions from file: {}", e);
            println!("Continuing with default regions...");
        }
    }

    // Create a channel for sending commands from the simulator thread to the GUI
    let (tx, rx) = mpsc::channel();

    // Start the command simulator in a separate thread
    println!("Starting command simulator for testing...");
    start_command_simulator(tx);

    // Start the GUI using GTK4 implementation (not egui)
    println!("Starting GUI with GTK4 implementation...");
    egui_gui::run(rx, roi_controller)?;

    Ok(())
}
