# Ygégé

<p align="center">
  <img src="website/img/ygege-logo-text.png" alt="Ygégé Logo" width="400"/>
</p>

- [Français](README-fr.md)

High-performance indexer for [ygg.gratis](https://ygg.gratis) via the Nostr protocol, written in Rust

## [LEGAL DISCLAIMER](DISCLAIMER.md)

**Key Features**:
- Connects to the ygg.gratis Nostr relay (`wss://relay.ygg.gratis`)
- No account or credentials required — ygg.gratis is public
- Automatic relay ranking by latency at startup
- Near-instant search
- Low memory usage
- Modular torrent search (by name, seeders, leechers, release date, etc.)
- Optional Tor support to anonymize relay connections
- TMDB/IMDB integration for ID-based lookups
- Compatible with Prowlarr, Jackett, and all \*arr applications

## Compilation Requirements
- Rust 1.85.0+

# Installation

A ready-to-use Docker image is available for Ygégé.
To get started with Docker deployment and configuration, see the [dedicated Docker guide](https://ygege.lila.ws/en/installation/docker-guide).

> [!IMPORTANT]
> If you encounter a `Permission denied` error after updating, see the [Permission Management](https://ygege.lila.ws/en/installation/docker-guide#permission-management) section in the Docker guide.

## Docker

To create a custom Docker image with your own optimizations, refer to the [Docker build guide](https://ygege.lila.ws/en/installation/docker-guide).

## Manual Installation

To compile the application from sources, follow the [manual installation guide](https://ygege.lila.ws/en/installation/source-guide).

## IMDB and TMDB configuration

To enable IMDB and TMDB metadata fetching, please follow the instructions in the [TMDB and IMDB support guide](https://ygege.lila.ws/en/tmdb-imdb).

## Tor Support

Ygégé can route its Nostr relay connections through Tor to anonymize traffic.

| Environment Variable | Default | Description |
|----------------------|---------|-------------|
| `USE_TOR` | `false` | Enable Tor routing (`true`/`false`) |
| `TOR_PROXY` | `127.0.0.1:9050` | SOCKS5 Tor proxy address |

Docker Compose example:

```yaml
environment:
  USE_TOR: "true"
  TOR_PROXY: "127.0.0.1:9050"  # Optional if using default
```

> [!NOTE]
> Tor must be installed and running on your machine (or accessible from the container) for this option to work.

<<<<<<< HEAD
=======
## HTTP Proxy Support

Ygégé can also use an outbound HTTP(S) proxy for its HTTP requests, in the same spirit as Flaresolverr.

| Environment Variable | Default | Description |
|----------------------|---------|-------------|
| `PROXY_URL` | - | HTTP(S) proxy URL, for example `http://127.0.0.1:8080` |
| `PROXY_USERNAME` | - | Proxy username |
| `PROXY_PASSWORD` | - | Proxy password |

Docker Compose example:

```yaml
environment:
  PROXY_URL: "http://127.0.0.1:8080"
  PROXY_USERNAME: "testuser"   # Optional
  PROXY_PASSWORD: "testpass"   # Optional
```

> [!NOTE]
> This proxy applies to outbound HTTP(S) requests such as TMDB/IMDB, and also to WebSocket connections to Nostr relays when `USE_TOR` is disabled. For Nostr relays, use an HTTP proxy URL such as `http://...`. If `USE_TOR=true`, Tor remains the preferred transport for relay connections.

>>>>>>> fd3f5c5 (Enhance proxy configuration support for Nostr relay connections and update documentation)
## Prowlarr integration

Ygégé can be used as a custom indexer for Prowlarr. To set it up, find your AppData directory (located in the `/system/status` page of Prowlarr) and copy the `ygege.yml` file on the repo in the `{your prowlarr appdata path}/Definitions/Custom` folder, you'll probably need to create the `Custom` folder.

Once it's done, restart Prowlarr and go to the indexer settings, you should see Ygégé in the list of available indexers.

> [!NOTE]
> Prowlarr doesn't allow custom "Base URL". By default the URL is `http://localhost:8715/`. For Docker Compose setups, use `http://ygege:8715/`. Alternatively, use ygege-dns-redirect.local with custom DNS or hosts file redirection.

## Jackett Integration

Ygégé can be used as a custom indexer for Jackett. To set it up, locate your Jackett AppData directory and copy the `ygege.yml` file from the repository into the `{your jackett appdata path}/cardigann/definitions/` folder. You may need to create the `cardigann/definitions/` subfolder if it doesn't exist.

> [!NOTE]
> The LinuxServer Jackett image provides a well-organized folder structure. If you're using a different Docker image, adjust the paths accordingly.

Once complete, restart Jackett and navigate to the indexer settings. You should see Ygégé listed among the available indexers.

# API Documentation

The API documentation is available [here](https://ygege.lila.ws/en/api).
