---
sidebar_position: 1
---

# Prowlarr Integration

Ygégé can be used as a custom indexer for Prowlarr, allowing you to integrate YGG Torrent into your media management stack.

## Prerequisites

- Prowlarr installed and running
- Ygégé started and accessible
- The `ygege.yml` file from the GitHub repository

## Installation

### 1. Locate Prowlarr's AppData Directory

The AppData directory path is displayed in Prowlarr's `/system/status` page.

![Prowlarr Status](/img/prowlarr-status.png)

Example paths:
- **Linux/Docker**: `/config` or `/data`
- **Windows**: `C:\ProgramData\Prowlarr`
- **macOS**: `~/.config/Prowlarr`

### 2. Create the Custom Folder

In Prowlarr's AppData directory, navigate to `Definitions/` and create a `Custom` folder if it doesn't exist:

```bash
mkdir -p /config/Definitions/Custom
```

### 3. Copy the Definition File

Copy the `ygege.yml` file (French by default, or `ygege-en.yml` for the English version) from the GitHub repository to the `Custom` folder:

```bash
# Download directly from GitHub
wget https://raw.githubusercontent.com/UwUDev/ygege/master/ygege.yml \
  -O /config/Definitions/Custom/ygege.yml
```

Or manually:
1. Download [`ygege.yml`](https://github.com/UwUDev/ygege/blob/master/ygege.yml)
2. Place it in `{appdata}/Definitions/Custom/`

### 4. Restart Prowlarr

Restart Prowlarr to detect the new indexer:

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

<Tabs groupId="runtime">
  <TabItem value="docker" label="Docker" default>

```bash
docker restart prowlarr
```

  </TabItem>
  <TabItem value="systemd" label="Systemd">

```bash
systemctl restart prowlarr
```

  </TabItem>
</Tabs>

## Indexer Configuration

### 1. Add the Indexer

1. Go to **Indexers**
2. Click the **+** button to add an indexer
3. Search for "Ygégé" in the list
4. Click on "Ygégé"

![Prowlarr Add Indexer](/img/prowlarr-add-indexer.png)

### 2. Configure Settings

![Prowlarr Ygege Configuration](/img/prowlarr-ygege-config.png)

| Parameter | Value | Description |
|-----------|-------|-------------|
| **Name** | Ygégé | Indexer name |
| **Enable** | ✅ | Enable the indexer |
| **URL** | `http://localhost:8715/` | Base URL |
| **API Path** | `/api` | API path |
| **Categories** | All | Categories to index |

:::warning Important Base URL
Prowlarr does **not** allow customizing the base URL. Use:
- **Local installation**: `http://localhost:8715/`
- **Docker Compose**: `http://ygege:8715/` (service name)
- **Custom DNS**: `http://ygege-dns-redirect.local:8715/`
:::

### 3. Docker Compose Configuration

If Prowlarr and Ygégé are in the same `compose.yml`:

```yaml
services:
  prowlarr:
    image: lscr.io/linuxserver/prowlarr:latest
    container_name: prowlarr
    # ... prowlarr configuration
  
  ygege:
    image: uwucode/ygege:latest
    container_name: ygege
    # ... ygege configuration

# They're automatically on the same network
# Use http://ygege:8715/ in Prowlarr
```

### 4. Test the Connection

1. Click **Test** in the indexer configuration
2. Prowlarr should connect successfully
3. Click **Save**

## Usage

### Manual Search

1. Go to **Search** in Prowlarr
2. Enter your search query
3. Ygégé will appear in the results

### Synchronization with Sonarr/Radarr

Prowlarr will automatically synchronize the Ygégé indexer with your connected \*arr applications.

## Supported Categories

Ygégé supports all YGG categories:

| Prowlarr Category | YGG Mapping |
|------------------|-------------|
| Movies | Films |
| TV | TV Series |
| Audio | Music |
| PC | Applications |
| XXX | Adult |
| Other | Other |

## Troubleshooting

### Indexer Doesn't Appear

1. Verify that `ygege.yml` is in `Definitions/Custom/`
2. Restart Prowlarr
3. Check Prowlarr logs for errors

### Connection Error

1. Verify that Ygégé is running: `curl http://localhost:8715/health`
2. Check the URL configured in Prowlarr
3. For Docker, verify containers are on the same network

### No Results

1. Check Ygégé logs: `docker logs ygege`
2. Verify your YGG credentials are valid
3. Test the API directly: `curl http://localhost:8715/api/search?q=test`

## Next Steps

- [Advanced Configuration](../configuration)
- [API Documentation](../api)
- [Jackett Integration](./jackett)
