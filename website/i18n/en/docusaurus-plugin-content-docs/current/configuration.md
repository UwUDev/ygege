---
sidebar_position: 5
sidebar_label: Configuration
---

# Configuration

This guide details all available configuration options for Ygégé.

## config.json File

The main configuration file is `config.json`. It should be placed in the `/config` folder (Docker) or at the project root (manual installation).

### Complete Structure

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

## Available Options

### YGG Authentication

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `username` | string | ✅ | YGG Torrent username |
| `password` | string | ✅ | YGG Torrent password |

:::warning Warning
Without valid credentials, you will be **rate-limited** by YGG and the service won't work properly.
:::

### Network Configuration

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `bind_ip` | string | `0.0.0.0` | Listening IP address |
| `bind_port` | number | `8715` | Server listening port |

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
| `tmdb_api_key` | string | `""` | TMDB API key (optional) |

## Environment Variables

All options can also be set via environment variables:

| Variable | config.json equivalent |
|----------|------------------------|
| `YGG_USERNAME` | `username` |
| `YGG_PASSWORD` | `password` |
| `BIND_IP` | `bind_ip` |
| `BIND_PORT` | `bind_port` |
| `LOG_LEVEL` | `log_level` |
| `TMDB_TOKEN` | `tmdb_token` |

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
    volumes:
      - ./config:/config
    environment:
      YGG_USERNAME: "my_username"
      YGG_PASSWORD: "my_password"
      LOG_LEVEL: "info"
      TMDB_TOKEN: "your_tmdb_token"
```

### For config.json File

```json
{
    "username": "my_username",
    "password": "my_password",
    "bind_ip": "0.0.0.0",
    "bind_port": 8715,
    "log_level": "debug",
    "tmdb_token": "your_tmdb_token"
}
```

## Configuration Validation

To verify your configuration is correct, check the logs at startup:

```bash
docker logs ygege
```

You should see:
```
[INFO] Configuration loaded successfully
[INFO] Connecting to YGG Torrent...
[INFO] Authentication successful
[INFO] Server started on 0.0.0.0:8715
```

## Next Steps

- [API Documentation](./api)
- [Prowlarr Integration](./integrations/prowlarr)
