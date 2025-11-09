## 1. Installer Rust

Sur Linux/macOS :

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

Sur Windows :
[Téléchargez l'installeur](https://www.rust-lang.org/tools/install) et suivez les instructions.

---

## 2. Installer les dépendances système

**Debian/Ubuntu** :

```bash
sudo apt-get update &amp;&amp; sudo apt-get install -y \
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

## 3. Cloner le dépôt

```bash
git clone https://github.com/UwUDev/ygege.git
cd ygege
```

---

## 4. Compiler l'application

**Mode développement** (débogage) :

```bash
cargo build
```

**Mode production** (optimisé) :

```bash
cargo build --release
```

---

## 5. Exécuter l'application

**Version debug** :

```bash
./target/debug/ygege --help
```

**Version optimisée** :

```bash
./target/release/ygege --help
```

---

## Configuration requise

Assurez-vous que votre `config.json` contient des identifiants YGG valides :

```json
{
    "username": "votre_identifiant",
    "password": "votre_mot_de_passe",
    "bind_ip": "0.0.0.0",
    "bind_port": 8715,
    "log_level": "debug"
}
```

---

## Bonus : Compression UPX (optionnel)

1. Installer UPX :

```bash
sudo apt-get install upx-ucl  # Debian/Ubuntu
```

2. Compresser le binaire :

```bash
upx --best --lzma ./target/release/ygege
```

---

## Dépannage courant

**Erreur de librairie manquante** :

```bash
sudo apt-get install libssl3
```

**Mise à jour de Rust** :

```bash
rustup update
```

**Nettoyage du projet** :

```bash
cargo clean
```