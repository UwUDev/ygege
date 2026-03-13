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
    "use_tor": false,
    "tor_proxy": "127.0.0.1:9050"
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

### Tor Support

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `use_tor` | boolean | `false` | Route relay connections through Tor |
| `tor_proxy` | string | `127.0.0.1:9050` | SOCKS5 Tor proxy address |

:::info
When `use_tor` is enabled, all Nostr relay connections are routed through the Tor proxy. Tor must be installed and running on your machine.
:::

<<<<<<< HEAD
=======
### Outbound HTTP Proxy

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `proxy_url` | string | `null` | Outbound HTTP(S) proxy URL |
| `proxy_username` | string | `null` | Proxy username |
| `proxy_password` | string | `null` | Proxy password |

:::info
This proxy applies to outbound HTTP(S) requests, for example TMDB/IMDB, and also to WebSocket connections to Nostr relays when `use_tor` is disabled. For Nostr relays, use an HTTP proxy URL such as `http://...`. If `use_tor` is enabled, `tor_proxy` remains the preferred relay transport.
:::

>>>>>>> fd3f5c5 (Enhance proxy configuration support for Nostr relay connections and update documentation)
## Environment Variables

All options can also be set via environment variables:

| Variable | config.json equivalent |
|----------|------------------------|
| `BIND_IP` | `bind_ip` |
| `BIND_PORT` | `bind_port` |
| `LOG_LEVEL` | `log_level` |
| `TMDB_TOKEN` | `tmdb_token` |
| `USE_TOR` | `use_tor` |
| `TOR_PROXY` | `tor_proxy` |

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
      TMDB_TOKEN: "your_tmdb_token"  # Optional
      # USE_TOR: "true"               # Optional: enable Tor
      # TOR_PROXY: "127.0.0.1:9050"   # Optional: alternative Tor proxy
```

### For config.json File

```json
{
    "bind_ip": "0.0.0.0",
    "bind_port": 8715,
    "log_level": "info",
    "tmdb_token": "your_tmdb_token",
    "use_tor": false,
    "tor_proxy": "127.0.0.1:9050"
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
INFO Tor routing disabled — connecting to relays directly
INFO Ranking Nostr relays by latency...
INFO Relay order: 1. wss://relay.ygg.gratis
INFO Categories initialized: 9 top-level categories
```

## Next Steps

- [API Documentation](./api)
- [Prowlarr Integration](./integrations/prowlarr)
