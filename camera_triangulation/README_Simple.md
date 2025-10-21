# 🎯 Simulateur de Triangulation par Caméra

**Version simplifiée sans OpenCV - Prêt à l'emploi sur Windows !**

## ✅ Ce qui fonctionne maintenant

- ✅ **Compilation réussie** sans dépendances complexes
- ✅ **Simulation de détection d'objets** avec données réalistes  
- ✅ **Calculs de triangulation** mathématiquement corrects
- ✅ **Affichage interactif** en console avec tableaux formatés
- ✅ **Mouvement simulé** des objets détectés

## 🚀 Comment utiliser

```powershell
# Dans le dossier camera_triangulation
cargo run
```

### Interface utilisateur :
- **Enter** : Frame suivante
- **q + Enter** : Quitter le programme

## 📊 Exemple de sortie

```
📊 Résultats de triangulation (Frame 1):
┌─────────┬──────────────────────────────┬────────────┬─────────────────────────┐
│ Objet   │ Position 3D (x, y, z)       │ Distance   │ Qualité                 │
├─────────┼──────────────────────────────┼────────────┼─────────────────────────┤
│       0 │ (-0.75, -1.93,  1.56)m      │     1.56m │  68.2% (     Bon)     │
│       1 │ ( 1.04, -1.81,  1.87)m      │     1.87m │  59.4% (     Bon)     │
│       2 │ ( 0.00, -0.71,  0.63)m      │     0.63m │  91.3% (Excellent)     │
└─────────┴──────────────────────────────┴────────────┴─────────────────────────┘

🎯 Objets détectés:
  Visage - Position: (200, 180)px, Taille: 80x100px, Confiance: 85.0%
  Visage - Position: (450, 200)px, Taille: 60x80px, Confiance: 75.0%
  Objet - Position: (320, 350)px, Taille: 120x40px, Confiance: 60.0%
```

## 🧮 Fonctionnalités techniques

### Détection simulée
- 3 objets avec mouvement réaliste
- Variation de taille et position
- Types d'objets : Visages et objets génériques

### Calculs de triangulation
- **Distance par taille apparente** : `distance = (taille_réelle × focale) / taille_pixels`
- **Position 3D** : Conversion pixel → rayon 3D → coordonnées monde
- **Qualité de triangulation** : Score basé sur taille, position et distance

### Mathématiques avancées
- Matrice de caméra intrinsèque
- Correction de hauteur et inclinaison
- Estimation de taille 3D des objets

## 🎯 Concepts démontrés

1. **Vision par ordinateur** sans OpenCV
2. **Géométrie 3D** avec nalgebra
3. **Triangulation monoculaire** par taille apparente
4. **Interface utilisateur** interactive en console
5. **Simulation réaliste** de données de caméra

## ⚙️ Configuration

Modifiez les paramètres dans `main.rs` :

```rust
let camera_params = CameraParameters {
    focal_length: 500.0,           // Distance focale (pixels)
    principal_point: (320.0, 240.0), // Centre image
    camera_height: 1.5,            // Hauteur caméra (mètres)
    tilt_angle: 0.0,              // Inclinaison (radians)
    // ...
};
```

## 🔄 Extensions possibles

Pour transformer ce simulateur en système réel :

1. **Ajouter OpenCV** pour vraie détection de caméra
2. **Intégrer YOLO** pour détection d'objets avancée
3. **Stéréovision** avec deux caméras
4. **Interface graphique** avec egui ou tauri
5. **Sauvegarde des données** en JSON
6. **Calibrage automatique** de caméra

## 🚨 Dépannage

### Le programme ne compile pas
```powershell
cargo clean
cargo build
```

### Les calculs semblent incorrects
- Vérifiez les paramètres de caméra
- Ajustez les tailles réelles des objets
- Modifiez la distance focale

## 📚 Apprentissage

Ce projet démontre :
- **Ownership Rust** avec structures complexes
- **Mathématiques 3D** appliquées
- **Modules et organisation** du code
- **Gestion d'erreurs** avec `Result<T, E>`
- **Simulation vs réalité** en vision par ordinateur

**Félicitations ! Vous avez un système de triangulation fonctionnel ! 🎉**