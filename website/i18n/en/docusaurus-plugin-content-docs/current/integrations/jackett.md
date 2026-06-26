---
sidebar_position: 2
---

# Jackett Integration

Francisca can be used as a custom indexer for Jackett via the Cardigann system.

## Prerequisites

- Jackett installed and running
- Francisca started and accessible
- The `francisca.yml` file from the GitHub repository

## Installation

### 1. Locate Jackett's AppData Directory

The path depends on your installation:

| Installation | AppData Path |
|--------------|--------------|
| **LinuxServer Docker** | `/config` |
| **Windows** | `C:\ProgramData\Jackett` |
| **Linux** | `~/.config/Jackett` |
| **macOS** | `~/Library/Application Support/Jackett` |

### 2. Create the Cardigann Structure

In the AppData directory, create the `cardigann/definitions/` structure if it doesn't exist:

```bash
mkdir -p /config/cardigann/definitions
```

### 3. Copy the Definition File

Download and copy the `francisca.yml` file:

```bash
# Download from GitHub
wget https://raw.githubusercontent.com/UwUDev/francisca/master/francisca.yml \
  -O /config/cardigann/definitions/francisca.yml
```

Or manually:
1. Download [`francisca.yml`](https://github.com/UwUDev/francisca/blob/master/francisca.yml)
2. Place it in `{appdata}/cardigann/definitions/`

:::tip LinuxServer Docker
The LinuxServer Jackett image already provides a well-organized folder structure. If you're using a different Docker image, adjust the paths accordingly.
:::

### 4. Restart Jackett

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

<Tabs groupId="runtime">
  <TabItem value="docker" label="Docker" default>

```bash
docker restart jackett
```

  </TabItem>
  <TabItem value="systemd" label="Systemd">

```bash
systemctl restart jackett
```

  </TabItem>
</Tabs>

## Indexer Configuration

### 1. Add the Indexer

1. Open the Jackett interface
2. Click **Add indexer**
3. Search for "Francisca" in the list
4. Click the **+** button next to Francisca

<!-- TODO: Add Jackett list screenshot with Francisca -->
<!-- ![Jackett Add Indexer](/img/jackett-add-indexer.png) -->

### 2. Configure Settings

<!-- TODO: Add Francisca configuration form screenshot in Jackett -->
<!-- ![Jackett Francisca Configuration](/img/jackett-francisca-config.png) -->

In the configuration window, enter:

| Parameter | Value | Description |
|-----------|-------|-------------|
| **Indexer URL** | `http://localhost:8715` | Francisca base URL |

:::info Centralized Configuration
If you've already configured credentials in Francisca's `config.json`, you don't need to enter them here again.
:::

### 3. Test the Connection

1. Click **OK** to save
2. Jackett will automatically test the connection
3. A success message should appear

## Docker Compose Configuration

If Jackett and Francisca are in the same `compose.yml`:

```yaml
services:
  jackett:
    image: lscr.io/linuxserver/jackett:latest
    container_name: jackett
    volumes:
      - ./jackett:/config
    ports:
      - "9117:9117"
    restart: unless-stopped
  
  francisca:
    image: uwucode/francisca:latest
    container_name: francisca
    volumes:
      - ./config:/config
    ports:
      - "8715:8715"
    environment:
      LOG_LEVEL: "info"
    restart: unless-stopped
```

In this case, use `http://francisca:8715` as the URL in Jackett configuration.

## Usage

### Manual Search

1. In Jackett, go to the home page
2. Use the search bar
3. Francisca will appear in the results

### Integration with Sonarr/Radarr

1. Copy the Torznab URL from Jackett (click **Copy Torznab Feed**)
2. In Sonarr/Radarr, add Jackett as an indexer
3. Paste the Torznab URL
4. Francisca results will be automatically integrated

## Supported Categories

| Category ID | Name | Description |
|-------------|------|-------------|
| 2000 | Movies | Movies |
| 5000 | TV | TV Series |
| 3000 | Audio | Music |
| 4000 | PC | Applications/Software |
| 6000 | XXX | Adult content |
| 8000 | Other | Other |

## Advanced Search

Francisca supports several search parameters:

### By Name
```
Moana 2
```

### By Category
Select categories in the Jackett interface

### By Season/Episode (TV)
```
Breaking Bad S01E01
```

### By IMDB ID
```
tt0903747
```

## Troubleshooting

### Indexer Doesn't Appear in the List

**Solution:**
1. Verify that `francisca.yml` is in `cardigann/definitions/`
2. Check file permissions (must be readable)
3. Restart Jackett
4. Check logs: `docker logs jackett`

### Connection Error

**Solution:**
1. Verify Francisca is running:
   ```bash
   curl http://localhost:8715/health
   ```
2. Check the configured URL (localhost vs container name)
3. For Docker, verify containers are on the same network

### No Search Results

**Solution:**
1. Test Francisca API directly:
   ```bash
   curl "http://localhost:8715/api/search?q=test"
   ```
2. Check Francisca logs:
   ```bash
   docker logs francisca
   ```
3. Verify the Nostr relay is accessible: `curl http://localhost:8715/status`

### No Results

**Solution:**
- Verify the Nostr relay is accessible: `curl http://localhost:8715/status`
- See [configuration documentation](../configuration)

## Prowlarr vs Jackett Comparison

| Feature | Prowlarr | Jackett |
|---------|----------|---------|
| \*arr Sync | ✅ Automatic | ❌ Manual |
| Modern UI | ✅ | ❌ |
| Configuration | More complex | Simpler |
| Performance | Better | Good |
| **Recommendation** | **Preferred** | Alternative |

:::tip Recommendation
We recommend **Prowlarr** for better integration with Sonarr/Radarr.
:::

## Next Steps

- [Prowlarr Integration](./prowlarr)
- [Advanced Configuration](../configuration)
- [API Documentation](../api)
