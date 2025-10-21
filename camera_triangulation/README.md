# 🎯 Système de Triangulation par Caméra en Rust

Ce projet utilise la vision par ordinateur pour détecter des objets via votre caméra et calculer leur position 3D dans l'espace en utilisant des techniques de triangulation.

## 🚀 Fonctionnalités

- **Détection d'objets en temps réel** via votre webcam
- **Calcul de triangulation** pour estimer la position 3D des objets
- **Estimation de distance** basée sur la taille apparente des objets
- **Interface visuelle** avec OpenCV
- **Calculs géométriques** avec nalgebra

## 📋 Prérequis

### Windows
```powershell
# Installer OpenCV
# Télécharger depuis: https://opencv.org/releases/
# Ou utiliser vcpkg:
vcpkg install opencv4[contrib,nonfree]

# Variables d'environnement à définir:
# OPENCV_LINK_LIBS=opencv_world4xx
# OPENCV_LINK_PATHS=C:\path\to\opencv\build\x64\vc16\lib
# OPENCV_INCLUDE_PATHS=C:\path\to\opencv\build\include
```

### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install libopencv-dev clang libclang-dev
```

### macOS
```bash
brew install opencv
```

## 🔧 Installation

1. **Cloner et compiler:**
```bash
cd camera_triangulation
cargo build --release
```

2. **Télécharger les fichiers de classification Haar (optionnel):**
```bash
# Pour la détection de visages
wget https://raw.githubusercontent.com/opencv/opencv/master/data/haarcascades/haarcascade_frontalface_alt.xml
```

## 🎮 Utilisation

```bash
cargo run
```

### Contrôles:
- **'q'** : Quitter l'application
- **'c'** : Calibrer la caméra (à implémenter)

## 🧮 Comment fonctionne la triangulation ?

### 1. **Détection d'objets:**
- Utilise les cascades Haar pour détecter les visages
- Détecte les contours pour d'autres objets
- Calcule le centre et la taille de chaque objet

### 2. **Conversion pixel → monde:**
```rust
// Coordonnées normalisées
let x_norm = (u - cx) / focal_length;
let y_norm = (v - cy) / focal_length;

// Position 3D
let x_world = x_norm * distance;
let y_world = y_norm * distance;
let z_world = distance;
```

### 3. **Estimation de distance:**
```rust
// Basée sur la taille apparente vs taille réelle
let distance = (taille_reelle * focal_length) / taille_pixels;
```

## ⚙️ Configuration de la caméra

Modifiez les paramètres dans `CameraParameters`:

```rust
let camera_params = CameraParameters {
    focal_length: 500.0,           // À calibrer pour votre caméra
    principal_point: (320.0, 240.0), // Centre de l'image
    camera_height: 1.5,            // Hauteur de montage (mètres)
    tilt_angle: 0.0,              // Inclinaison (radians)
    // ...
};
```

## 🎯 Calibrage de caméra

Pour une précision optimale, calibrez votre caméra:

1. **Utilisez un échiquier de calibrage**
2. **Calculez la matrice intrinsèque**
3. **Mesurez la hauteur de montage**
4. **Ajustez les paramètres dans le code**

## 📊 Sortie

Le système affiche:
- **Position 3D** de chaque objet (x, y, z en mètres)
- **Distance** de la caméra
- **Taille estimée** de l'objet
- **Visualisation temps réel** avec rectangles de détection

## 🔍 Exemples de résultats

```
📊 Résultats de triangulation (Frame 30):
  Objet 0: Position 3D = (0.15, -0.20, 2.50)m, Distance = 2.50m
  Objet 1: Position 3D = (-0.30, 0.10, 1.80)m, Distance = 1.80m
```

## 🚨 Dépannage

### Erreur "Impossible d'ouvrir la caméra"
- Vérifiez qu'une caméra est connectée
- Fermez les autres applications utilisant la caméra
- Essayez de changer l'index de caméra (0, 1, 2...)

### Erreur de compilation OpenCV
- Vérifiez l'installation d'OpenCV
- Définissez les variables d'environnement
- Utilisez `pkg-config --libs opencv4` sur Linux

### Détection imprécise
- Calibrez votre caméra
- Ajustez les paramètres de détection
- Améliorez l'éclairage
- Modifiez les seuils de détection

## 🔄 Améliorations possibles

- [ ] Calibrage automatique de caméra
- [ ] Détection d'objets par deep learning (YOLO)
- [ ] Tracking d'objets entre frames
- [ ] Filtrage Kalman pour lisser les positions
- [ ] Sauvegarde des données de triangulation
- [ ] Interface graphique avancée
- [ ] Support multi-caméras (stéréovision)
- [ ] Détection de profondeur par stéréovision

## 📚 Ressources

- [Documentation OpenCV-Rust](https://docs.rs/opencv/)
- [Tutoriel calibrage caméra](https://docs.opencv.org/4.x/dc/dbb/tutorial_py_calibration.html)
- [Triangulation en vision par ordinateur](https://en.wikipedia.org/wiki/Triangulation_(computer_vision))