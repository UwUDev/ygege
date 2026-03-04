---
sidebar_position: 1
---

# Installation with Docker

Ygégé is available as an official multi-architecture Docker image. This guide explains how to deploy and configure the service.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) installed on your system
- [Docker Compose](https://docs.docker.com/compose/install/) (recommended for simplified management)

## Quick Installation

### With Docker Run

```bash
docker run -d \
  --name ygege \
  -p 8715:8715 \
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
    environment:
      LOG_LEVEL: "info"
      # TMDB_TOKEN: "your_tmdb_token"  # Optional
      # RELAY_URL: "wss://relay.ygg.gratis"  # Optional
    healthcheck:
      test: ["CMD-SHELL", "curl --fail http://localhost:$${BIND_PORT:-8715}/health || exit 1"]
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

Create a `config.json` file and mount it read-only:

```json
{
    "bind_ip": "0.0.0.0",
    "bind_port": 8715,
    "log_level": "info",
    "tmdb_token": null,
    "relay_url": null
}
```

### With environment variables

The following variables are supported:

| Variable | Description | Default |
|----------|-------------|---------|
| `BIND_IP` | Listening IP address | `0.0.0.0` |
| `BIND_PORT` | Listening port | `8715` |
| `LOG_LEVEL` | Log level (trace, debug, info, warn, error) | `info` |
| `TMDB_TOKEN` | TMDB API token (optional) | - |
| `RELAY_URL` | Nostr relay URL (optional) | `wss://relay.ygg.gratis` |
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

## Security

### Non-root User

The Ygégé Docker image runs by default with a non-root user (UID 10001) for security reasons. This ensures:

- ✅ Compatibility with Docker and Kubernetes security policies
- ✅ Protection against privilege escalation
- ✅ Compliance with container security best practices

### Permission Management

:::warning "Permission denied" Error
If you get `Error: Os { code: 13, kind: PermissionDenied }` after updating, it's related to volume permissions.
:::

**Recommended solution**: Use **named volumes** (already in the example above):

```yaml
volumes:
  - ygege_sessions:/app/sessions  # Automatic permission management
```

**Alternative with bind mounts**: If you need to mount a local folder:

```yaml
services:
  ygege:
    image: uwucode/ygege:latest
    user: "10001:10001"  # Container UID/GID
    volumes:
      - ./ygege/sessions:/app/sessions
```

Then set permissions:

**Linux/macOS**:
```bash
sudo chown -R 10001:10001 ./ygege/sessions
sudo chmod -R 755 ./ygege/sessions
```

**Windows** (PowerShell as Administrator):
```powershell
icacls ".\ygege\sessions" /grant Everyone:(OI)(CI)F /T
```

### Running with a Custom UID

If you want to run the container with a specific UID/GID (for example to match your host user):

```bash
docker run -d \
  --name ygege \
  --user 1000:1000 \
  -p 8715:8715 \
  -v ./config:/app/sessions \
  -v ./config.json:/app/config.json \
  uwucode/ygege:latest
```

Or with Docker Compose:

```yaml
services:
  ygege:
    image: uwucode/ygege:latest
    user: "1000:1000"  # Your UID:GID
    # ... rest of configuration
```

:::tip
Make sure mounted volumes have appropriate permissions for the specified user:
```bash
sudo chown -R 1000:1000 ./config ./sessions
```
:::

### Running as Root (Not Recommended)

:::danger Security Warning
Running as root is **not recommended** and may present security risks. Use this option only if you understand the implications.
:::

If you absolutely need to run the container as root:

**Docker Run**:
```bash
docker run -d \
  --name ygege \
  --user 0:0 \
  -p 8715:8715 \
  -v ./ygege/sessions:/app/sessions \
  -e YGG_USERNAME="your_username" \
  -e YGG_PASSWORD="your_password" \
  uwucode/ygege:latest
```

**Docker Compose**:
```yaml
services:
  ygege:
    image: uwucode/ygege:latest
    container_name: ygege
    user: "0:0"  # Root
    restart: unless-stopped
    environment:
      YGG_USERNAME: "your_username"
      YGG_PASSWORD: "your_password"
    volumes:
      - ./ygege/sessions:/app/sessions
    ports:
      - "8715:8715"
```

With this configuration, you won't have permission issues, but you lose the security benefits of non-root mode.

## Next Steps

- [Advanced Configuration](../configuration)
- [Prowlarr Integration](../integrations/prowlarr)
- [Jackett Integration](../integrations/jackett)
