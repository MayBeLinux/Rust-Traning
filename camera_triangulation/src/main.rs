use nalgebra::{Point3, Matrix3};
use std::io;

mod config;
mod math;

use config::{CameraConfig, TriangulationConfig};
use math::TriangulationMath;

// Structure pour représenter un objet détecté (simulation)
#[derive(Debug, Clone)]
struct DetectedObject {
    id: u32,
    center: (f64, f64),    // Centre en pixels (x, y)
    width: f64,            // Largeur en pixels
    height: f64,           // Hauteur en pixels
    confidence: f64,       // Confiance de la détection (0-1)
    object_type: String,   // Type d'objet détecté
}

// Structure pour les paramètres de la caméra
#[derive(Debug, Clone)]
struct CameraParameters {
    focal_length: f64,        // Distance focale en pixels
    principal_point: (f64, f64), // Point principal (cx, cy)
    camera_matrix: Matrix3<f64>, // Matrice intrinsèque de la caméra
    distortion_coeffs: Vec<f64>, // Coefficients de distorsion
    camera_height: f64,       // Hauteur de la caméra par rapport au sol (en mètres)
    tilt_angle: f64,          // Angle d'inclinaison de la caméra (en radians)
}

// Structure pour les résultats de triangulation
#[derive(Debug, Clone)]
struct TriangulationResult {
    object_id: u32,
    world_position: Point3<f64>, // Position dans l'espace 3D (x, y, z)
    distance_from_camera: f64,   // Distance de la caméra en mètres
    estimated_size: (f64, f64, f64), // Taille estimée (largeur, hauteur, profondeur) en mètres
}

struct CameraTriangulationSimulator {
    camera_params: CameraParameters,
    frame_count: u32,
    objects: Vec<DetectedObject>,
}

impl CameraTriangulationSimulator {
    // Initialisation du simulateur de triangulation
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        println!("🎯 Initialisation du simulateur de triangulation...");
        
        // Paramètres de caméra par défaut (à calibrer pour votre caméra spécifique)
        let camera_params = CameraParameters {
            focal_length: 500.0, // À ajuster selon votre caméra
            principal_point: (320.0, 240.0), // Centre de l'image pour 640x480
            camera_matrix: Matrix3::new(
                500.0, 0.0, 320.0,
                0.0, 500.0, 240.0,
                0.0, 0.0, 1.0
            ),
            distortion_coeffs: vec![0.0, 0.0, 0.0, 0.0, 0.0], // Pas de distorsion par défaut
            camera_height: 1.5, // Caméra à 1.5m de hauteur
            tilt_angle: 0.0,    // Caméra horizontale
        };
        
        // Objets simulés
        let objects = vec![
            DetectedObject {
                id: 0,
                center: (200.0, 180.0),
                width: 80.0,
                height: 100.0,
                confidence: 0.85,
                object_type: "Visage".to_string(),
            },
            DetectedObject {
                id: 1,
                center: (450.0, 200.0),
                width: 60.0,
                height: 80.0,
                confidence: 0.75,
                object_type: "Visage".to_string(),
            },
            DetectedObject {
                id: 2,
                center: (320.0, 350.0),
                width: 120.0,
                height: 40.0,
                confidence: 0.60,
                object_type: "Objet".to_string(),
            },
        ];
        
