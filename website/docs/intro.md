---
sidebar_position: 1
slug: /
---

# Introduction à Francisca

Bienvenue dans la documentation **Francisca** ! 🚀

## Qu'est-ce que Francisca ?

**Francisca** est un indexeur haute performance *general-purpose* pour les systèmes supportant le protocole Nostr (NIP-35), écrit en Rust. Il fait le pont entre n'importe quel relais Nostr et vos applications de gestion de médias (Prowlarr, Jackett, Sonarr, Radarr, etc.) via le protocole **Nostr** (NIP-35).

### Pourquoi Francisca ?

- ⚡ **Performance exceptionnelle** : Écrit en Rust pour une rapidité maximale
- 🔓 **Aucun compte requis** : selon le relais Nostr (souvent public), aucun identifiant nécessaire
- 📡 **Protocole Nostr** : Connexion directe à n'importe quel relais Nostr (NIP-35)
- 🐳 **Déploiement simplifié** : Images Docker multi-architecture (AMD64, ARM64, ARMv7)
- 🔍 **Recherche complète** : Support intégral des catégories et filtres
- 🎬 **Métadonnées enrichies** : Intégration TMDB/IMDB automatique
- 🔌 **Compatible universel** : Fonctionne avec Prowlarr, Jackett et toutes les applications \*arr
- 🧅 **Support Tor** : Routage optionnel des connexions via Tor

## Démarrage rapide

:::tip Nouveau sur Francisca ?
Suivez le **[Guide de démarrage](./getting-started)** pour une installation complète pas à pas.
:::

### Installation en 30 secondes

```bash
# Créer le dossier de configuration
mkdir -p ~/francisca && cd ~/francisca

# Télécharger et démarrer avec Docker Compose
curl -o compose.yml https://raw.githubusercontent.com/UwUDev/francisca/master/docker/compose.yml
docker compose up -d
```

:::info Aucune configuration requise
Aucun compte ni identifiant à configurer (selon le relais). Francisca fonctionne directement après démarrage.
:::

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
- **[GitHub Issues](https://github.com/UwUDev/francisca/issues)** - Rapporter un bug ou demander de l'aide

## Besoin d'aide ?

- 📖 Consultez la **[FAQ](./faq)** pour les questions courantes
- 🐛 **[Ouvrez une issue](https://github.com/UwUDev/francisca/issues)** sur GitHub
- 💬 Parcourez les **[issues existantes](https://github.com/UwUDev/francisca/issues?q=is%3Aissue)**

:::info Open Source
Francisca est **open-source** et accueille vos contributions sur **[GitHub](https://github.com/UwUDev/francisca)** !
:::
