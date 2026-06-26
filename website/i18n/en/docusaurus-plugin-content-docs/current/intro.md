---
sidebar_position: 1
slug: /
---

# Introduction to Francisca

Welcome to the **Francisca** documentation! 🚀

## What is Francisca?

**Francisca** is a general-purpose, high-performance indexer for any system that supports the Nostr protocol (NIP-35), written in Rust. It bridges any Nostr relay with your media management applications (Prowlarr, Jackett, Sonarr, Radarr, etc.) via the **Nostr** protocol (NIP-35).

### Why Francisca?

- ⚡ **Exceptional Performance**: Written in Rust for maximum speed
- 🔓 **No Account Required**: depending on the Nostr relay (often public), no credentials needed
- 📡 **Nostr Protocol**: Direct connection to any Nostr relay (NIP-35)
- 🐳 **Simplified Deployment**: Multi-architecture Docker images (AMD64, ARM64, ARMv7)
- 🔍 **Complete Search**: Full support for categories and filters
- 🎬 **Enriched Metadata**: Automatic TMDB/IMDB integration
- 🔌 **Universal Compatibility**: Works with Prowlarr, Jackett, and all \*arr applications
- 🧅 **Tor Support**: Optional routing of connections through Tor

## Quick Start

:::tip New to Francisca?
Follow the **[Getting Started Guide](./getting-started)** for a complete step-by-step installation.
:::

### 30-Second Installation

```bash
# Create configuration directory
mkdir -p ~/francisca && cd ~/francisca

# Download and start with Docker Compose
curl -o compose.yml https://raw.githubusercontent.com/UwUDev/francisca/master/docker/compose.yml
docker compose up -d
```

:::info No configuration required
No account or credentials to configure (depending on the relay). Francisca works immediately after startup.
:::

## Documentation Navigation

### 🚀 Installation

- **[Getting Started Guide](./getting-started)** - Complete installation and configuration
- **[Docker Installation](./installation/docker-guide)** - Detailed Docker guide
- **[Build from Source](./installation/source-guide)** - For developers

### 🔌 Integrations

- **[Prowlarr](./integrations/prowlarr)** - Prowlarr configuration (recommended)
- **[Jackett](./integrations/jackett)** - Alternative to Prowlarr

### 📖 Developer

- **[API Documentation](./api)** - Complete REST API reference
- **[TMDB/IMDB Configuration](./tmdb-imdb)** - Metadata enrichment

### ❓ Support

- **[FAQ](./faq)** - Frequently asked questions
- **[GitHub Issues](https://github.com/UwUDev/francisca/issues)** - Report a bug or get help

## Need Help?

- 📖 Check the **[FAQ](./faq)** for common questions
- 🐛 **[Open an issue](https://github.com/UwUDev/francisca/issues)** on GitHub
- 💬 Browse **[existing issues](https://github.com/UwUDev/francisca/issues?q=is%3Aissue)**

:::info Open Source
Francisca is **open-source** and welcomes your contributions on **[GitHub](https://github.com/UwUDev/francisca)**!
:::
