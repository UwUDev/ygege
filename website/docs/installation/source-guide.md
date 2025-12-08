---
sidebar_position: 3
---

# Compiler Ygégé depuis les sources

Ce guide explique comment compiler Ygégé depuis les sources sur différentes plateformes.

## Prérequis

### Toutes les plateformes
- [Rust](https://rustup.rs/) (toolchain stable)
- Git

### Dépendances spécifiques par plateforme

#### Linux (Debian/Ubuntu)
```bash
sudo apt-get update
sudo apt-get install -y build-essential cmake perl pkg-config libclang-dev
```

#### Windows
Installez les éléments suivants via [Chocolatey](https://chocolatey.org/) :
```powershell
choco install cmake strawberryperl pkgconfiglite llvm nasm -y
```

Ou téléchargez manuellement :
- [CMake](https://cmake.org/download/)
- [Perl](https://strawberryperl.com/)
- [LLVM](https://releases.llvm.org/)
- [NASM](https://www.nasm.us/)

#### macOS
```bash
brew install cmake pkg-config llvm
```

## Compilation

### Compilation standard

```bash
# Cloner le dépôt
git clone https://github.com/UwUDev/ygege.git
cd ygege

# Compiler en mode release
cargo build --release

# Le binaire sera dans : target/release/ygege (ou ygege.exe sur Windows)
```

### Avec informations de build

Pour inclure les informations de commit, date et branche :

```bash
# Linux/macOS
BUILD_COMMIT=$(git rev-parse HEAD) \
BUILD_DATE=$(git log -1 --pretty=%ct) \
BUILD_BRANCH=$(git rev-parse --abbrev-ref HEAD) \
cargo build --release

# Windows (PowerShell)
$env:BUILD_COMMIT = git rev-parse HEAD
$env:BUILD_DATE = git log -1 --pretty=%ct
$env:BUILD_BRANCH = git rev-parse --abbrev-ref HEAD
cargo build --release
```

### Cross-compilation (Linux uniquement)

#### Pour ARM64 (aarch64)
```bash
# Installer les outils de cross-compilation
sudo apt-get install -y crossbuild-essential-arm64

# Ajouter la cible Rust
rustup target add aarch64-unknown-linux-gnu

# Compiler
export CC=aarch64-linux-gnu-gcc
export CXX=aarch64-linux-gnu-g++
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
export CFLAGS_aarch64_unknown_linux_gnu=-D__ARM_ARCH=8
cargo build --release --target=aarch64-unknown-linux-gnu
```

#### Pour ARMv7
```bash
# Installer les outils de cross-compilation
sudo apt-get install -y crossbuild-essential-armhf

# Ajouter la cible Rust
rustup target add armv7-unknown-linux-gnueabihf

# Compiler
export CC=arm-linux-gnueabihf-gcc
export CXX=arm-linux-gnueabihf-g++
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc
cargo build --release --target=armv7-unknown-linux-gnueabihf
```

#### Pour i686 (32-bit)
```bash
# Installer les outils de cross-compilation
sudo apt-get install -y crossbuild-essential-i386

# Ajouter la cible Rust
rustup target add i686-unknown-linux-gnu

# Compiler
export CC=i686-linux-gnu-gcc
export CXX=i686-linux-gnu-g++
export CARGO_TARGET_I686_UNKNOWN_LINUX_GNU_LINKER=i686-linux-gnu-gcc
cargo build --release --target=i686-unknown-linux-gnu
```

## Optimisation de la taille du binaire

### Utilisation de la compression UPX

```bash
# Installer UPX
# Linux (Debian/Ubuntu)
sudo apt-get install upx-ucl

# macOS
brew install upx

# Windows
choco install upx

# Compresser le binaire
upx --best --lzma target/release/ygege
```

### Linkage statique (Windows)

Pour des exécutables Windows autonomes sans dépendances runtime :

```powershell
$env:RUSTFLAGS = "-C target-feature=+crt-static"
cargo build --release
```

## Build Docker

### Build local

```bash
# Build pour la plateforme actuelle uniquement (plus rapide)
docker build -f docker/Dockerfile -t ygege:local .

# Build multi-plateforme (nécessite Docker Buildx)
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -f docker/Dockerfile \
  -t ygege:latest \
  --load \
  .
```

### Avec arguments de build

```bash
docker build \
  -f docker/Dockerfile \
  --build-arg BUILD_COMMIT=$(git rev-parse HEAD) \
  --build-arg BUILD_DATE=$(date -u +%s) \
  --build-arg BUILD_BRANCH=$(git branch --show-current) \
  -t ygege:latest \
  .
```

## Exécution du binaire

Après la compilation, vous pouvez exécuter Ygégé :

```bash
# Afficher la version
./target/release/ygege --version

# Exécuter avec les paramètres par défaut (voir README.md pour la configuration)
./target/release/ygege
```

## Dépannage

### Problèmes courants

#### **Dépendances manquantes lors de la compilation**
Assurez-vous que tous les prérequis spécifiques à votre plateforme sont installés. Les messages d'erreur indiquent généralement quelle bibliothèque manque.

#### **Échecs de cross-compilation avec boring-sys2**
La dépendance BoringSSL peut être difficile à cross-compiler. Si vous rencontrez des problèmes :
- Vérifiez que toutes les toolchains de cross-compilation sont correctement installées
- Vérifiez que les variables d'environnement sont correctement définies
- Envisagez d'utiliser Docker pour les builds multi-plateformes à la place

#### **Erreurs de mémoire insuffisante**
La compilation Rust peut être gourmande en mémoire. Essayez :
```bash
# Réduire les jobs parallèles
cargo build --release -j 2

# Ou compiler par morceaux en désactivant la compilation incrémentale
export CARGO_INCREMENTAL=0
cargo build --release
```

#### **Windows: link.exe introuvable**
Installez [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022) avec la charge de travail C++.

## Conseils de performance de build

1. **Activer la compilation incrémentale** (builds de développement) :
   ```bash
   export CARGO_INCREMENTAL=1
   ```

2. **Utiliser `sccache` ou `cargo-cache`** pour la mise en cache :
   ```bash
   cargo install sccache
   export RUSTC_WRAPPER=sccache
   ```

3. **Builds parallèles** : Cargo compile en parallèle par défaut. Ajustez avec :
   ```bash
   cargo build -j <nombre_de_cœurs>
   ```

## Builds CI/CD

Des builds officiels sont automatiquement créés par GitHub Actions pour chaque release :
- Linux : x86_64, i686, aarch64, armv7
- Windows : x86_64, i686
- macOS : x86_64 (Intel), aarch64 (Apple Silicon)
- Docker : linux/amd64, linux/arm64

Consultez la page [Releases](https://github.com/UwUDev/ygege/releases) pour les binaires pré-compilés.
