## 1. Install Rust

On Linux/macOS :

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

On Windows :
[Download the installer](https://www.rust-lang.org/tools/install) et suivez les instructions.

---

## 2. Install system dependencies

**Debian/Ubuntu** :

```bash
sudo apt-get update && sudo apt-get install -y \
    build-essential \
    cmake \
    pkg-config \
    libssl-dev \
    git
```

**Fedora** :

```bash
sudo dnf install gcc cmake pkgconfig openssl-devel git
```

---

## 3. Clone the repository

```bash
git clone https://github.com/UwUDev/ygege.git
cd ygege
```

---

## 4. Compile the application

**Development mode** (debugging) :

```bash
cargo build
```

**Production mode** (optimized) :

```bash
cargo build --release
```

---

## 5. Run the application

**Debug version ** :

```bash
./target/debug/ygege --help
```

**Optimized version** :

```bash
./target/release/ygege --help
```

---

## Required configuration

Make sure your `config.json` contains valid YGG credentials :

```json
{
  "username": "your_ygg_username",
  "password": "your_ygg_password",
  "bind_ip": "0.0.0.0", 
  "bind_port": 8715,
  "log_level": "debug"
}
```

---

## Bonus: UPX compression (optional)

1. Install UPX :
```bash
sudo apt-get install upx-ucl  # Debian/Ubuntu
```

2. Compress the binary :
```bash
upx --best --lzma ./target/release/ygege
```

---

## Common troubleshooting

**Missing library error** :

```bash
sudo apt-get install libssl3 # Adapte to your distribution
```

**Update Rust** :

```bash
rustup update
```

**Project clean-up** :

```bash
cargo clean
```

This method produces an optimized native binary without Docker overload, ideal for development and deployment on dedicated servers.