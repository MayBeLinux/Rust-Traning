// Modules mathématiques pour la triangulation et les calculs géométriques

use nalgebra::{Vector3, Point3, Matrix3, Rotation3};

/// Calculs de triangulation avancés
pub struct TriangulationMath;

impl TriangulationMath {
    /// Triangulation par intersection de rayons (méthode classique)
    pub fn triangulate_rays(
        camera_pos: Point3<f64>,
        ray_direction: Vector3<f64>,
        ground_plane_height: f64,
    ) -> Option<Point3<f64>> {
        // Intersection du rayon avec le plan du sol
        if ray_direction.y.abs() < 1e-6 {
            return None; // Rayon parallèle au sol
        }
        
        let t = (ground_plane_height - camera_pos.y) / ray_direction.y;
        if t < 0.0 {
            return None; // Intersection derrière la caméra
        }
        
        Some(camera_pos + t * ray_direction)
    }
    
    /// Calcul de la distance par taille apparente
    pub fn distance_from_apparent_size(
        apparent_size_pixels: f64,
        real_size_meters: f64,
        focal_length_pixels: f64,
    ) -> f64 {
        (real_size_meters * focal_length_pixels) / apparent_size_pixels
    }
    
    /// Conversion coordonnées pixel vers rayon 3D
    pub fn pixel_to_ray(
        pixel_x: f64,
        pixel_y: f64,
        camera_matrix: &Matrix3<f64>,
    ) -> Vector3<f64> {
        // Coordonnées normalisées
        let fx = camera_matrix[(0, 0)];
        let fy = camera_matrix[(1, 1)];
        let cx = camera_matrix[(0, 2)];
        let cy = camera_matrix[(1, 2)];
        
        let x_norm = (pixel_x - cx) / fx;
        let y_norm = (pixel_y - cy) / fy;
        
        Vector3::new(x_norm, y_norm, 1.0).normalize()
    }
    
    /// Calcul de profondeur par stéréovision (si deux caméras)
    pub fn stereo_depth(
        left_pixel: (f64, f64),
        right_pixel: (f64, f64),
        baseline: f64,          // Distance entre les caméras
        focal_length: f64,
    ) -> Option<f64> {
        let disparity = (left_pixel.0 - right_pixel.0).abs();
        
        if disparity < 1e-6 {
            return None; // Pas de disparité détectable
        }
        
        Some((baseline * focal_length) / disparity)
    }
    
    /// Filtrage Kalman simple pour lisser les positions
    pub fn kalman_filter_position(
        previous_position: Point3<f64>,
        current_measurement: Point3<f64>,
        previous_velocity: Vector3<f64>,
        dt: f64,                // Temps écoulé
        measurement_noise: f64, // Bruit de mesure
        process_noise: f64,     // Bruit de processus
    ) -> (Point3<f64>, Vector3<f64>) {
        // Prédiction
        let predicted_position = previous_position + previous_velocity * dt;
        
        // Gain de Kalman simplifié
        let kalman_gain = process_noise / (process_noise + measurement_noise);
        
        // Correction
        let corrected_position = predicted_position + 
            kalman_gain * (current_measurement - predicted_position);
        
        // Mise à jour de la vélocité
        let new_velocity = (corrected_position - previous_position) / dt;
        
        (corrected_position, new_velocity)
    }
    
    /// Transformation de coordonnées avec rotation et translation
    pub fn transform_coordinates(
        point: Point3<f64>,
        rotation: &Rotation3<f64>,
        translation: &Vector3<f64>,
    ) -> Point3<f64> {
        rotation * point + translation
    }
    
    /// Calcul de l'erreur de triangulation
    pub fn triangulation_error(
        measured_position: Point3<f64>,
        estimated_position: Point3<f64>,
    ) -> f64 {
        (measured_position - estimated_position).norm()
    }
    
    /// Compensation de la distorsion de l'objectif
    pub fn undistort_point(
        distorted_point: (f64, f64),
        camera_matrix: &Matrix3<f64>,
        distortion_coeffs: &[f64],
    ) -> (f64, f64) {
        if distortion_coeffs.len() < 5 {
            return distorted_point; // Pas de correction si coefficients insuffisants
        }
        
        let fx = camera_matrix[(0, 0)];
        let fy = camera_matrix[(1, 1)];
        let cx = camera_matrix[(0, 2)];
        let cy = camera_matrix[(1, 2)];
        
        // Coordonnées normalisées
        let x = (distorted_point.0 - cx) / fx;
        let y = (distorted_point.1 - cy) / fy;
        
        let k1 = distortion_coeffs[0];
        let k2 = distortion_coeffs[1];
        let p1 = distortion_coeffs[2];
        let p2 = distortion_coeffs[3];
        let k3 = distortion_coeffs[4];
        
        let r2 = x * x + y * y;
        let r4 = r2 * r2;
        let r6 = r4 * r2;
        
        // Correction radiale
        let radial_correction = 1.0 + k1 * r2 + k2 * r4 + k3 * r6;
        
        // Correction tangentielle
        let x_corrected = x * radial_correction + 2.0 * p1 * x * y + p2 * (r2 + 2.0 * x * x);
        let y_corrected = y * radial_correction + p1 * (r2 + 2.0 * y * y) + 2.0 * p2 * x * y;
        
        // Retour aux coordonnées pixel
        (
            x_corrected * fx + cx,
            y_corrected * fy + cy,
        )
    }
    
