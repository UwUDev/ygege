#!/usr/bin/bash

# Get version from Cargo.toml
VERSION=$(grep '^version =' Cargo.toml | head -1 | cut -d '"' -f2)
echo "Building releases for version $VERSION"

# Clean previous builds
cargo clean
rm -rf target/
rm -rf releases/v$VERSION

# Linux x86 Release Build
cargo build --release

# Windows Release Build
cargo build --target x86_64-pc-windows-gnu --release

# Linux ARMv7 Release Build
./compiling/cross-compile-linux-armv7.sh

# Create releases directory
mkdir -p releases/v$VERSION

# Copy binaries to releases directory
cp target/release/ygege releases/v$VERSION/ygege-linux-x86_64
cp target/x86_64-pc-windows-gnu/release/ygege.exe releases/v$VERSION/ygege-windows-x86_64.exe
cp target/armv7-unknown-linux-gnueabihf/release/ygege releases/v$VERSION/ygege-linux-armv7

echo "Releases built and stored in releases/v$VERSION/"

cargo clean


