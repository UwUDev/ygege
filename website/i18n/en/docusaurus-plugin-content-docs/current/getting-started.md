---
sidebar_position: 2
---

# Getting Started Guide

This guide walks you step by step through installing and configuring Francisca, from initial setup to integration with your media management applications.

## Choosing an Installation Method

### Docker (Recommended)

**Advantages:**
- One-command installation
- Simplified updates
- Complete isolation
- Multi-architecture (AMD64, ARM64, ARMv7)

**For whom?**
- Users with Docker already installed
- Synology, QNAP NAS, etc.
- Linux servers
- Windows users with WSL2

👉 [Docker Guide](./installation/docker-guide)

### Manual Installation (Advanced)

**Advantages:**
- Full control
- No Docker dependency
- Native performance

**For whom?**
- Developers
- Servers without Docker
- Experienced users

:::tip Pre-compiled binaries available
With each release, **pre-compiled binaries** are provided for multiple platforms (Linux, Windows, macOS). Download them directly from the [releases page](https://github.com/UwUDev/francisca/releases).
:::

👉 To compile yourself, see the [GitHub README](https://github.com/UwUDev/francisca#building-from-source)

## Quick Installation (Docker Compose)

### Step 1: Create Configuration Directory

```bash
mkdir -p ~/francisca/config
cd ~/francisca
```

### Step 2: Create compose.yml File

```yaml
services:
  francisca:
    image: uwucode/francisca:latest
    container_name: francisca
    restart: unless-stopped
    ports:
      - "8715:8715"
    environment:
      LOG_LEVEL: "info"
      BIND_IP: "0.0.0.0"
      BIND_PORT: "8715"
      # TMDB_TOKEN: "your_tmdb_token"  # Optional: for TMDB/IMDB ID searches
      # USE_TOR: "true"               # Optional: enable Tor routing

    # Health check to verify proper operation
    healthcheck:
      test: ["CMD-SHELL", "curl -f http://localhost:8715/health || exit 1"]
      interval: 1m30s
      timeout: 10s
      retries: 3
      start_period: 30s
```

### Step 3: Start the Service

```bash
docker compose up -d
```

### Step 4: Verify Operation

```bash
# Check logs
docker compose logs -f francisca

# Test the API
curl http://localhost:8715/health
```

You should see:
```
INFO Francisca v0.x.x (commit: ..., branch: ..., built: ...)
INFO Using Nostr relay: wss://relay.example.org
INFO Categories initialized: 9 top-level categories
```

You can also access the information page in your browser: `http://localhost:8715/`

This page displays real-time status of all Francisca components:
- Nostr relay connectivity
- Search functionality
- TMDB/IMDB integration

## Basic Configuration

:::info No authentication required
Many Nostr relays are **public**. Depending on the relay, no account or credentials are needed to use Francisca.
:::

### Network Ports

By default, Francisca listens on port **8715**. If this port is already in use:

```yaml
ports:
  - "9090:8715"  # Use port 9090 on your machine
```

Or modify the port in the configuration:
```yaml
environment:
  BIND_PORT: "9090"
ports:
  - "9090:9090"
```

## Integration with Your Applications

Once Francisca is configured, integrate it with your applications:

### Prowlarr (Recommended)

Prowlarr automatically synchronizes indexers with Sonarr, Radarr, Lidarr, etc.

1. Download the [`francisca.yml`](https://github.com/UwUDev/francisca/blob/master/francisca.yml) file
2. Place it in `{prowlarr_appdata}/Definitions/Custom/`
3. Restart Prowlarr
4. Add the Francisca indexer in Prowlarr

👉 [Complete Prowlarr Guide](./integrations/prowlarr)

### Jackett

Alternative to Prowlarr, simpler but requires manual configuration.

1. Download the [`francisca.yml`](https://github.com/UwUDev/francisca/blob/master/francisca.yml) file
2. Place it in `{jackett_appdata}/cardigann/definitions/`
3. Restart Jackett
4. Add the Francisca indexer in Jackett

👉 [Complete Jackett Guide](./integrations/jackett)

### Direct API Usage

You can also use the REST API directly:

```bash
# Search for a torrent
curl "http://localhost:8715/search?q=breaking+bad&season=1&ep=1"

# Download a torrent
curl -O "http://localhost:8715/download?id=1234567"
```

👉 [Complete API Documentation](./api)

## Quick Troubleshooting

### Service Won't Start

1. Check the logs:
   ```bash
   docker compose logs francisca
   ```

2. Verify port 8715 is available:
   ```bash
   # Linux/Mac
   lsof -i :8715
   
   # Windows
   netstat -ano | findstr :8715
   ```

### No Search Results

**Possible causes:**
1. Nostr relay unreachable → Check logs (`INFO Using Nostr relay: ...`)
2. Query too specific → Try with fewer keywords
3. Misconfigured categories → Check Prowlarr/Jackett configuration

### "Connection Refused" Error

The service is not accessible:

1. Verify the container is running:
   ```bash
   docker ps | grep francisca
   ```

2. Verify the port is properly exposed:
   ```bash
   docker compose ps
   ```

3. Test from within the container:
   ```bash
   docker exec francisca curl http://localhost:8715/health
   ```

## Updates

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

<Tabs groupId="installation-method">
  <TabItem value="docker-compose" label="Docker Compose" default>

```bash
# Download the latest image
docker compose pull

# Restart with the new image
docker compose up -d

# Clean up old images
docker image prune -f
```

  </TabItem>
  <TabItem value="docker-run" label="Docker Run">

```bash
# Stop the current container
docker stop francisca
docker rm francisca

# Download the latest image
docker pull uwucode/francisca:latest

# Recreate the container with the same command as installation
# (reuse your docker run command)

# Clean up old images
docker image prune -f
```

  </TabItem>
  <TabItem value="binary" label="Binary">

```bash
# Stop Francisca
sudo systemctl stop francisca

# Download the new version
wget https://github.com/UwUDev/francisca/releases/latest/download/francisca-linux-amd64

# Replace the binary
sudo mv francisca-linux-amd64 /usr/local/bin/francisca
sudo chmod +x /usr/local/bin/francisca

# Restart
sudo systemctl start francisca
```

  </TabItem>
</Tabs>

### Check installed version

```bash
curl http://localhost:8715/status | jq '.version'
```

## Next Steps

Now that Francisca is installed and configured:

1. 📖 **[Configure Prowlarr](./integrations/prowlarr)** - Automatic synchronization with your \*arr applications
2. 🔧 **[Advanced Configuration](./configuration)** - TMDB/IMDB, logging, etc.
3. 📡 **[Explore the API](./api)** - Use Francisca in your own scripts
4. 🐳 **[Advanced Docker Options](./installation/docker-guide)** - Tags, architectures, health checks

## Need Help?

- 📚 Check the [complete documentation](/)
- 🐛 [Open an issue on GitHub](https://github.com/UwUDev/francisca/issues)
- 💬 Read [existing issues](https://github.com/UwUDev/francisca/issues?q=is%3Aissue)

:::tip Contribution
Francisca is open-source! Feel free to contribute on [GitHub](https://github.com/UwUDev/francisca).
:::
