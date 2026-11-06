use serde::{Serialize, Deserialize};

/// PBR Material with basic parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub name: String,
    pub diffuse_color: [f32; 3],
    pub metallic: f32,
    pub roughness: f32,
    pub emissive: [f32; 3],
}

impl Material {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    /// Get the diffuse color as an egui Color32
    pub fn to_egui_color(&self) -> egui::Color32 {
        let r = (self.diffuse_color[0] * 255.0) as u8;
        let g = (self.diffuse_color[1] * 255.0) as u8;
        let b = (self.diffuse_color[2] * 255.0) as u8;
        egui::Color32::from_rgb(r, g, b)
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            diffuse_color: [0.8, 0.8, 0.8],
            metallic: 0.0,
            roughness: 0.5,
            emissive: [0.0, 0.0, 0.0],
        }
    }
}
