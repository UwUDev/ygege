---
sidebar_position: 1
---

# Installation with Docker

Ygégé is available as an official multi-architecture Docker image. This guide explains how to deploy and configure the service.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) installed on your system
- [Docker Compose](https://docs.docker.com/compose/install/) (recommended for simplified management)
- A valid YGG Torrent account

## Quick Installation

### With Docker Run

```bash
docker run -d \
  --name ygege \
  -p 8715:8715 \
  -v ./config:/config \
  -e YGG_USERNAME="your_username" \
  -e YGG_PASSWORD="your_password" \
  uwucode/ygege:latest
```

### With Docker Compose

Create a `compose.yml` file:

```yaml
services:
  ygege:
    image: uwucode/ygege:latest
    container_name: ygege
    restart: unless-stopped
    ports:
      - "8715:8715"
    volumes:
      - ./config:/config
    environment:
      YGG_USERNAME: "your_username"
      YGG_PASSWORD: "your_password"
      LOG_LEVEL: "debug"
    healthcheck:
      test: ["CMD-SHELL", "curl --fail http://localhost:8715/health || exit 1"]
      interval: 1m30s
      timeout: 20s
      retries: 3
      start_period: 10s
```

Then start the service:

```bash
docker compose up -d
```

## Configuration

### With config.json file

Create a `config/config.json` file:

```json
{
    "username": "your_ygg_username",
    "password": "your_password",
    "bind_ip": "0.0.0.0",
    "bind_port": 8715,
    "log_level": "debug"
}
```

### With environment variables

The following variables are supported:

| Variable | Description | Default |
|----------|-------------|---------|
| `YGG_USERNAME` | YGG username | - |
| `YGG_PASSWORD` | YGG password | - |
| `BIND_IP` | Listening IP address | `0.0.0.0` |
| `BIND_PORT` | Listening port | `8715` |
| `LOG_LEVEL` | Log level (trace, debug, info, warn, error) | `info` |

## Available Docker Tags

| Tag | Description |
|-----|-------------|
| `latest` | Latest stable version |
| `stable` | Alias for latest |
| `noupx` | Version without UPX compression (for Synology) |
| `0.6.2` | Specific version |
| `develop` | Development version |

### For systems with older architectures

If you encounter segmentation faults on older architectures or certain NAS (like Synology), use the `noupx` image:

```yaml
services:
  ygege:
    image: uwucode/ygege:noupx
    # ... rest of configuration
```

## Verification

Once the container is started, verify it's working:

```bash
curl http://localhost:8715/health
```

You should receive an `OK` response.

## Next Steps

- [Advanced Configuration](../configuration)
- [Prowlarr Integration](../integrations/prowlarr)
- [Jackett Integration](../integrations/jackett)
