---
sidebar_position: 1
---

# Prowlarr Integration

Francisca can be used as a custom indexer for Prowlarr, allowing you to integrate U2P system into your media management stack.

## Prerequisites

- Prowlarr installed and running
- Francisca started and accessible
- The `francisca.yml` file from the GitHub repository

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

Copy the `francisca.yml` file (French by default, or `francisca-en.yml` for the English version) from the GitHub repository to the `Custom` folder:

```bash
# Download directly from GitHub
wget https://raw.githubusercontent.com/UwUDev/francisca/master/francisca.yml \
  -O /config/Definitions/Custom/francisca.yml
```

Or manually:
1. Download [`francisca.yml`](https://github.com/UwUDev/francisca/blob/master/francisca.yml)
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
3. Search for "Francisca" in the list
4. Click on "Francisca"

![Prowlarr Add Indexer](/img/prowlarr-add-indexer.png)

### 2. Configure Settings

![Prowlarr Francisca Configuration](/img/prowlarr-francisca-config.png)

| Parameter | Value | Description |
|-----------|-------|-------------|
| **Name** | Francisca | Indexer name |
| **Enable** | ✅ | Enable the indexer |
| **URL** | `http://localhost:8715/` | Base URL |
| **API Path** | `/api` | API path |
| **Categories** | All | Categories to index |

:::warning Important Base URL
Prowlarr does **not** allow customizing the base URL. Use:
- **Local installation**: `http://localhost:8715/`
- **Docker Compose**: `http://francisca:8715/` (service name)
- **Custom DNS**: `http://francisca-dns-redirect.local:8715/`
:::

### 3. Docker Compose Configuration

If Prowlarr and Francisca are in the same `compose.yml`:

```yaml
services:
  prowlarr:
    image: lscr.io/linuxserver/prowlarr:latest
    container_name: prowlarr
    # ... prowlarr configuration
  
  francisca:
    image: uwucode/francisca:latest
    container_name: francisca
    # ... francisca configuration

# They're automatically on the same network
# Use http://francisca:8715/ in Prowlarr
```

### 4. Test the Connection

1. Click **Test** in the indexer configuration
2. Prowlarr should connect successfully
3. Click **Save**

## Usage

### Manual Search

1. Go to **Search** in Prowlarr
2. Enter your search query
3. Francisca will appear in the results

### Synchronization with Sonarr/Radarr

Prowlarr will automatically synchronize the Francisca indexer with your connected \*arr applications.

## Supported Categories

Francisca supports all available categories:

| Prowlarr Category | Category Mapping |
|------------------|-------------|
| Movies | Films |
| TV | TV Series |
| Audio | Music |
| PC | Applications |
| XXX | Adult |
| Other | Other |

## Troubleshooting

### Indexer Doesn't Appear

1. Verify that `francisca.yml` is in `Definitions/Custom/`
2. Restart Prowlarr
3. Check Prowlarr logs for errors

### Connection Error

1. Verify that Francisca is running: `curl http://localhost:8715/health`
2. Check the URL configured in Prowlarr
3. For Docker, verify containers are on the same network

### No Results

1. Check Francisca logs: `docker logs francisca`
2. Verify the Nostr relay is accessible: `curl http://localhost:8715/status`
3. Test the API directly: `curl "http://localhost:8715/search?q=test"`

## Next Steps

- [Advanced Configuration](../configuration)
- [API Documentation](../api)
- [Jackett Integration](./jackett)
