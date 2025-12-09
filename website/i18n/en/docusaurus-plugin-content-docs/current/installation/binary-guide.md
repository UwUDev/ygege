---
sidebar_position: 2
---

# Installation with Pre-compiled Binaries

This guide explains how to install and use Ygégé with pre-compiled binaries provided with each release.

## Prerequisites

- Supported operating system: Linux, Windows, macOS
- No external dependencies required (static binaries)

## Download

### Option 1: From GitHub Releases (Recommended)

1. Go to the [releases page](https://github.com/UwUDev/ygege/releases)
2. Download the binary for your platform:
   - **Linux AMD64**: `ygege-linux-x86_64`
   - **Linux ARM64**: `ygege-linux-aarch64`
   - **Linux ARMv7**: `ygege-linux-armv7`
   - **Windows AMD64**: `ygege-windows-x86_64.exe`
   - **macOS Intel**: `ygege-macos-x86_64`
   - **macOS Apple Silicon**: `ygege-macos-aarch64`

### Option 2: Via wget/curl (Linux/macOS)

```bash
# Replace VERSION with the desired version (e.g., v1.0.0)
# Replace PLATFORM with your platform (e.g., linux-x86_64)
wget https://github.com/UwUDev/ygege/releases/download/VERSION/ygege-PLATFORM

# Or with curl
curl -L -o ygege https://github.com/UwUDev/ygege/releases/download/VERSION/ygege-PLATFORM
```

## Installation

### Linux / macOS

```bash
# Make the binary executable
chmod +x ygege-*

# Move to a PATH folder (optional)
sudo mv ygege-* /usr/local/bin/ygege

# Verify installation
ygege --version
```

### Windows

1. Create a folder `C:\Program Files\Ygege\`
2. Move `ygege-windows-x86_64.exe` to this folder
3. Rename it to `ygege.exe`
4. Add the folder to PATH (optional)

## Configuration

### Create configuration file

Create a `config.json` file in the same folder as the binary:

```json
{
  "username": "your_ygg_username",
  "password": "your_password",
  "bind_ip": "0.0.0.0",
  "bind_port": 8715,
  "log_level": "debug",
  "tmdb_token": null
}
```

:::danger Required credentials
YGG Torrent is a private tracker. Valid credentials are **absolutely required**.
:::

### Configuration via environment variables

You can also use environment variables:

```bash
export YGG_USERNAME="your_username"
export YGG_PASSWORD="your_password"
export BIND_PORT="8715"
export LOG_LEVEL="debug"
```

## Launch

### Simple launch

```bash
# Linux/macOS
./ygege

# Windows (PowerShell)
.\ygege.exe
```

The server starts on `http://localhost:8715`

### Background launch (Linux/macOS)

```bash
# With nohup
nohup ./ygege > ygege.log 2>&1 &

# With screen
screen -S ygege
./ygege
# Ctrl+A then D to detach
```

### Systemd service (Linux)

Create `/etc/systemd/system/ygege.service`:

```ini
[Unit]
Description=Ygégé - YGG Torrent Indexer
After=network.target

[Service]
Type=simple
User=youruser
WorkingDirectory=/opt/ygege
ExecStart=/usr/local/bin/ygege
Restart=on-failure
RestartSec=5s

Environment="YGG_USERNAME=your_username"
Environment="YGG_PASSWORD=your_password"

[Install]
WantedBy=multi-user.target
```

Enable and start the service:

```bash
sudo systemctl daemon-reload
sudo systemctl enable ygege
sudo systemctl start ygege
sudo systemctl status ygege
```

### Windows scheduled task

1. Open Task Scheduler
2. Create a new basic task
3. Configure:
   - **Trigger**: At startup
   - **Action**: Start a program → `C:\Program Files\Ygege\ygege.exe`
   - **Conditions**: Uncheck "Start only on AC power"

## Update

### Manual method

1. Download the new binary from releases
2. Stop Ygégé (`systemctl stop ygege` or `Ctrl+C`)
3. Replace the old binary
4. Restart (`systemctl start ygege` or relaunch)

### Update script (Linux)

```bash
#!/bin/bash
LATEST=$(curl -s https://api.github.com/repos/UwUDev/ygege/releases/latest | grep tag_name | cut -d '"' -f 4)
PLATFORM="linux-x86_64" # Change according to your platform

echo "Downloading Ygégé $LATEST..."
wget -O ygege.new "https://github.com/UwUDev/ygege/releases/download/$LATEST/ygege-$PLATFORM"

chmod +x ygege.new
sudo systemctl stop ygege
sudo mv ygege.new /usr/local/bin/ygege
sudo systemctl start ygege

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
  "auth": "authenticated",
  "domain": "www.**********",
  "domain_dns": "resolves",
  "domain_reachability": "reachable",
  "parsing": "ok",
  "search": "ok",
  "user_info": "ok"
}
```

## Troubleshooting

### "Permission denied" (Linux/macOS)

```bash
chmod +x ygege
```

### "Port already in use"

Change the port in `config.json` or via the `BIND_PORT` variable.

### Debug logs

```bash
export LOG_LEVEL="debug"
./ygege
```

### Binary doesn't start on older architectures

Use the `noupx` version available in release assets (without UPX compression).

## Building from source

If no pre-compiled binary matches your platform, see the [build guide](https://github.com/UwUDev/ygege#building-from-source).

## Next steps

Once Ygégé is installed and running:

1. [Configure advanced options](../configuration)
2. [Integrate with Prowlarr](../integrations/prowlarr) or [Jackett](../integrations/jackett)
3. [Explore the API](../api)
