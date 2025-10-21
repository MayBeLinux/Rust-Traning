# Script d'installation PowerShell pour Windows
# Installation du Système de Triangulation par Caméra

Write-Host "🎯 Installation du Système de Triangulation par Caméra" -ForegroundColor Blue
Write-Host "=====================================================" -ForegroundColor Blue

# Fonction pour afficher les messages colorés
function Write-Status {
    param($Message)
    Write-Host "[INFO] $Message" -ForegroundColor Cyan
}

function Write-Success {
    param($Message)
    Write-Host "[SUCCESS] $Message" -ForegroundColor Green
}

function Write-Warning {
    param($Message)
    Write-Host "[WARNING] $Message" -ForegroundColor Yellow
}

function Write-Error-Custom {
    param($Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

# Vérification de Rust
Write-Status "Vérification de l'installation Rust..."
try {
    $rustVersion = & cargo --version 2>$null
    Write-Success "Rust trouvé: $rustVersion"
} catch {
    Write-Error-Custom "Rust n'est pas installé. Installez-le depuis https://rustup.rs/"
    exit 1
}

# Vérification d'OpenCV
Write-Status "Vérification d'OpenCV..."
$opencvPaths = @(
    "C:\opencv\build\include",
    "C:\vcpkg\installed\x64-windows\include\opencv2",
    "C:\tools\opencv\build\include",
    "$env:ProgramFiles\OpenCV\build\include"
)

$opencvFound = $false
foreach ($path in $opencvPaths) {
    if (Test-Path $path) {
        Write-Success "OpenCV trouvé dans: $path"
        $opencvFound = $true
        break
    }
}

if (-not $opencvFound) {
    Write-Warning "OpenCV non trouvé dans les emplacements standard"
    Write-Host ""
    Write-Host "Instructions d'installation pour Windows:" -ForegroundColor Yellow
    Write-Host "1. Téléchargez OpenCV depuis https://opencv.org/releases/" -ForegroundColor Yellow
    Write-Host "2. Ou utilisez vcpkg: vcpkg install opencv4[contrib,nonfree]" -ForegroundColor Yellow
    Write-Host "3. Définissez les variables d'environnement:" -ForegroundColor Yellow
    Write-Host "   OPENCV_LINK_LIBS=opencv_world4xx" -ForegroundColor Yellow
    Write-Host "   OPENCV_LINK_PATHS=C:\path\to\opencv\build\x64\vc16\lib" -ForegroundColor Yellow
    Write-Host "   OPENCV_INCLUDE_PATHS=C:\path\to\opencv\build\include" -ForegroundColor Yellow
    
    $continue = Read-Host "Continuer malgré tout? (y/N)"
    if ($continue -ne "y" -and $continue -ne "Y") {
        exit 1
    }
}

# Téléchargement des fichiers Haar Cascade
Write-Status "Téléchargement des fichiers de classification..."
$haarUrl = "https://raw.githubusercontent.com/opencv/opencv/master/data/haarcascades/haarcascade_frontalface_alt.xml"
$haarFile = "haarcascade_frontalface_alt.xml"

if (-not (Test-Path $haarFile)) {
    try {
        Invoke-WebRequest -Uri $haarUrl -OutFile $haarFile -UseBasicParsing
        Write-Success "Fichier Haar Cascade téléchargé"
    } catch {
        Write-Warning "Échec du téléchargement automatique"
        Write-Host "Téléchargez manuellement depuis: $haarUrl" -ForegroundColor Yellow
    }
} else {
    Write-Success "Fichier Haar Cascade déjà présent"
}

# Build du projet
Write-Status "Compilation du projet..."
try {
    $buildOutput = & cargo build --release 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Success "Compilation réussie!"
    } else {
        throw "Échec de compilation"
    }
} catch {
    Write-Error-Custom "Échec de la compilation"
    Write-Host ""
    Write-Host "Conseils de dépannage:" -ForegroundColor Yellow
    Write-Host "1. Vérifiez l'installation d'OpenCV" -ForegroundColor Yellow
    Write-Host "2. Définissez les variables d'environnement OpenCV" -ForegroundColor Yellow
    Write-Host "3. Consultez le README.md pour plus d'informations" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Sortie de compilation:" -ForegroundColor Yellow
    Write-Host $buildOutput -ForegroundColor Gray
    exit 1
}

# Test de la caméra
Write-Status "Test de disponibilité de la caméra..."
try {
    # Tentative de détection des périphériques de capture vidéo
    $cameras = Get-WmiObject -Class Win32_PnPEntity | Where-Object { $_.Name -like "*camera*" -or $_.Name -like "*webcam*" }
    if ($cameras.Count -gt 0) {
        Write-Success "Caméra(s) détectée(s): $($cameras.Count)"
        foreach ($camera in $cameras) {
            Write-Host "  - $($camera.Name)" -ForegroundColor Gray
        }
    } else {
        Write-Warning "Aucune caméra détectée automatiquement"
    }
} catch {
    Write-Warning "Impossible de détecter les caméras automatiquement"
}

Write-Host ""
Write-Success "🎉 Installation terminée!"
Write-Host ""
Write-Host "Pour démarrer le système:" -ForegroundColor Green
Write-Host "  cargo run --release" -ForegroundColor White
Write-Host ""
Write-Host "Ou directement:" -ForegroundColor Green
Write-Host "  .\target\release\camera_triangulation.exe" -ForegroundColor White
Write-Host ""
Write-Host "Contrôles:" -ForegroundColor Green
Write-Host "  'q' - Quitter" -ForegroundColor White
Write-Host "  'c' - Calibrer (à implémenter)" -ForegroundColor White
Write-Host ""
Write-Host "Consultez README.md pour plus d'informations." -ForegroundColor Green

# Demander si l'utilisateur veut démarrer maintenant
Write-Host ""
$runNow = Read-Host "Voulez-vous démarrer le système maintenant? (y/N)"
if ($runNow -eq "y" -or $runNow -eq "Y") {
    Write-Host ""
    Write-Status "Démarrage du système de triangulation..."
    & cargo run --release
}