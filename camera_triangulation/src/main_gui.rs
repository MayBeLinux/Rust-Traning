use macroquad::prelude::*;
use nalgebra::{Point3, Matrix3};
use nokhwa::{Camera, utils::{RequestedFormat, RequestedFormatType, CameraIndex}, pixel_format::RgbFormat};

mod config;
mod math;

use math::TriangulationMath;

// Structure pour représenter un objet détecté
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
    focal_length: f64,
    principal_point: (f64, f64),
    camera_matrix: Matrix3<f64>,
    distortion_coeffs: Vec<f64>,
    camera_height: f64,
    tilt_angle: f64,
}

// Structure pour les résultats de triangulation
#[derive(Debug, Clone)]
struct TriangulationResult {
    object_id: u32,
    world_position: Point3<f64>,
    distance_from_camera: f64,
    estimated_size: (f64, f64, f64),
}

struct CameraTriangulationGUI {
    camera_params: CameraParameters,
    frame_count: u32,
    objects: Vec<DetectedObject>,
    camera: Option<Camera>,
    camera_texture: Option<Texture2D>,
    show_info: bool,
}

impl CameraTriangulationGUI {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        println!("🎯 Initialisation du système avec caméra...");
        
        let camera_params = CameraParameters {
            focal_length: 500.0,
            principal_point: (320.0, 240.0),
            camera_matrix: Matrix3::new(
                500.0, 0.0, 320.0,
                0.0, 500.0, 240.0,
                0.0, 0.0, 1.0
            ),
            distortion_coeffs: vec![0.0, 0.0, 0.0, 0.0, 0.0],
            camera_height: 1.5,
            tilt_angle: 0.0,
        };
        
        // Essayer d'initialiser la caméra
        let camera = match Self::init_camera().await {
            Ok(cam) => {
                println!("✅ Caméra initialisée avec succès");
                Some(cam)
            }
            Err(e) => {
                println!("⚠️ Impossible d'initialiser la caméra: {}", e);
                println!("🔄 Basculement en mode simulation");
                None
            }
        };
        
        // Objets simulés pour le mode sans caméra
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
        
