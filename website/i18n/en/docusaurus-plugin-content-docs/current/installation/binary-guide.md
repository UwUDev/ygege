---
sidebar_position: 2
---

# Installation with Pre-compiled Binaries

This guide explains how to install and use Francisca with pre-compiled binaries provided with each release.

## Prerequisites

- Supported operating system: Linux, Windows, macOS
- No external dependencies required (static binaries)

## Download

### Option 1: From GitHub Releases (Recommended)

1. Go to the [releases page](https://github.com/UwUDev/francisca/releases)
2. Download the binary for your platform:
   - **Linux AMD64**: `francisca-linux-x86_64`
   - **Linux ARM64**: `francisca-linux-aarch64`
   - **Linux ARMv7**: `francisca-linux-armv7`
   - **Windows AMD64**: `francisca-windows-x86_64.exe`
   - **macOS Intel**: `francisca-macos-x86_64`
   - **macOS Apple Silicon**: `francisca-macos-aarch64`

### Option 2: Via wget/curl (Linux/macOS)

```bash
# Replace VERSION with the desired version (e.g., v1.0.0)
# Replace PLATFORM with your platform (e.g., linux-x86_64)
wget https://github.com/UwUDev/francisca/releases/download/VERSION/francisca-PLATFORM

# Or with curl
curl -L -o francisca https://github.com/UwUDev/francisca/releases/download/VERSION/francisca-PLATFORM
```

## Installation

### Linux / macOS

```bash
# Make the binary executable
chmod +x francisca-*

# Move to a PATH folder (optional)
sudo mv francisca-* /usr/local/bin/francisca

# Verify installation
francisca --version
```

### Windows

1. Create a folder `C:\Program Files\Francisca\`
2. Move `francisca-windows-x86_64.exe` to this folder
3. Rename it to `francisca.exe`
4. Add the folder to PATH (optional)

## Configuration

### Create configuration file

Create a `config.json` file in the same folder as the binary:

```json
{
  "bind_ip": "0.0.0.0",
  "bind_port": 8715,
  "log_level": "info",
  "tmdb_token": null,
  "use_tor": false,
  "tor_proxy": "127.0.0.1:9050"
}
```

:::info No authentication required
The Nostr relay can be public. Depending on the relay, no credentials are needed.
:::

### Configuration via environment variables

You can also use environment variables:

```bash
export BIND_PORT="8715"
export LOG_LEVEL="info"
# export TMDB_TOKEN="your_token"  # Optional
```

## Launch

### Simple launch

```bash
# Linux/macOS
./francisca

# Windows (PowerShell)
.\francisca.exe
```

The server starts on `http://localhost:8715`

### Background launch (Linux/macOS)

```bash
# With nohup
nohup ./francisca > francisca.log 2>&1 &

# With screen
screen -S francisca
./francisca
# Ctrl+A then D to detach
```

### Systemd service (Linux)

Create `/etc/systemd/system/francisca.service`:

```ini
[Unit]
Description=Francisca - U2P System Indexer
After=network.target

[Service]
Type=simple
User=youruser
WorkingDirectory=/opt/francisca
ExecStart=/usr/local/bin/francisca
Restart=on-failure
RestartSec=5s

Environment="LOG_LEVEL=info"

[Install]
WantedBy=multi-user.target
```

Enable and start the service:

```bash
sudo systemctl daemon-reload
sudo systemctl enable francisca
sudo systemctl start francisca
sudo systemctl status francisca
```

### Windows scheduled task

1. Open Task Scheduler
2. Create a new basic task
3. Configure:
   - **Trigger**: At startup
   - **Action**: Start a program → `C:\Program Files\Francisca\francisca.exe`
   - **Conditions**: Uncheck "Start only on AC power"

## Update

### Manual method

1. Download the new binary from releases
2. Stop Francisca (`systemctl stop francisca` or `Ctrl+C`)
3. Replace the old binary
4. Restart (`systemctl start francisca` or relaunch)

### Update script (Linux)

```bash
#!/bin/bash
LATEST=$(curl -s https://api.github.com/repos/UwUDev/francisca/releases/latest | grep tag_name | cut -d '"' -f 4)
PLATFORM="linux-x86_64" # Change according to your platform

echo "Downloading Francisca $LATEST..."
wget -O francisca.new "https://github.com/UwUDev/francisca/releases/download/$LATEST/francisca-$PLATFORM"

chmod +x francisca.new
sudo systemctl stop francisca
sudo mv francisca.new /usr/local/bin/francisca
sudo systemctl start francisca

echo "Update completed to $LATEST"
```

## Verification

Test that the service is working:

```bash
curl http://localhost:8715/health
```

Expected response:
```
OK
```

For detailed status:
```bash
curl http://localhost:8715/status
```

Response:
```json
{
  "relay": "wss://relay.example.org",
  "search": "ok",
  "parsing": "ok",
  "tmdb_integration": "disabled"
}
```

## Troubleshooting

### "Permission denied" (Linux/macOS)

```bash
chmod +x francisca
```

### "Port already in use"

Change the port in `config.json` or via the `BIND_PORT` variable.

### Debug logs

```bash
export LOG_LEVEL="debug"
./francisca
```

### Binary doesn't start on older architectures

Use the `noupx` version available in release assets (without UPX compression).

## Building from source

If no pre-compiled binary matches your platform, see the [build guide](https://github.com/UwUDev/francisca#building-from-source).

## Next steps

Once Francisca is installed and running:

1. [Configure advanced options](../configuration)
2. [Integrate with Prowlarr](../integrations/prowlarr) or [Jackett](../integrations/jackett)
3. [Explore the API](../api)
