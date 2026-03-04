---
sidebar_position: 5
sidebar_label: Configuration
---

# Configuration

This guide details all available configuration options for Ygégé.

## config.json File

The main configuration file is `config.json`. It should be placed at the project root (manual installation) or mounted via a Docker volume.

### Complete Structure

```json
{
    "bind_ip": "0.0.0.0",
    "bind_port": 8715,
    "log_level": "info",
    "tmdb_token": null,
    "relay_url": null
}
```

## Available Options

### Network Configuration

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `bind_ip` | string | `0.0.0.0` | Listening IP address |
| `bind_port` | number | `8715` | Server listening port |

:::tip Custom Port
To avoid port conflicts (e.g., on Windows), simply change `BIND_PORT`:
```yaml
environment:
  BIND_PORT: "3000"  # Use port 3000 instead of 8715
ports:
  - "3000:3000"
```
The healthcheck automatically adapts using `$${BIND_PORT:-8715}`.
:::

### Logging

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `log_level` | string | `info` | Log verbosity level |

Available levels:
- `trace`: Maximum details (development)
- `debug`: Debug information
- `info`: General information
- `warn`: Warnings only
- `error`: Errors only

### TMDB/IMDB Metadata

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `tmdb_token` | string | `null` | TMDB API key (optional) |

:::info
When `tmdb_token` is configured, both **TMDB and IMDB** resolvers are automatically enabled.
:::

### Nostr Relay

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `relay_url` | string | `wss://relay.ygg.gratis` | Nostr relay WebSocket URL |

:::tip When to use RELAY_URL?
By default, Ygégé connects to the official relay at `wss://relay.ygg.gratis`. To use an alternative relay or mirror, specify its WebSocket URL:
```
RELAY_URL=wss://relay.ygg.gratis
```
:::

## Environment Variables

All options can also be set via environment variables:

| Variable | config.json equivalent |
|----------|------------------------|
| `BIND_IP` | `bind_ip` |
| `BIND_PORT` | `bind_port` |
| `LOG_LEVEL` | `log_level` |
| `TMDB_TOKEN` | `tmdb_token` |
| `RELAY_URL` | `relay_url` |

:::tip Priority
Environment variables have **priority** over config.json file.
:::

## Complete Configuration Example

### For Docker Compose

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
      TMDB_TOKEN: "your_tmdb_token"
      # RELAY_URL: "wss://relay.ygg.gratis"  # Optional: alternative Nostr relay
```

### For config.json File

```json
{
    "bind_ip": "0.0.0.0",
    "bind_port": 8715,
    "log_level": "info",
    "tmdb_token": "your_tmdb_token",
    "relay_url": null
}
```

## Configuration Validation

To verify your configuration is correct, check the logs at startup:

```bash
docker logs ygege
```

You should see:
```
INFO Ygégé v0.x.x (commit: ..., branch: ..., built: ...)
INFO Using Nostr relay: wss://relay.ygg.gratis
INFO Categories initialized: 9 top-level categories
```

## Next Steps

- [API Documentation](./api)
- [Prowlarr Integration](./integrations/prowlarr)
