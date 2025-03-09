use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Grid, Label, Box, Orientation};
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use crate::roi_controller::ROIController;

// Application commands from other threads
#[derive(Debug, Clone)]
pub enum AppCommand {
    Click(&'static str),
}

// Main application state
pub struct TestInterface {
    command_rx: Arc<Mutex<mpsc::Receiver<AppCommand>>>, // FIXED: Wrapped in Arc<Mutex<>>
    roi_controller: Arc<Mutex<ROIController>>,
    status_text: Arc<Mutex<String>>,
}

impl TestInterface {
    fn new(command_rx: mpsc::Receiver<AppCommand>, roi_controller: Arc<Mutex<ROIController>>) -> Self {
        TestInterface {
            command_rx: Arc::new(Mutex::new(command_rx)), // FIXED: Wrapped in Arc<Mutex<>>
            roi_controller,
            status_text: Arc::new(Mutex::new(String::from("Waiting for voice commands..."))),
        }
    }

    fn run(&self) {
        // Create a very basic GTK application with minimal dependencies
        let app = Application::builder()
            .application_id("com.example.rustROI")
            .build();
            
        app.connect_activate(|app| {
            // Create a simple window with a fixed size
            let window = ApplicationWindow::builder()
                .application(app)
                .title("Raspberry Pi Control Panel")
                .default_width(1280)
                .default_height(800)
                .build();
                
            // Create a vertical box to hold our widgets
            let vbox = Box::new(Orientation::Vertical, 20);
            vbox.set_margin_top(20);
            vbox.set_margin_bottom(20);
            vbox.set_margin_start(20);
            vbox.set_margin_end(20);
            
            // Add a status label at the top
            let status_label = Label::new(Some("Waiting for commands..."));
            status_label.set_margin_bottom(40);
            vbox.append(&status_label);
            
            // Create a grid for the buttons
            let grid = Grid::new();
            grid.set_row_spacing(20);
            grid.set_column_spacing(20);
            grid.set_halign(gtk::Align::Center);
            grid.set_valign(gtk::Align::Center);
            
            // Create buttons with large text and set colors
            let power_button = Button::with_label("POWER");
            power_button.set_size_request(250, 120);
            set_button_colors(&power_button, true); // Red with white text
            
            let start_button = Button::with_label("START");
            start_button.set_size_request(250, 120);
            set_button_colors(&start_button, true); // Red with white text
            
            let stop_button = Button::with_label("STOP");
            stop_button.set_size_request(250, 120);
            set_button_colors(&stop_button, true); // Red with white text
            
            let exit_button = Button::with_label("EXIT");
            exit_button.set_size_request(250, 120);
            // Exit button with default background but larger bold text
            let exit_css_provider = gtk::CssProvider::new();
            exit_css_provider.load_from_data("button { font-weight: bold; font-size: 24px; color: #000000; }");
            exit_button.style_context().add_provider(&exit_css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
            
            // Add buttons to the grid
            grid.attach(&power_button, 0, 0, 1, 1);
            grid.attach(&start_button, 1, 0, 1, 1);
            grid.attach(&stop_button, 0, 1, 1, 1);
            grid.attach(&exit_button, 1, 1, 1, 1);
            
            // Add button click handlers
            power_button.connect_clicked(|_| {
                println!("POWER button clicked");
            });
            
            start_button.connect_clicked(|_| {
                println!("START button clicked");
            });
            
            stop_button.connect_clicked(|_| {
                println!("STOP button clicked");
            });
            
            exit_button.connect_clicked(|_| {
                std::process::exit(0);
            });
            
            // Add the grid to the vertical box
            vbox.append(&grid);
            
            // Set the vertical box as the window's child
            window.set_child(Some(&vbox));
            
            // Show all widgets
            window.present();
        });
        
        // Run the application
        app.run();
    }
}

// ROIController is imported from roi_controller.rs

// Helper function to set button colors
fn set_button_colors(button: &Button, is_red: bool) {
    // Create a CSS provider for custom styling
    let css_provider = gtk::CssProvider::new();
    
    // Define CSS for red button with white text
    let css = if is_red {
        "button { background: #FF0000; color: #FFFFFF; font-weight: bold; font-size: 24px; }"
    } else {
        "button { background: #00FF00; color: #000000; font-weight: bold; font-size: 24px; }"
    };
    
    css_provider.load_from_data(css);
    
    // Get the button's style context and add the provider
    let style_context = button.style_context();
    style_context.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
}

// Run function
pub fn run(
    command_rx: mpsc::Receiver<AppCommand>,
    roi_controller: Arc<Mutex<ROIController>>,
) -> anyhow::Result<()> {
    let interface = TestInterface::new(command_rx, roi_controller);
    interface.run();
    Ok(())
}
