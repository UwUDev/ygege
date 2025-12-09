---
sidebar_position: 2
---

# Getting Started Guide

This guide walks you step by step through installing and configuring Ygégé, from initial setup to integration with your media management applications.

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
With each release, **pre-compiled binaries** are provided for multiple platforms (Linux, Windows, macOS). Download them directly from the [releases page](https://github.com/UwUDev/ygege/releases).
:::

👉 To compile yourself, see the [GitHub README](https://github.com/UwUDev/ygege#building-from-source)

## Quick Installation (Docker Compose)

### Step 1: Create Configuration Directory

```bash
mkdir -p ~/ygege/config
cd ~/ygege
```

### Step 2: Create compose.yml File

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
      # YGG Torrent credentials (REQUIRED)
      YGG_USERNAME: "your_username"
      YGG_PASSWORD: "your_password"
      
      # Optional configuration
      LOG_LEVEL: "debug"
      BIND_IP: "0.0.0.0"
      BIND_PORT: "8715"
    
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
docker compose logs -f ygege

# Test the API
curl http://localhost:8715/health
```

You should see:
```
[INFO] Configuration loaded successfully
[INFO] Connecting to YGG Torrent...
[INFO] Authentication successful
[INFO] Server started on 0.0.0.0:8715
```

You can also access the information page in your browser: `http://localhost:8715/`

![Ygégé Info Page](/img/ygege-info.png)

This page displays real-time status of all Ygégé components:
- YGG authentication status
- Domain DNS resolution
- Domain reachability
- Search and parser functionality
- TMDB/IMDB integration
- User information

## Basic Configuration

### YGG Torrent Credentials

:::danger IMPORTANT
YGG Torrent is a **private** tracker. Valid credentials are **absolutely required** to use Ygégé. Without them, Ygégé cannot connect.
:::

You have two options to configure your credentials:

**Option 1: Environment Variables (Recommended)**
```yaml
environment:
  YGG_USERNAME: "your_username"
  YGG_PASSWORD: "your_password"
```

**Option 2: config.json File**
```json
{
    "username": "your_username",
    "password": "your_password",
    "bind_ip": "0.0.0.0",
    "bind_port": 8715,
    "log_level": "debug"
}
```

### Network Ports

By default, Ygégé listens on port **8715**. If this port is already in use:

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

Once Ygégé is configured, integrate it with your applications:

### Prowlarr (Recommended)

Prowlarr automatically synchronizes indexers with Sonarr, Radarr, Lidarr, etc.

1. Download the [`ygege.yml`](https://github.com/UwUDev/ygege/blob/master/ygege.yml) file
2. Place it in `{prowlarr_appdata}/Definitions/Custom/`
3. Restart Prowlarr
4. Add the Ygégé indexer in Prowlarr

👉 [Complete Prowlarr Guide](./integrations/prowlarr)

### Jackett

Alternative to Prowlarr, simpler but requires manual configuration.

1. Download the [`ygege.yml`](https://github.com/UwUDev/ygege/blob/master/ygege.yml) file
2. Place it in `{jackett_appdata}/cardigann/definitions/`
3. Restart Jackett
4. Add the Ygégé indexer in Jackett

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
   docker compose logs ygege
   ```

2. Verify port 8715 is available:
   ```bash
   # Linux/Mac
   lsof -i :8715
   
   # Windows
   netstat -ano | findstr :8715
   ```

### YGG Authentication Error

```
[ERROR] YGG authentication failed
```

**Solutions:**
- Verify your YGG credentials
- Log in to the YGG website to verify your account
- Check that your account is not banned or suspended

### No Search Results

**Possible causes:**
1. YGG credentials not configured → You're rate-limited
2. Connection issue with YGG → Check the logs
3. Misconfigured categories → Check Prowlarr/Jackett configuration

### "Connection Refused" Error

The service is not accessible:

1. Verify the container is running:
   ```bash
   docker ps | grep ygege
   ```

2. Verify the port is properly exposed:
   ```bash
   docker compose ps
   ```

3. Test from within the container:
   ```bash
   docker exec ygege curl http://localhost:8715/health
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
docker stop ygege
docker rm ygege

# Download the latest image
docker pull uwucode/ygege:latest

# Recreate the container with the same command as installation
# (reuse your docker run command)

# Clean up old images
docker image prune -f
```

  </TabItem>
  <TabItem value="binary" label="Binary">

```bash
# Stop Ygégé
sudo systemctl stop ygege

# Download the new version
wget https://github.com/UwUDev/ygege/releases/latest/download/ygege-linux-amd64

# Replace the binary
sudo mv ygege-linux-amd64 /usr/local/bin/ygege
sudo chmod +x /usr/local/bin/ygege

# Restart
sudo systemctl start ygege
```

  </TabItem>
</Tabs>

### Check installed version

```bash
curl http://localhost:8715/status | jq '.version'
```

## Next Steps

Now that Ygégé is installed and configured:

1. 📖 **[Configure Prowlarr](./integrations/prowlarr)** - Automatic synchronization with your \*arr applications
2. 🔧 **[Advanced Configuration](./configuration)** - TMDB/IMDB, logging, etc.
3. 📡 **[Explore the API](./api)** - Use Ygégé in your own scripts
4. 🐳 **[Advanced Docker Options](./installation/docker-guide)** - Tags, architectures, health checks

## Need Help?

- 📚 Check the [complete documentation](/)
- 🐛 [Open an issue on GitHub](https://github.com/UwUDev/ygege/issues)
- 💬 Read [existing issues](https://github.com/UwUDev/ygege/issues?q=is%3Aissue)

:::tip Contribution
Ygégé is open-source! Feel free to contribute on [GitHub](https://github.com/UwUDev/ygege).
:::
