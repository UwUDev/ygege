---
sidebar_position: 1
slug: /
---

# Introduction à Ygégé

Bienvenue dans la documentation **Ygégé** ! 🚀

## Qu'est-ce que Ygégé ?

**Ygégé** est un indexeur haute performance pour YGG Torrent, écrit en Rust. Il fait le pont entre YGG Torrent et vos applications de gestion de médias (Prowlarr, Jackett, Sonarr, Radarr, etc.).

### Pourquoi Ygégé ?

- ⚡ **Performance exceptionnelle** : Écrit en Rust pour une rapidité maximale
- 🔐 **Contournement Cloudflare** : Bypass automatique intelligent sans navigateur
- 🐳 **Déploiement simplifié** : Images Docker multi-architecture (AMD64, ARM64, ARMv7)
- 🔍 **Recherche complète** : Support intégral des catégories et filtres YGG
- 🎬 **Métadonnées enrichies** : Intégration TMDB/IMDB automatique
- 🔌 **Compatible universel** : Fonctionne avec Prowlarr, Jackett et toutes les applications \*arr

## Démarrage rapide

:::tip Nouveau sur Ygégé ?
Suivez le **[Guide de démarrage](./getting-started)** pour une installation complète pas à pas.
:::

### Installation en 30 secondes

```bash
# Créer le dossier de configuration
mkdir -p ~/ygege/config && cd ~/ygege

# Télécharger et démarrer avec Docker Compose
curl -o compose.yml https://raw.githubusercontent.com/UwUDev/ygege/master/docker/compose.yml
docker compose up -d
```

N'oubliez pas de configurer vos identifiants YGG dans `config/config.json` ou via les variables d'environnement.

## Navigation de la documentation

### 🚀 Installation

- **[Guide de démarrage](./getting-started)** - Installation et configuration complète
- **[Installation Docker](./installation/docker-guide)** - Guide détaillé Docker
- **[Compilation depuis les sources](./installation/source-guide)** - Pour les développeurs

### 🔌 Intégrations

- **[Prowlarr](./integrations/prowlarr)** - Configuration avec Prowlarr (recommandé)
- **[Jackett](./integrations/jackett)** - Alternative à Prowlarr

### 📖 Développeur

- **[Documentation API](./api)** - Référence API REST complète
- **[Configuration TMDB/IMDB](./tmdb-imdb)** - Enrichissement métadonnées

### ❓ Support

- **[FAQ](./faq)** - Questions fréquentes
- **[GitHub Issues](https://github.com/UwUDev/ygege/issues)** - Rapporter un bug ou demander de l'aide

## Besoin d'aide ?

- 📖 Consultez la **[FAQ](./faq)** pour les questions courantes
- 🐛 **[Ouvrez une issue](https://github.com/UwUDev/ygege/issues)** sur GitHub
- 💬 Parcourez les **[issues existantes](https://github.com/UwUDev/ygege/issues?q=is%3Aissue)**

:::info Open Source
Ygégé est **open-source** et accueille vos contributions sur **[GitHub](https://github.com/UwUDev/ygege)** !
:::