        Ok(CameraTriangulationGUI {
            camera_params,
            frame_count: 0,
            objects,
            camera,
            camera_texture: None,
            show_info: true,
        })
    }
    
    async fn init_camera() -> Result<Camera, Box<dyn std::error::Error + Send + Sync>> {
        let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
        let mut camera = Camera::new(CameraIndex::Index(0), requested)?;
        camera.open_stream()?;
        Ok(camera)
    }
    
    // Simuler la détection d'objets (pour le mode sans caméra)
    fn simulate_detection(&mut self) -> Vec<DetectedObject> {
        for obj in &mut self.objects {
            let noise_x = (self.frame_count as f64 * 0.1).sin() * 5.0;
            let noise_y = (self.frame_count as f64 * 0.07).cos() * 3.0;
            
            obj.center.0 = (obj.center.0 + noise_x).clamp(50.0, 590.0);
            obj.center.1 = (obj.center.1 + noise_y).clamp(50.0, 430.0);
            
            let size_variation = (self.frame_count as f64 * 0.05).sin() * 0.1 + 1.0;
            obj.width = (obj.width * size_variation).clamp(30.0, 150.0);
            obj.height = (obj.height * size_variation).clamp(30.0, 150.0);
        }
        
        self.objects.clone()
    }
    
    // Détection simple pour mode caméra réelle
    fn detect_simple_objects(&self) -> Vec<DetectedObject> {
        // Pour le moment, on place quelques objets fixes pour tester
        // Dans une vraie implémentation, on analyserait les pixels de la texture
        vec![
            DetectedObject {
                id: 0,
                center: (320.0, 240.0), // Centre de l'image
                width: 100.0,
                height: 120.0,
                confidence: 0.8,
                object_type: "Centre".to_string(),
            },
            DetectedObject {
                id: 1,
                center: (160.0, 120.0), // Coin supérieur gauche
                width: 80.0,
                height: 100.0,
                confidence: 0.6,
                object_type: "Zone".to_string(),
            },
        ]
    }

    // Détection simple basée sur les couleurs (pour vraie caméra)
    fn detect_objects_from_camera(&self, frame_data: &[u8], width: u32, height: u32) -> Vec<DetectedObject> {
        let mut objects = Vec::new();
        
        // 1. Détection basée sur la couleur de peau
        let mut skin_pixels = Vec::new();
        
        // 2. Détection de mouvement basée sur les contrastes
        let mut bright_regions = Vec::new();
        
        // 3. Détection de contours simples
        let mut edge_pixels = Vec::new();
        
        for y in (5..height.saturating_sub(5)).step_by(8) {
            for x in (5..width.saturating_sub(5)).step_by(8) {
                let pixel_index = ((y * width + x) * 3) as usize;
                if pixel_index + 2 < frame_data.len() {
                    let r = frame_data[pixel_index] as f32;
                    let g = frame_data[pixel_index + 1] as f32;
                    let b = frame_data[pixel_index + 2] as f32;
                    
                    // Détection couleur de peau améliorée
                    if r > 95.0 && g > 40.0 && b > 20.0 && 
                       r > g && r > b && r - g > 15.0 &&
                       r + g + b > 200.0 {
                        skin_pixels.push((x as f64, y as f64));
                    }
                    
                    // Détection de zones lumineuses (objets blancs/clairs)
                    let brightness = (r + g + b) / 3.0;
                    if brightness > 180.0 && (r - g).abs() < 30.0 && (g - b).abs() < 30.0 {
                        bright_regions.push((x as f64, y as f64));
                    }
                    
                    // Détection de contours simples (changements brusques de couleur)
                    if x >= 8 && y >= 8 {
                        let prev_x_idx = ((y * width + (x.saturating_sub(8))) * 3) as usize;
                        let prev_y_idx = (((y.saturating_sub(8)) * width + x) * 3) as usize;
                        
                        if prev_x_idx + 2 < frame_data.len() && prev_y_idx + 2 < frame_data.len() {
                            let prev_r_x = frame_data[prev_x_idx] as f32;
                            let prev_r_y = frame_data[prev_y_idx] as f32;
                            
                            let diff_x = (r - prev_r_x).abs();
                            let diff_y = (r - prev_r_y).abs();
                            
                            if diff_x > 50.0 || diff_y > 50.0 {
                                edge_pixels.push((x as f64, y as f64));
                            }
                        }
                    }
                }
            }
        }
        
        // Grouper les pixels de peau en objets
        if skin_pixels.len() > 10 {
            let center_x = skin_pixels.iter().map(|(x, _)| x).sum::<f64>() / skin_pixels.len() as f64;
            let center_y = skin_pixels.iter().map(|(_, y)| y).sum::<f64>() / skin_pixels.len() as f64;
            
            // Calculer la taille approximative de la région
            let min_x = skin_pixels.iter().map(|(x, _)| *x).fold(f64::INFINITY, f64::min);
            let max_x = skin_pixels.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
            let min_y = skin_pixels.iter().map(|(_, y)| *y).fold(f64::INFINITY, f64::min);
            let max_y = skin_pixels.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);
            
            let width_obj = (max_x - min_x).max(60.0);
            let height_obj = (max_y - min_y).max(80.0);
            
            objects.push(DetectedObject {
                id: 0,
                center: (center_x, center_y),
                width: width_obj,
                height: height_obj,
                confidence: (skin_pixels.len() as f64 / 100.0).min(1.0),
                object_type: "Visage".to_string(),
            });
        }
        
        // Grouper les régions lumineuses
        if bright_regions.len() > 15 {
            let center_x = bright_regions.iter().map(|(x, _)| x).sum::<f64>() / bright_regions.len() as f64;
            let center_y = bright_regions.iter().map(|(_, y)| y).sum::<f64>() / bright_regions.len() as f64;
            
            objects.push(DetectedObject {
                id: 1,
                center: (center_x, center_y),
                width: 80.0,
                height: 60.0,
                confidence: (bright_regions.len() as f64 / 50.0).min(1.0),
                object_type: "Objet Clair".to_string(),
            });
        }
        
        // Grouper les contours en objets
        if edge_pixels.len() > 20 {
            let center_x = edge_pixels.iter().map(|(x, _)| x).sum::<f64>() / edge_pixels.len() as f64;
            let center_y = edge_pixels.iter().map(|(_, y)| y).sum::<f64>() / edge_pixels.len() as f64;
            
            objects.push(DetectedObject {
                id: 2,
                center: (center_x, center_y),
                width: 90.0,
                height: 70.0,
                confidence: (edge_pixels.len() as f64 / 100.0).min(1.0),
                object_type: "Contour".to_string(),
            });
        }
        
        objects
    }
    
    fn triangulate_object(&self, object: &DetectedObject) -> TriangulationResult {
        let distance = TriangulationMath::distance_from_apparent_size(
            object.width,
            match object.object_type.as_str() {
                "Visage" | "Détection" => 0.25,
                "Objet" => 0.15,
                _ => 0.20,
            },
            self.camera_params.focal_length,
        );
        
        let ray = TriangulationMath::pixel_to_ray(
            object.center.0,
            object.center.1,
            &self.camera_params.camera_matrix,
        );
        
        let x_world = ray.x * distance;
        let y_world = ray.y * distance;
        let z_world = distance;
        
        let adjusted_y = y_world * self.camera_params.tilt_angle.cos() 
                       + z_world * self.camera_params.tilt_angle.sin() 
                       - self.camera_params.camera_height;
        
        let adjusted_z = z_world * self.camera_params.tilt_angle.cos() 
                       - y_world * self.camera_params.tilt_angle.sin();
        
        let size_width = (object.width / self.camera_params.focal_length) * distance;
        let size_height = (object.height / self.camera_params.focal_length) * distance;
        let size_depth = match object.object_type.as_str() {
            "Visage" | "Détection" => 0.25,
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
    
    async fn update_camera_frame(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(ref mut camera) = self.camera {
            let frame = camera.frame()?;
            let frame_data = frame.decode_image::<RgbFormat>()?;
            let resolution = frame.resolution();
            
            // Effectuer la détection d'objets sur les données RGB brutes
            let detected_objects = self.detect_objects_from_camera(
                &frame_data, 
                resolution.width(), 
                resolution.height()
            );
            
            // Stocker les objets détectés pour le rendu
            self.objects = detected_objects;
            
            // Convertir RGB en RGBA pour macroquad
            let mut rgba_data = Vec::with_capacity(frame_data.len() * 4 / 3);
            for rgb_chunk in frame_data.chunks(3) {
                if rgb_chunk.len() == 3 {
                    rgba_data.push(rgb_chunk[0]); // R
                    rgba_data.push(rgb_chunk[1]); // G
                    rgba_data.push(rgb_chunk[2]); // B
                    rgba_data.push(255);          // A (alpha)
                }
            }
            
            // Créer une texture à partir des données RGBA
            let texture = Texture2D::from_rgba8(
                resolution.width() as u16,
                resolution.height() as u16,
                &rgba_data,
            );
            
            self.camera_texture = Some(texture);
        }
        Ok(())
    }
    
    async fn render(&mut self) {
        clear_background(BLACK);
        
        // Dessiner le flux de la caméra ou une simulation
        if let Some(texture) = &self.camera_texture {
            draw_texture(texture, 0.0, 0.0, WHITE);
        } else {
            // Mode simulation - dessiner un fond et des objets simulés
            draw_rectangle(0.0, 0.0, 640.0, 480.0, DARKBLUE);
            draw_text("MODE SIMULATION", 10.0, 30.0, 30.0, WHITE);
            draw_text("(Caméra non disponible)", 10.0, 60.0, 20.0, GRAY);
        }
        
        // Obtenir les objets détectés
        let detected_objects = if self.camera.is_some() && self.camera_texture.is_some() {
            // Mode caméra réelle - utiliser les objets détectés en temps réel
            self.objects.clone()
        } else {
            self.simulate_detection()
        };
        
        // Calculer la triangulation et dessiner les résultats
        let mut triangulation_results = Vec::new();
        for object in &detected_objects {
            let result = self.triangulate_object(object);
            
            // Dessiner le rectangle de détection
            let rect_color = match object.object_type.as_str() {
                "Visage" => GREEN,
                "Détection" => YELLOW,
                "Objet Clair" => BLUE,
                "Contour" => MAGENTA,
                _ => RED,
            };
            
            draw_rectangle_lines(
                (object.center.0 - object.width / 2.0) as f32,
                (object.center.1 - object.height / 2.0) as f32,
                object.width as f32,
                object.height as f32,
                3.0,
                rect_color,
            );
            
            // Dessiner un point au centre
            draw_circle(
                object.center.0 as f32,
                object.center.1 as f32,
                5.0,
                rect_color,
            );
            
            // Afficher les informations sur l'objet
            let info_text = format!(
                "{}: {:.1}m ({:.0}%)",
                object.object_type,
                result.distance_from_camera,
                object.confidence * 100.0
            );
            
            draw_text(
                &info_text,
                object.center.0 as f32 - 60.0,
                (object.center.1 - object.height / 2.0 - 15.0) as f32,
                18.0,
                WHITE,
            );
            
            triangulation_results.push(result);
        }
        
        // Panneau d'informations
        if self.show_info {
            self.draw_info_panel(&triangulation_results);
        }
        
        // Contrôles
        self.draw_controls();
        
        self.frame_count += 1;
    }
    
    fn draw_info_panel(&self, results: &[TriangulationResult]) {
        let panel_x = screen_width() - 350.0;
        let panel_y = 10.0;
        let panel_width = 340.0;
        let panel_height = 200.0;
        
        // Fond du panneau
        draw_rectangle(panel_x, panel_y, panel_width, panel_height, Color::new(0.0, 0.0, 0.0, 0.8));
        draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, 2.0, WHITE);
        
        // Titre
        draw_text("🎯 Triangulation Results", panel_x + 10.0, panel_y + 30.0, 20.0, YELLOW);
        
        let mut y_offset = 60.0;
        for result in results {
            let text = format!(
                "Objet {}: ({:.1}, {:.1}, {:.1})m - {:.1}m",
                result.object_id,
                result.world_position.x,
                result.world_position.y,
                result.world_position.z,
                result.distance_from_camera
            );
            
            draw_text(&text, panel_x + 10.0, panel_y + y_offset, 16.0, WHITE);
            y_offset += 25.0;
        }
        
        // Informations sur la frame
        draw_text(
            &format!("Frame: {}", self.frame_count),
            panel_x + 10.0,
            panel_y + panel_height - 20.0,
            16.0,
            GRAY
        );
    }
    
    fn draw_controls(&self) {
        let controls = [
            "ESC - Quitter",
            "SPACE - Toggle Info Panel",
            "R - Reset",
        ];
        
        for (i, control) in controls.iter().enumerate() {
            draw_text(
                control,
                10.0,
                screen_height() - 80.0 + (i as f32 * 20.0),
                16.0,
                LIGHTGRAY,
            );
        }
    }
    
    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.show_info = !self.show_info;
        }
        
        if is_key_pressed(KeyCode::R) {
            self.frame_count = 0;
            // Reset object positions
            self.objects = vec![
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
        }
    }
    
    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        loop {
            // Mettre à jour la frame de la caméra
            if let Err(e) = self.update_camera_frame().await {
                println!("Erreur caméra: {}", e);
            }
            
            // Gérer les entrées
            self.handle_input();
            
            // Quitter si ESC est pressé
            if is_key_pressed(KeyCode::Escape) {
                break;
            }
            
            // Rendu
            self.render().await;
            
            next_frame().await;
        }
        
        Ok(())
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "🎯 Triangulation par Caméra - Détection en Temps Réel".to_owned(),
        window_width: 1024,
        window_height: 768,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("🎯 Système de Triangulation par Caméra - Interface Graphique");
    println!("===============================================================");
    
    let mut app = CameraTriangulationGUI::new().await?;
    app.run().await?;
    
    println!("👋 Arrêt du système");
    Ok(())
}