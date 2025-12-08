---
sidebar_position: 1
slug: /
---

# Introduction to Ygégé

Welcome to the **Ygégé** documentation! 🚀

## What is Ygégé?

**Ygégé** is a high-performance indexer for YGG Torrent, written in Rust. It allows you to integrate YGG Torrent with applications like Prowlarr, Jackett, and other media management tools.

### Main Features

- ⚡ **High Performance**: Written in Rust for maximum speed
- 🔐 **Cloudflare Bypass**: Intelligent bypass without browser
- 🐳 **Docker Ready**: Multi-architecture images available
- 🔍 **Advanced Search**: Full support for categories and filters
- 🎬 **TMDB/IMDB Metadata**: Automatic result enrichment
- 🔌 **Prowlarr/Jackett Integration**: Simple configuration

## Quick Start

### Installation with Docker

```bash
docker run -d \
  --name ygege \
  -p 8715:8715 \
  -v ./config:/config \
  uwudev/ygege:latest
```

### With Docker Compose

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

## Next Steps

- 🚀 [Getting Started Guide](./getting-started)
- 📚 [FAQ - Frequently Asked Questions](./faq)
- 📖 [Docker Installation Guide](./installation/docker-guide)
- 🔧 [Configuration](./configuration)
- 🔗 [Prowlarr Integration](./integrations/prowlarr)
- 🔗 [Jackett Integration](./integrations/jackett)
- 📡 [API Documentation](./api)
