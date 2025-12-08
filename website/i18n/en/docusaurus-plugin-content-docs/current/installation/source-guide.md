---
sidebar_position: 3
---

# Building Ygégé from Source

This guide explains how to build Ygégé from source on different platforms.

## Prerequisites

### All Platforms
- [Rust](https://rustup.rs/) (stable toolchain)
- Git

### Platform-Specific Requirements

#### Linux (Debian/Ubuntu)
```bash
sudo apt-get update
sudo apt-get install -y build-essential cmake perl pkg-config libclang-dev
```

#### Windows
Install the following via [Chocolatey](https://chocolatey.org/):
```powershell
choco install cmake strawberryperl pkgconfiglite llvm nasm -y
```

Or download manually:
- [CMake](https://cmake.org/download/)
- [Perl](https://strawberryperl.com/)
- [LLVM](https://releases.llvm.org/)
- [NASM](https://www.nasm.us/)

#### macOS
```bash
brew install cmake pkg-config llvm
```

## Building

### Standard Build

```bash
# Clone the repository
git clone https://github.com/UwUDev/ygege.git
cd ygege

# Build in release mode
cargo build --release

# Binary will be at: target/release/ygege (or ygege.exe on Windows)
```

### With Build Information

To include commit, date, and branch information:

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

### Cross-Compilation (Linux only)

#### For ARM64 (aarch64)
```bash
# Install cross-compilation tools
sudo apt-get install -y crossbuild-essential-arm64

# Add Rust target
rustup target add aarch64-unknown-linux-gnu

# Build
export CC=aarch64-linux-gnu-gcc
export CXX=aarch64-linux-gnu-g++
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
export CFLAGS_aarch64_unknown_linux_gnu=-D__ARM_ARCH=8
cargo build --release --target=aarch64-unknown-linux-gnu
```

#### For ARMv7
```bash
# Install cross-compilation tools
sudo apt-get install -y crossbuild-essential-armhf

# Add Rust target
rustup target add armv7-unknown-linux-gnueabihf

# Build
export CC=arm-linux-gnueabihf-gcc
export CXX=arm-linux-gnueabihf-g++
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc
cargo build --release --target=armv7-unknown-linux-gnueabihf
```

#### For i686 (32-bit)
```bash
# Install cross-compilation tools
sudo apt-get install -y crossbuild-essential-i386

# Add Rust target
rustup target add i686-unknown-linux-gnu

# Build
export CC=i686-linux-gnu-gcc
export CXX=i686-linux-gnu-g++
export CARGO_TARGET_I686_UNKNOWN_LINUX_GNU_LINKER=i686-linux-gnu-gcc
cargo build --release --target=i686-unknown-linux-gnu
```

## Optimizing Binary Size

### Using UPX Compression

```bash
# Install UPX
# Linux (Debian/Ubuntu)
sudo apt-get install upx-ucl

# macOS
brew install upx

# Windows
choco install upx

# Compress the binary
upx --best --lzma target/release/ygege
```

### Static Linking (Windows)

For standalone Windows executables without runtime dependencies:

```powershell
$env:RUSTFLAGS = "-C target-feature=+crt-static"
cargo build --release
```

## Docker Build

### Local Build

```bash
# Build for current platform only (fastest)
docker build -f docker/Dockerfile -t ygege:local .

# Multi-platform build (requires Docker Buildx)
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -f docker/Dockerfile \
  -t ygege:latest \
  --load \
  .
```

### With Build Arguments

```bash
docker build \
  -f docker/Dockerfile \
  --build-arg BUILD_COMMIT=$(git rev-parse HEAD) \
  --build-arg BUILD_DATE=$(date -u +%s) \
  --build-arg BUILD_BRANCH=$(git branch --show-current) \
  -t ygege:latest \
  .
```

## Running the Binary

After building, you can run Ygégé:

```bash
# Show version
./target/release/ygege --version

# Run with default settings (see README.md for configuration)
./target/release/ygege
```

## Troubleshooting

### Common Issues

#### **Missing dependencies during build**
Make sure all platform-specific prerequisites are installed. The error messages usually indicate which library is missing.

#### **Cross-compilation failures on boring-sys2**
The BoringSSL dependency can be tricky to cross-compile. If you encounter issues:
- Ensure all cross-compilation toolchains are properly installed
- Verify environment variables are set correctly
- Consider using Docker for cross-platform builds instead

#### **Out of memory errors**
Rust compilation can be memory-intensive. Try:
```bash
# Reduce parallel jobs
cargo build --release -j 2

# Or build in chunks by disabling incremental compilation
export CARGO_INCREMENTAL=0
cargo build --release
```

#### **Windows: link.exe not found**
Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022) with the C++ workload.

## Build Performance Tips

1. **Enable incremental compilation** (development builds):
   ```bash
   export CARGO_INCREMENTAL=1
   ```

2. **Use `sccache` or `cargo-cache`** for caching:
   ```bash
   cargo install sccache
   export RUSTC_WRAPPER=sccache
   ```

3. **Parallel builds**: Cargo builds in parallel by default. Adjust with:
   ```bash
   cargo build -j <number_of_cores>
   ```

## CI/CD Builds

Official builds are automatically created by GitHub Actions for every release:
- Linux: x86_64, i686, aarch64, armv7
- Windows: x86_64, i686
- macOS: x86_64 (Intel), aarch64 (Apple Silicon)
- Docker: linux/amd64, linux/arm64

Check the [Releases](https://github.com/UwUDev/ygege/releases) page for pre-built binaries.
