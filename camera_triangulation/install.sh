#!/bin/bash
# Script d'installation et de build pour le système de triangulation par caméra

echo "🎯 Installation du Système de Triangulation par Caméra"
echo "====================================================="

# Couleurs pour l'affichage
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Fonction pour afficher les messages colorés
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Vérification de Rust
print_status "Vérification de l'installation Rust..."
if command -v cargo &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    print_success "Rust trouvé: $RUST_VERSION"
else
    print_error "Rust n'est pas installé. Installez-le depuis https://rustup.rs/"
    exit 1
fi

# Vérification d'OpenCV
print_status "Vérification d'OpenCV..."
if pkg-config --exists opencv4 2>/dev/null; then
    OPENCV_VERSION=$(pkg-config --modversion opencv4)
    print_success "OpenCV trouvé: version $OPENCV_VERSION"
elif pkg-config --exists opencv 2>/dev/null; then
    OPENCV_VERSION=$(pkg-config --modversion opencv)
    print_success "OpenCV trouvé: version $OPENCV_VERSION"
else
    print_warning "OpenCV non détecté via pkg-config"
    print_status "Tentative de détection manuelle..."
    
    # Vérifier les chemins communs d'OpenCV
    OPENCV_PATHS=(
        "/usr/include/opencv4"
        "/usr/local/include/opencv4"
        "/opt/homebrew/include/opencv4"
        "C:/opencv/build/include"
    )
    
    OPENCV_FOUND=false
    for path in "${OPENCV_PATHS[@]}"; do
        if [ -d "$path" ]; then
            print_success "OpenCV trouvé dans: $path"
            OPENCV_FOUND=true
            break
        fi
    done
    
    if [ "$OPENCV_FOUND" = false ]; then
        print_error "OpenCV non trouvé. Installation requise."
        echo
        echo "Instructions d'installation:"
        echo "Ubuntu/Debian: sudo apt install libopencv-dev clang libclang-dev"
        echo "macOS: brew install opencv"
        echo "Windows: Téléchargez depuis https://opencv.org/releases/"
        exit 1
    fi
fi

# Téléchargement des fichiers Haar Cascade
print_status "Téléchargement des fichiers de classification..."
HAAR_URL="https://raw.githubusercontent.com/opencv/opencv/master/data/haarcascades/haarcascade_frontalface_alt.xml"
if [ ! -f "haarcascade_frontalface_alt.xml" ]; then
    if command -v wget &> /dev/null; then
        wget -q "$HAAR_URL"
        print_success "Fichier Haar Cascade téléchargé"
    elif command -v curl &> /dev/null; then
        curl -s -o haarcascade_frontalface_alt.xml "$HAAR_URL"
        print_success "Fichier Haar Cascade téléchargé"
    else
        print_warning "wget/curl non trouvé. Téléchargement manuel requis:"
        echo "$HAAR_URL"
    fi
else
    print_success "Fichier Haar Cascade déjà présent"
fi

# Build du projet
print_status "Compilation du projet..."
if cargo build --release; then
    print_success "Compilation réussie!"
else
    print_error "Échec de la compilation"
    echo
    echo "Conseils de dépannage:"
    echo "1. Vérifiez l'installation d'OpenCV"
    echo "2. Définissez les variables d'environnement si nécessaire:"
    echo "   export OPENCV_LINK_LIBS=opencv_world4xx"
    echo "   export OPENCV_LINK_PATHS=/path/to/opencv/lib"
    echo "   export OPENCV_INCLUDE_PATHS=/path/to/opencv/include"
    echo "3. Consultez le README.md pour plus d'informations"
    exit 1
fi

# Création des configurations d'exemple
print_status "Création des configurations d'exemple..."
cargo run --bin create_configs 2>/dev/null || print_warning "Configurations d'exemple non créées"

# Test de la caméra
print_status "Test de disponibilité de la caméra..."
if [ -e /dev/video0 ] || [ -e /dev/video1 ]; then
    print_success "Caméra détectée"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    print_success "macOS détecté - caméra probablement disponible"
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
    print_success "Windows détecté - caméra probablement disponible"
else
    print_warning "Caméra non détectée automatiquement"
fi

echo
print_success "🎉 Installation terminée avec succès!"
echo
echo "Pour démarrer le système:"
echo "  cargo run --release"
echo
echo "Ou directement:"
echo "  ./target/release/camera_triangulation"
echo
echo "Contrôles:"
echo "  'q' - Quitter"
echo "  'c' - Calibrer (à implémenter)"
echo
echo "Consultez README.md pour plus d'informations."