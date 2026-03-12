---
sidebar_position: 2
---

# Guide du Pipeline CI/CD Ygégé

> **Démarrage rapide :** Ce document explique comment Ygégé est automatiquement compilé, testé et distribué. Utilisez-le comme référence pour dépanner les problèmes de build ou comprendre le processus de release.

## 📋 Table des matières
- [Ce qui est compilé automatiquement](#ce-qui-est-compilé-automatiquement)
- [Comment obtenir Ygégé](#comment-obtenir-ygégé)
- [Stratégie des branches](#stratégie-des-branches)
- [Informations de version](#informations-de-version)
- [Dépannage](#dépannage)
- [Pour les contributeurs](#pour-les-contributeurs)

---

## Ce qui est compilé automatiquement

À chaque fois que du code est poussé sur GitHub, notre système automatisé compile Ygégé pour plusieurs plateformes :

### 📦 Téléchargements binaires (16 variantes)

| Plateforme | Architectures | Types |
|------------|---------------|-------|
| **Linux (glibc)** | x86_64, i686, aarch64, armv7 | Normal + compressé UPX |
| **Windows** | x86_64, i686 | Normal + compressé UPX |
| **macOS** | Intel (x86_64), Apple Silicon (aarch64) | Normal + compressé UPX |

**Qu'est-ce qu'UPX ?** Les versions compressées sont 50-70% plus petites mais mettent légèrement plus de temps à démarrer. Utilisez la version normale si vous n'êtes pas sûr.

### 🐳 Images Docker (2 plateformes)

| Architecture | Appareils |
|--------------|-----------|
| **linux/amd64** | La plupart des PCs, serveurs, instances cloud |
| **linux/arm64** | Raspberry Pi 4+, Apple M1/M2/M3, AWS Graviton |

---

## Comment obtenir Ygégé

### Option 1 : Docker (Recommandé pour les serveurs) 🐳

```bash
# Dernière version stable
docker pull uwucode/ygege:latest

# Version de développement (dernières fonctionnalités, peut être instable)
docker pull uwucode/ygege:develop

# Version spécifique
docker pull uwucode/ygege:0.4.2
```

**Registre alternatif (GitHub) :**
```bash
docker pull ghcr.io/uwudev/ygege:latest
```

**Exécuter avec Docker :**
```bash
docker run -d \
  -p 8080:8080 \
  -v ./sessions:/app/sessions \
  uwucode/ygege:latest
```

### Option 2 : Télécharger les binaires 💾

1. Allez sur [GitHub Actions](https://github.com/UwUDev/ygege/actions)
2. Cliquez sur la dernière exécution de workflow réussie
3. Faites défiler jusqu'à la section **"Artifacts"**
4. Téléchargez le fichier correspondant à votre système :
   - **Linux (la plupart des systèmes) :** `ygege-linux-gnu-x86_64.zip`
   - **Linux (ARM/Raspberry Pi) :** `ygege-linux-gnu-aarch64.zip` ou `ygege-linux-gnu-armv7.zip`
   - **Windows :** `ygege-windows-x86_64.zip`
   - **macOS (Intel) :** `ygege-macos-x86_64.zip`
   - **macOS (Apple Silicon) :** `ygege-macos-aarch64.zip`

**Vous voulez des fichiers plus petits ?** Cherchez les versions `-upx` (ex : `ygege-linux-gnu-x86_64-upx.zip`)

**Note :** Les artifacts sont conservés pendant 7 jours. Pour des releases permanentes, utilisez les images Docker ou attendez une release taguée.

---

## Stratégie des branches

Comprendre quelle version vous utilisez :

| Branche | Objectif | Quand l'utiliser | Tag Docker |
|---------|----------|------------------|------------|
| **master** | Releases stables | Utilisation en production | `latest`, `stable`, `0.4.2` |
| **develop** | Dernier développement | Tester les nouvelles fonctionnalités | `develop` |

### Laquelle utiliser ?

- 🟢 **master** : Pour les serveurs de production (le plus stable)
- 🔴 **develop** : Pour les développeurs et testeurs précoces (peut contenir des bugs)

---

## Informations de version

### Vérifier votre version

Chaque binaire Ygégé inclut des informations de build :

```bash
# Afficher les détails de version
./ygege --version
```

**Sortie :**
```
Ygégé v0.4.2
Commit: a1b2c3d4e5f6
Build Date: 2025-11-11T10:30:00Z
Branch: develop
```

**Ce que cela vous indique :**
- **Version** : Le numéro de release
- **Commit** : Snapshot exact du code (utile pour les rapports de bugs)
- **Build Date** : Quand ce binaire a été compilé
- **Branch** : Quelle branche de version vous utilisez

### Version dans les logs

Quand vous démarrez Ygégé, il enregistre automatiquement les infos de version :

```
INFO Ygégé v0.4.2 (commit: a1b2c3d, branch: develop, built: 2025-11-11T10:30:00Z)
INFO Logged in to YGG with username: youruser
```

---

## Dépannage

### ❓ "Je ne trouve pas le binaire pour mon système"

**Solution :** Vérifiez la [section Artifacts](#option-2--télécharger-les-binaires-💾) dans GitHub Actions. Nous compilons pour :
- Linux : x86_64 (la plupart des PCs), i686 (32-bit), aarch64 (ARM 64-bit), armv7 (ARM 32-bit)
- Windows : x86_64 (64-bit), i686 (32-bit)
- macOS : x86_64 (Intel), aarch64 (Apple Silicon)

Pas dans la liste ? Ouvrez une [issue](https://github.com/UwUDev/ygege/issues) pour demander votre plateforme.

### ❓ "Le pull Docker échoue ou l'image est introuvable"

**Vérifiez :**
1. Orthographe : `uwucode/ygege` (pas `uwudev`)
2. Le tag existe : `develop`, `latest`, `stable`, ou numéro de version
3. Essayez le registre alternatif : `ghcr.io/uwudev/ygege:latest`

**Exemple d'erreur :**
```
Error: manifest for uwucode/ygege:wrong-tag not found
```
**Solution :** Utilisez un tag valide comme `develop` ou `latest`

### ❓ "Le binaire ne s'exécute pas / Permission refusée"

**Linux/macOS :**
```bash
chmod +x ygege
./ygege
```

**Windows :** Clic droit → Propriétés → Débloquer → Appliquer

### ❓ "La version UPX plante au démarrage"

**Solution :** Utilisez la version normale (non-UPX). Certains antivirus signalent les exécutables compressés.

### ❓ "Comment signaler un problème de build ?"

Lors du signalement de problèmes, incluez :
1. **Votre système :** OS, architecture (exécutez `uname -m` sur Linux/macOS)
2. **Info de version :** Sortie de `./ygege --version`
3. **Comment vous l'avez obtenu :** Tag Docker ou nom d'artifact
4. **Message d'erreur :** Sortie d'erreur complète

**Modèle :**
```
Système: Ubuntu 22.04 x86_64
Version: Ygégé v0.4.2 (commit: a1b2c3d, branch: develop)
Source: Docker uwucode/ygege:develop
Erreur: [collez l'erreur ici]
```

### ❓ "Artifacts expirés (erreur 404)"

Les artifacts sont conservés pendant 7 jours. Options :
- Utilisez les images Docker (permanentes)
- Compilez depuis les sources
- Attendez le prochain commit pour déclencher de nouveaux builds

---

## Pour les contributeurs

### Configuration du CI/CD

**Secrets GitHub requis :**

Allez sur : `Settings` → `Secrets and variables` → `Actions` → `New repository secret`

| Nom du Secret | Description | Comment l'obtenir |
|---------------|-------------|-------------------|
| `DOCKERHUB_USERNAME` | Votre nom d'utilisateur Docker Hub | Nom de votre compte Docker Hub |
| `DOCKERHUB_TOKEN` | Jeton d'accès Docker Hub | [Créer un token](https://hub.docker.com/settings/security) |

**Note :** `GITHUB_TOKEN` est automatique, aucune configuration nécessaire.

### Quand le CI s'exécute-t-il ?

| Événement | Ce qui se passe |
|-----------|-----------------|
| **Push sur develop/master** | Build complet + publication Docker + artifacts |
| **Pull Request** | Tests + vérification du build uniquement (pas de publication) |
| **Déclenchement manuel** | Via l'onglet Actions → "Run workflow" |

### Temps de build

Durées approximatives :
- **Tests uniquement :** ~5 minutes
- **Tous les binaires (16) :** ~30-45 minutes
- **Images Docker :** ~15-20 minutes
- **Total (sur develop/master) :** ~60-80 minutes

### Modifier le CI

**Fichiers à connaître :**
- `.github/workflows/ci.yml` - Configuration CI principale
- `docker/Dockerfile` - Instructions de build de l'image Docker
- `src/main.rs` - Logique d'affichage de la version

**Avant de modifier :**
1. Testez localement si possible
2. Utilisez une branche de fonctionnalité
3. Vérifiez que le CI passe sur votre PR avant de merger

### Tâches CI courantes

**Ajouter une nouvelle architecture :**
Éditez `.github/workflows/ci.yml` → Ajoutez à la matrice sous le job concerné

**Changer les tags Docker :**
Éditez `.github/workflows/ci.yml` → job `docker` → étape `Determine Docker tags`

**Mettre à jour la version de Rust :**
Éditez `docker/Dockerfile` → Changez `FROM rust:1.94-slim-trixie`

---

## Détails techniques

### Optimisations de build

Tous les binaires utilisent ces paramètres de profil Cargo :
```toml
[profile.release]
opt-level = "z"      # Optimiser pour la taille
lto = true           # Optimisation au moment du link
codegen-units = 1    # Meilleure optimisation
strip = true         # Supprimer les symboles de debug
```

### Labels de l'image Docker

Les images incluent des métadonnées OpenContainer :
- `org.opencontainers.image.source` - URL du dépôt GitHub
- `org.opencontainers.image.revision` - SHA du commit Git
- `org.opencontainers.image.created` - Horodatage du build

Visualiser avec : `docker inspect uwucode/ygege:latest`

### Variables d'environnement de build

Celles-ci sont intégrées pendant la compilation :
- `BUILD_COMMIT` - SHA du commit Git
- `BUILD_DATE` - Horodatage ISO 8601
- `BUILD_BRANCH` - Nom de la branche (develop/master)

---

## Support

**Vous avez trouvé un bug ?** [Ouvrez une issue](https://github.com/UwUDev/ygege/issues)

**Besoin d'aide ?** Vérifiez les issues existantes ou démarrez une discussion

**Vous voulez contribuer ?** Lisez les [directives de contribution](../contribution.md)
