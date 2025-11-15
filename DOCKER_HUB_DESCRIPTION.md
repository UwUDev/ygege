# Ygégé - High-Performance YGG Torrent Indexer

[![GitHub](https://img.shields.io/badge/GitHub-UwUDev%2Fygege-blue?logo=github)](https://github.com/UwUDev/ygege)
[![License](https://img.shields.io/github/license/UwUDev/ygege)](https://github.com/UwUDev/ygege/blob/master/LICENSE)

Ygégé is a blazing-fast indexer for YGG Torrent written in Rust, designed for seamless integration with Prowlarr and other media automation tools.

## 🚀 Key Features

- **Automatic Cloudflare Bypass** - No manual challenge solving required
- **Lightning Fast** - Near-instant search results with low memory footprint (14.7MB)
- **Smart Session Management** - Automatic reconnection with session caching
- **DNS Bypass** - Circumvents lying DNS servers
- **Zero Dependencies** - No browser drivers or external services needed
- **Prowlarr Ready** - Drop-in custom indexer support
- **TMDB/IMDB Integration** - Enhanced metadata fetching
- **Multi-Architecture** - Supports `linux/amd64` and `linux/arm64`

## 📦 Quick Start

### Using Docker Run

```bash
docker run -d \
  --name ygege \
  -p 8715:8715 \
  -e YGG_USERNAME=your_username \
  -e YGG_PASSWORD=your_password \
  -v ./ygege/sessions:/app/sessions \
  --restart unless-stopped \
  uwucode/ygege:latest
```

### Using Docker Compose

Create a `compose.yml` file:

```yaml
services:
  ygege:
    image: uwucode/ygege:latest
    container_name: ygege
    restart: unless-stopped
    environment:
      - YGG_USERNAME=your_username
      - YGG_PASSWORD=your_password
      - BIND_IP=0.0.0.0
      - BIND_PORT=8715
      - LOG_LEVEL=info
    volumes:
      - ./ygege/sessions:/app/sessions
    ports:
      - 8715:8715
```

Then run:

```bash
docker compose up -d
```

## 🔧 Configuration

### Environment Variables

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `YGG_USERNAME` | Your YGG Torrent username | - | ✅ Yes |
| `YGG_PASSWORD` | Your YGG Torrent password | - | ✅ Yes |
| `BIND_IP` | Server bind address | `0.0.0.0` | No |
| `BIND_PORT` | Server port | `8715` | No |
| `LOG_LEVEL` | Log level (trace, debug, info, warn, error) | `info` | No |

### Using config.json (Alternative)

Mount a configuration file instead of using environment variables:

```yaml
services:
  ygege:
    image: uwucode/ygege:latest
    container_name: ygege
    restart: unless-stopped
    volumes:
      - ./ygege/sessions:/app/sessions
      - ./ygege/config.json:/app/config.json
    ports:
      - 8715:8715
```

**config.json example:**

```json
{
  "username": "your_username",
  "password": "your_password",
  "bind_ip": "0.0.0.0",
  "bind_port": 8715,
  "log_level": "info"
}
```

## 🔌 Prowlarr Integration

1. Find your Prowlarr AppData directory (in `/system/status` page)
2. Download `ygege.yml` from the [GitHub repo](https://github.com/UwUDev/ygege/blob/master/ygege.yml)
3. Copy it to `{prowlarr_appdata}/Definitions/Custom/` (create `Custom` folder if needed)
4. Restart Prowlarr
5. Add Ygégé as an indexer with:
   - **Base URL**: `http://localhost:8715/` (or `http://ygege:8715/` for Docker Compose)

## 🏷️ Available Tags

| Tag | Description |
|-----|-------------|
| `latest`, `stable` | Latest stable release (UPX-compressed) |
| `latest-noupx`, `noupx`, `stable-noupx` | Latest stable release (uncompressed, for Synology/older systems) |
| `0.5.0`, `0.5`, `0` | Specific version (UPX-compressed) |
| `0.5.0-noupx`, `0.5-noupx` | Specific version (uncompressed) |
| `master` | Latest build from master branch |
| `master-noupx` | Latest build from master branch (uncompressed) |
| `develop`, `beta` | Development/beta builds |
| `develop-noupx`, `beta-noupx` | Development/beta builds (uncompressed) |

> **Note for Synology NAS users**: If you experience segfaults (exit code 139), use the `-noupx` tags which contain uncompressed binaries compatible with older kernels/CPUs.

## 📊 API Endpoints

- `GET /search` - Search torrents with advanced filters
- `GET /torrent/info` - Get detailed torrent information
- `GET /torrent/{id}/files` - List files in a torrent
- `GET /health` - Health check endpoint

Full API documentation: [GitHub Docs](https://github.com/UwUDev/ygege/blob/master/docs/api-documentation.md)

## 🛡️ Security & Performance

Ygégé uses [wreq](https://crates.io/crates/wreq) for advanced TLS and HTTP/2 fingerprinting to bypass Cloudflare detection without browser automation. This ensures reliable access while maintaining YGG Torrent's terms of service.

**Performance benchmarks:**
- Domain resolution: ~128ms average
- Search query: ~176ms average
- Session restoration: ~206ms average

## 📝 Important Notes

> **⚠️ Rate Limiting**: Always provide valid YGG credentials via environment variables or config file to avoid being rate-limited or blocked by YGG Torrent.

> **🔐 Credentials Security**: For Docker Compose setups, consider using Docker secrets or environment files instead of hardcoding credentials.

## 🆘 Support & Documentation

- **GitHub Repository**: [UwUDev/ygege](https://github.com/UwUDev/ygege)
- **Full Documentation**: [GitHub Docs](https://github.com/UwUDev/ygege/tree/master/docs)
- **Issues & Bug Reports**: [GitHub Issues](https://github.com/UwUDev/ygege/issues)
- **Docker Guide**: [Detailed Docker Setup](https://github.com/UwUDev/ygege/blob/master/docs/docker-guide.md)

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/UwUDev/ygege/blob/master/LICENSE) file for details.

---

**Built with ❤️ using Rust** | Multi-arch support: `linux/amd64`, `linux/arm64`
