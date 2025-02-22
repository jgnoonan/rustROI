use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use mouse_rs::{Mouse, types::keys::Keys};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Region {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

pub struct ROIController {
    regions: HashMap<String, Region>,
    mouse: Mouse,
}

impl ROIController {
    pub fn new() -> Self {
        Self {
            regions: HashMap::new(),
            mouse: Mouse::new(),
        }
    }

    pub fn add_region(&mut self, name: &str, region: Region) {
        self.regions.insert(name.to_string(), region);
    }

    pub fn load_regions_from_file(&mut self, path: &str) -> std::io::Result<()> {
        let content = std::fs::read_to_string(path)?;
        let regions: HashMap<String, Region> = serde_json::from_str(&content)?;
        self.regions = regions;
        Ok(())
    }

    pub fn click_region(&mut self, name: &str) -> Option<()> {
        if let Some(region) = self.regions.get(name) {
            // Calculate center of region
            let center_x = region.x + (region.width / 2);
            let center_y = region.y + (region.height / 2);
            
            // Move mouse and click
            let _ = self.mouse.move_to(center_x, center_y);
            std::thread::sleep(std::time::Duration::from_millis(100));
            let _ = self.mouse.click(&Keys::LEFT);
            
            Some(())
        } else {
            None
        }
    }

    pub fn get_region(&self, name: &str) -> Option<&Region> {
        self.regions.get(name)
    }
}
