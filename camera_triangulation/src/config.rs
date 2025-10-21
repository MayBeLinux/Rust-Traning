// Configuration avancée pour différents types de caméras et scénarios

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    pub name: String,
    pub focal_length: f64,
    pub principal_point: (f64, f64),
    pub distortion_coeffs: Vec<f64>,
    pub resolution: (u32, u32),
    pub fps: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriangulationConfig {
    pub camera_height: f64,
    pub tilt_angle: f64,
    pub object_types: Vec<ObjectTypeConfig>,
    pub detection_params: DetectionParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectTypeConfig {
    pub name: String,
    pub real_size: f64,  // Taille réelle en mètres
    pub min_pixel_size: f64,
    pub max_pixel_size: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionParams {
    pub min_contour_area: f64,
    pub scale_factor: f64,
    pub min_neighbors: i32,
    pub confidence_threshold: f64,
}

impl Default for CameraConfig {
    fn default() -> Self {
        // Configuration pour webcam standard 640x480
        Self {
            name: "Webcam Standard".to_string(),
            focal_length: 500.0,
            principal_point: (320.0, 240.0),
            distortion_coeffs: vec![0.0, 0.0, 0.0, 0.0, 0.0],
            resolution: (640, 480),
            fps: 30.0,
        }
    }
}

impl Default for TriangulationConfig {
    fn default() -> Self {
        Self {
            camera_height: 1.5, // 1.5 mètres du sol
            tilt_angle: 0.0,    // Horizontal
            object_types: vec![
                ObjectTypeConfig {
                    name: "Visage".to_string(),
                    real_size: 0.25, // 25cm de large
                    min_pixel_size: 30.0,
                    max_pixel_size: 300.0,
                },
                ObjectTypeConfig {
                    name: "Personne".to_string(),
                    real_size: 0.6, // 60cm de large (épaules)
                    min_pixel_size: 50.0,
                    max_pixel_size: 400.0,
                },
                ObjectTypeConfig {
                    name: "Objet".to_string(),
                    real_size: 0.15, // 15cm
                    min_pixel_size: 20.0,
                    max_pixel_size: 200.0,
                },
            ],
            detection_params: DetectionParams {
                min_contour_area: 1000.0,
                scale_factor: 1.1,
                min_neighbors: 3,
                confidence_threshold: 0.5,
            },
        }
    }
}

// Configurations prédéfinies pour différents scénarios
impl CameraConfig {
    pub fn webcam_hd() -> Self {
        Self {
            name: "Webcam HD".to_string(),
            focal_length: 800.0,
            principal_point: (640.0, 360.0),
            resolution: (1280, 720),
            fps: 30.0,
            ..Default::default()
        }
    }
    
    pub fn smartphone_camera() -> Self {
        Self {
            name: "Smartphone".to_string(),
            focal_length: 1200.0,
            principal_point: (960.0, 540.0),
            resolution: (1920, 1080),
            fps: 60.0,
            distortion_coeffs: vec![0.1, -0.2, 0.0, 0.0, 0.0],
        }
    }
    
    pub fn security_camera() -> Self {
        Self {
            name: "Caméra de sécurité".to_string(),
            focal_length: 600.0,
            principal_point: (400.0, 300.0),
            resolution: (800, 600),
            fps: 25.0,
            distortion_coeffs: vec![0.05, -0.1, 0.0, 0.0, 0.0],
        }
    }
}

impl TriangulationConfig {
    pub fn indoor_monitoring() -> Self {
        Self {
            camera_height: 2.5,
            tilt_angle: -0.2, // Légèrement inclinée vers le bas
            ..Default::default()
        }
    }
    
    pub fn outdoor_surveillance() -> Self {
        Self {
            camera_height: 4.0,
            tilt_angle: -0.3,
            object_types: vec![
                ObjectTypeConfig {
                    name: "Personne".to_string(),
                    real_size: 0.6,
                    min_pixel_size: 20.0,
                    max_pixel_size: 150.0,
                },
                ObjectTypeConfig {
                    name: "Véhicule".to_string(),
                    real_size: 2.0,
                    min_pixel_size: 50.0,
                    max_pixel_size: 400.0,
                },
            ],
            detection_params: DetectionParams {
                min_contour_area: 2000.0,
                scale_factor: 1.2,
                min_neighbors: 5,
                confidence_threshold: 0.7,
            },
        }
    }
    
    pub fn desktop_interaction() -> Self {
        Self {
            camera_height: 0.8, // Caméra sur le bureau
            tilt_angle: 0.1,    // Légèrement vers le haut
            object_types: vec![
                ObjectTypeConfig {
                    name: "Main".to_string(),
                    real_size: 0.12, // 12cm
                    min_pixel_size: 30.0,
                    max_pixel_size: 200.0,
                },
                ObjectTypeConfig {
                    name: "Visage".to_string(),
                    real_size: 0.25,
                    min_pixel_size: 50.0,
                    max_pixel_size: 300.0,
                },
                ObjectTypeConfig {
                    name: "Objet bureau".to_string(),
                    real_size: 0.08, // 8cm (stylo, souris, etc.)
                    min_pixel_size: 15.0,
                    max_pixel_size: 100.0,
                },
            ],
            detection_params: DetectionParams {
                min_contour_area: 500.0,
                scale_factor: 1.05,
                min_neighbors: 2,
                confidence_threshold: 0.4,
            },
        }
    }
}

// Utilitaires pour sauvegarder/charger la configuration
impl CameraConfig {
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
    
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let config = serde_json::from_str(&json)?;
        Ok(config)
    }
}

impl TriangulationConfig {
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
    
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let config = serde_json::from_str(&json)?;
        Ok(config)
    }
}

// Fonction pour créer des configurations d'exemple
pub fn create_example_configs() -> Result<(), Box<dyn std::error::Error>> {
    // Sauvegarder des configurations d'exemple
    CameraConfig::webcam_hd().save_to_file("camera_hd.json")?;
    CameraConfig::smartphone_camera().save_to_file("camera_smartphone.json")?;
    
    TriangulationConfig::indoor_monitoring().save_to_file("triangulation_indoor.json")?;
    TriangulationConfig::outdoor_surveillance().save_to_file("triangulation_outdoor.json")?;
    TriangulationConfig::desktop_interaction().save_to_file("triangulation_desktop.json")?;
    
    println!("✅ Configurations d'exemple créées avec succès !");
    Ok(())
}