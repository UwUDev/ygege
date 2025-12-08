---
sidebar_position: 1
slug: /
---

# Introduction à Ygégé

Bienvenue dans la documentation **Ygégé** ! 🚀

## Qu'est-ce que Ygégé ?

**Ygégé** est un indexeur haute performance pour YGG Torrent, écrit en Rust. Il permet d'intégrer YGG Torrent avec des applications comme Prowlarr, Jackett, et d'autres gestionnaires de médias.

### Caractéristiques principales

- ⚡ **Haute performance** : Écrit en Rust pour une rapidité maximale
- 🔐 **Contournement Cloudflare** : Bypass intelligent sans navigateur
- 🐳 **Docker Ready** : Images multi-architecture disponibles
- 🔍 **Recherche avancée** : Support complet des catégories et filtres
- 🎬 **Métadonnées TMDB/IMDB** : Enrichissement automatique des résultats
- 🔌 **Intégration Prowlarr/Jackett** : Configuration simple

## Démarrage rapide

### Installation avec Docker

```bash
docker run -d \
  --name ygege \
  -p 8715:8715 \
  -v ./config:/config \
  uwudev/ygege:latest
```

### Avec Docker Compose

```yaml
services:
  ygege:
    image: uwudev/ygege:latest
    container_name: ygege
    ports:
      - "8715:8715"
    volumes:
      - ./config:/config
    restart: unless-stopped
```

## Prochaines étapes

- 🚀 [Guide de démarrage](./getting-started)
- 📚 [FAQ - Questions fréquentes](./faq)
- 📖 [Guide d'installation Docker](./installation/docker-guide)
- 🔧 [Configuration](./configuration)
- 🔗 [Intégration Prowlarr](./integrations/prowlarr)
- 🔗 [Intégration Jackett](./integrations/jackett)
- 📡 [Documentation API](./api)
