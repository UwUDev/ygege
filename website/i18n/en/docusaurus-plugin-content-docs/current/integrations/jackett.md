---
sidebar_position: 2
---

# Jackett Integration

Ygégé can be used as a custom indexer for Jackett via the Cardigann system.

## Prerequisites

- Jackett installed and running
- Ygégé started and accessible
- The `ygege.yml` file from the GitHub repository

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

Download and copy the `ygege.yml` file:

```bash
# Download from GitHub
wget https://raw.githubusercontent.com/UwUDev/ygege/master/ygege.yml \
  -O /config/cardigann/definitions/ygege.yml
```

Or manually:
1. Download [`ygege.yml`](https://github.com/UwUDev/ygege/blob/master/ygege.yml)
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
3. Search for "Ygégé" in the list
4. Click the **+** button next to Ygégé

<!-- TODO: Add Jackett list screenshot with Ygégé -->
<!-- ![Jackett Add Indexer](/img/jackett-add-indexer.png) -->

### 2. Configure Settings

<!-- TODO: Add Ygégé configuration form screenshot in Jackett -->
<!-- ![Jackett Ygege Configuration](/img/jackett-ygege-config.png) -->

In the configuration window, enter:

| Parameter | Value | Description |
|-----------|-------|-------------|
| **Indexer URL** | `http://localhost:8715` | Ygégé base URL |
| **Username** | Your YGG username | Optional (if not in config) |
| **Password** | Your YGG password | Optional (if not in config) |

:::info Centralized Configuration
If you've already configured credentials in Ygégé's `config.json`, you don't need to enter them here again.
:::

### 3. Test the Connection

1. Click **OK** to save
2. Jackett will automatically test the connection
3. A success message should appear

## Docker Compose Configuration

If Jackett and Ygégé are in the same `compose.yml`:

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
  
  ygege:
    image: uwucode/ygege:latest
    container_name: ygege
    volumes:
      - ./config:/config
    ports:
      - "8715:8715"
    environment:
      YGG_USERNAME: "your_username"
      YGG_PASSWORD: "your_password"
    restart: unless-stopped
```

In this case, use `http://ygege:8715` as the URL in Jackett configuration.

## Usage

### Manual Search

1. In Jackett, go to the home page
2. Use the search bar
3. Ygégé will appear in the results

### Integration with Sonarr/Radarr

1. Copy the Torznab URL from Jackett (click **Copy Torznab Feed**)
2. In Sonarr/Radarr, add Jackett as an indexer
3. Paste the Torznab URL
4. Ygégé results will be automatically integrated

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

Ygégé supports several search parameters:

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
1. Verify that `ygege.yml` is in `cardigann/definitions/`
2. Check file permissions (must be readable)
3. Restart Jackett
4. Check logs: `docker logs jackett`

### Connection Error

**Solution:**
1. Verify Ygégé is running:
   ```bash
   curl http://localhost:8715/health
   ```
2. Check the configured URL (localhost vs container name)
3. For Docker, verify containers are on the same network

### No Search Results

**Solution:**
1. Test Ygégé API directly:
   ```bash
   curl "http://localhost:8715/api/search?q=test"
   ```
2. Check Ygégé logs:
   ```bash
   docker logs ygege
   ```
3. Verify your YGG credentials are valid

### YGG Rate Limiting

**Solution:**
- Ensure YGG credentials are configured
- Check `config.json` file or environment variables
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