        Ok(CameraTriangulationSimulator {
            camera_params,
            frame_count: 0,
            objects,
        })
    }
    
    // Simuler la détection d'objets (sans vraie caméra)
    fn simulate_detection(&mut self) -> Vec<DetectedObject> {
        // Faire bouger légèrement les objets pour simuler le mouvement
        for obj in &mut self.objects {
            // Petit mouvement aléatoire
            let noise_x = (self.frame_count as f64 * 0.1).sin() * 5.0;
            let noise_y = (self.frame_count as f64 * 0.07).cos() * 3.0;
            
            obj.center.0 += noise_x;
            obj.center.1 += noise_y;
            
            // Variation légère de taille
            let size_variation = (self.frame_count as f64 * 0.05).sin() * 0.1 + 1.0;
            obj.width *= size_variation;
            obj.height *= size_variation;
        }
        
        self.objects.clone()
    }
    
    // Calculer la triangulation pour un objet détecté
    fn triangulate_object(&self, object: &DetectedObject) -> TriangulationResult {
        // Utiliser les méthodes du module math
        let distance = TriangulationMath::distance_from_apparent_size(
            object.width,
            match object.object_type.as_str() {
                "Visage" => 0.25, // 25cm de large
                "Objet" => 0.15,  // 15cm de large
                _ => 0.20,
            },
            self.camera_params.focal_length,
        );
        
        // Convertir pixel vers rayon 3D
        let ray = TriangulationMath::pixel_to_ray(
            object.center.0,
            object.center.1,
            &self.camera_params.camera_matrix,
        );
        
        // Position 3D dans le repère de la caméra
        let x_world = ray.x * distance;
        let y_world = ray.y * distance;
        let z_world = distance;
        
        // Ajustement pour la hauteur et l'inclinaison de la caméra
        let adjusted_y = y_world * self.camera_params.tilt_angle.cos() 
                       + z_world * self.camera_params.tilt_angle.sin() 
                       - self.camera_params.camera_height;
        
        let adjusted_z = z_world * self.camera_params.tilt_angle.cos() 
                       - y_world * self.camera_params.tilt_angle.sin();
        
        // Estimation de la taille 3D de l'objet
        let size_width = (object.width / self.camera_params.focal_length) * distance;
        let size_height = (object.height / self.camera_params.focal_length) * distance;
        let size_depth = match object.object_type.as_str() {
            "Visage" => 0.25,
            "Objet" => 0.15,
            _ => 0.20,
        };
        
        TriangulationResult {
            object_id: object.id,
            world_position: Point3::new(x_world, adjusted_y, adjusted_z),
            distance_from_camera: distance,
            estimated_size: (size_width, size_height, size_depth),
        }
    }
    
    // Afficher les informations de triangulation
    fn display_results(&self, results: &[TriangulationResult]) {
        println!("\n📊 Résultats de triangulation (Frame {}):", self.frame_count);
        println!("┌─────────┬──────────────────────────────┬────────────┬─────────────────────────┐");
        println!("│ Objet   │ Position 3D (x, y, z)       │ Distance   │ Qualité                 │");
        println!("├─────────┼──────────────────────────────┼────────────┼─────────────────────────┤");
        
        for result in results {
            let object = &self.objects[result.object_id as usize];
            let quality = TriangulationMath::triangulation_quality(
                object.width,
                self.camera_params.principal_point,
                object.center,
                result.distance_from_camera,
            );
            
            println!(
                "│ {:7} │ ({:5.2}, {:5.2}, {:5.2})m     │ {:8.2}m │ {:5.1}% ({:>8})     │",
                result.object_id,
                result.world_position.x,
                result.world_position.y,
                result.world_position.z,
                result.distance_from_camera,
                quality * 100.0,
                if quality > 0.7 { "Excellent" } 
                else if quality > 0.5 { "Bon" } 
                else { "Moyen" }
            );
        }
        println!("└─────────┴──────────────────────────────┴────────────┴─────────────────────────┘");
    }
    
    // Boucle principale de simulation
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Démarrage de la simulation de triangulation...");
        println!("Appuyez sur 'Enter' pour continuer, 'q' + Enter pour quitter");
        
        loop {
            self.frame_count += 1;
            
            // Simuler la détection d'objets
            let detected_objects = self.simulate_detection();
            
            // Calculer la triangulation pour chaque objet
            let mut triangulation_results = Vec::new();
            for object in &detected_objects {
                let result = self.triangulate_object(object);
                triangulation_results.push(result);
            }
            
            // Afficher les résultats
            self.display_results(&triangulation_results);
            
            // Afficher les objets détectés
            println!("\n🎯 Objets détectés:");
            for object in &detected_objects {
                println!(
                    "  {} - Position: ({:.0}, {:.0})px, Taille: {:.0}x{:.0}px, Confiance: {:.1}%",
                    object.object_type,
                    object.center.0,
                    object.center.1,
                    object.width,
                    object.height,
                    object.confidence * 100.0
                );
            }
            
            println!("\n💡 Conseils d'interprétation:");
            println!("  - Distance négative = objet derrière la caméra");
            println!("  - Qualité élevée = triangulation plus fiable");
            println!("  - Position Y négative = en dessous de la caméra");
            
            // Attendre l'entrée utilisateur
            println!("\nAppuyez sur Enter pour la frame suivante, ou 'q' + Enter pour quitter...");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            if input.trim().to_lowercase() == "q" {
                break;
            }
            
            // Effacer l'écran (compatible Windows/Linux)
            if cfg!(windows) {
                std::process::Command::new("cmd")
                    .args(&["/c", "cls"])
                    .status()
                    .unwrap_or_default();
            } else {
                print!("\x1b[2J\x1b[1;1H");
            }
        }
        
        println!("👋 Arrêt du simulateur de triangulation");
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialiser le logger
    env_logger::init();
    
    println!("🎯 Simulateur de Triangulation par Caméra");
    println!("=========================================");
    println!("Version sans OpenCV - Données simulées");
    println!();
    
    // Créer et lancer le simulateur
    match CameraTriangulationSimulator::new() {
        Ok(mut simulator) => {
            simulator.run()?;
        }
        Err(e) => {
            println!("❌ Erreur d'initialisation: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}