    /// Calcul de la matrice de homographie pour la rectification
    pub fn compute_homography(
        source_points: &[(f64, f64)],
        target_points: &[(f64, f64)],
    ) -> Option<Matrix3<f64>> {
        if source_points.len() != 4 || target_points.len() != 4 {
            return None; // Besoin de 4 points pour calculer l'homographie
        }
        
        // Implémentation simplifiée - dans un vrai projet, utilisez OpenCV
        // ou une bibliothèque spécialisée pour le calcul robuste
        
        // Pour l'instant, retourne une matrice identité
        Some(Matrix3::identity())
    }
    
    /// Estimation de la qualité de la triangulation
    pub fn triangulation_quality(
        object_size_pixels: f64,
        image_center: (f64, f64),
        object_center: (f64, f64),
        distance_estimate: f64,
    ) -> f64 {
        // Facteurs influençant la qualité :
        
        // 1. Taille de l'objet (plus grand = plus précis)
        let size_factor = (object_size_pixels / 100.0).min(1.0);
        
        // 2. Distance du centre de l'image (plus proche du centre = plus précis)
        let center_distance = ((object_center.0 - image_center.0).powi(2) + 
                              (object_center.1 - image_center.1).powi(2)).sqrt();
        let center_factor = 1.0 / (1.0 + center_distance / 100.0);
        
        // 3. Distance estimée (plus proche = plus précis)
        let distance_factor = 1.0 / (1.0 + distance_estimate / 5.0);
        
        // Score combiné (0-1)
        (size_factor * center_factor * distance_factor).clamp(0.0, 1.0)
    }
}

/// Outils de calibrage de caméra
pub struct CameraCalibration;

impl CameraCalibration {
    /// Estimation grossière de la distance focale à partir de la résolution
    pub fn estimate_focal_length(image_width: u32, fov_degrees: f64) -> f64 {
        let fov_radians = fov_degrees.to_radians();
        (image_width as f64 / 2.0) / (fov_radians / 2.0).tan()
    }
    
    /// Validation des paramètres de caméra
    pub fn validate_camera_params(
        focal_length: f64,
        principal_point: (f64, f64),
        image_size: (u32, u32),
    ) -> bool {
        // Vérifications de base
        focal_length > 0.0 &&
        principal_point.0 >= 0.0 && principal_point.0 <= image_size.0 as f64 &&
        principal_point.1 >= 0.0 && principal_point.1 <= image_size.1 as f64
    }
    
    /// Calcul du champ de vision à partir de la distance focale
    pub fn calculate_fov(focal_length: f64, sensor_size: f64) -> f64 {
        2.0 * (sensor_size / (2.0 * focal_length)).atan().to_degrees()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_distance_calculation() {
        let distance = TriangulationMath::distance_from_apparent_size(
            100.0, // 100 pixels
            0.25,  // 25cm de large
            500.0, // Distance focale
        );
        
        assert!((distance - 1.25).abs() < 0.01); // Environ 1.25m
    }
    
    #[test]
    fn test_pixel_to_ray() {
        let camera_matrix = Matrix3::new(
            500.0, 0.0, 320.0,
            0.0, 500.0, 240.0,
            0.0, 0.0, 1.0,
        );
        
        let ray = TriangulationMath::pixel_to_ray(320.0, 240.0, &camera_matrix);
        
        // Le centre de l'image devrait donner un rayon vers l'avant (0, 0, 1)
        assert!((ray.z - 1.0).abs() < 0.01);
        assert!(ray.x.abs() < 0.01);
        assert!(ray.y.abs() < 0.01);
    }
    
    #[test]
    fn test_triangulation_quality() {
        let quality = TriangulationMath::triangulation_quality(
            50.0,           // Taille objet
            (320.0, 240.0), // Centre image
            (320.0, 240.0), // Centre objet (même que centre image)
            2.0,            // Distance 2m
        );
        
        assert!(quality > 0.0 && quality <= 1.0);
    }
}