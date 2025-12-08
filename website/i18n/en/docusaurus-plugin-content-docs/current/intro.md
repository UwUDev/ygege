---
sidebar_position: 1
slug: /
---

# Introduction to Ygégé

Welcome to the **Ygégé** documentation! 🚀

## What is Ygégé?

**Ygégé** is a high-performance indexer for YGG Torrent, written in Rust. It bridges YGG Torrent with your media management applications (Prowlarr, Jackett, Sonarr, Radarr, etc.).

### Why Ygégé?

- ⚡ **Exceptional Performance**: Written in Rust for maximum speed
- 🔐 **Cloudflare Bypass**: Automatic intelligent bypass without browser
- 🐳 **Simplified Deployment**: Multi-architecture Docker images (AMD64, ARM64, ARMv7)
- 🔍 **Complete Search**: Full support for YGG categories and filters
- 🎬 **Enriched Metadata**: Automatic TMDB/IMDB integration
- 🔌 **Universal Compatibility**: Works with Prowlarr, Jackett, and all \*arr applications

## Quick Start

:::tip New to Ygégé?
Follow the **[Getting Started Guide](./getting-started)** for a complete step-by-step installation.
:::

### 30-Second Installation

```bash
# Create configuration directory
mkdir -p ~/ygege/config && cd ~/ygege

# Download and start with Docker Compose
curl -o compose.yml https://raw.githubusercontent.com/UwUDev/ygege/master/docker/compose.yml
docker compose up -d
```

Don't forget to configure your YGG credentials in `config/config.json` or via environment variables.

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
- **[GitHub Issues](https://github.com/UwUDev/ygege/issues)** - Report a bug or get help

## Need Help?

- 📖 Check the **[FAQ](./faq)** for common questions
- 🐛 **[Open an issue](https://github.com/UwUDev/ygege/issues)** on GitHub
- 💬 Browse **[existing issues](https://github.com/UwUDev/ygege/issues?q=is%3Aissue)**

:::info Open Source
Ygégé is **open-source** and welcomes your contributions on **[GitHub](https://github.com/UwUDev/ygege)**!
:::
