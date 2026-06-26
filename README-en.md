# Francisca

<p align="center">
  <img src="website/img/francisca-logo-text.png" alt="Francisca Logo" width="400"/>
</p>

- [Français](README-fr.md)

General-purpose, high-performance indexer for any system that supports the Nostr protocol (NIP-35 torrent events), written in Rust

## [LEGAL DISCLAIMER](DISCLAIMER.md)

**Key Features**:
- Works with any Nostr relay exposing NIP-35 torrent events
- Fully configurable relays — no account or authentication required
- Automatic relay ranking by latency at startup
- Near-instant search
- Low memory usage
- Modular torrent search (by name, seeders, leechers, release date, etc.)
- Optional Tor support to anonymize relay connections
- TMDB/IMDB integration for ID-based lookups
- Compatible with Prowlarr, Jackett, and all \*arr applications

## Configuration (required before first run)

Francisca ships with **no default relay**: you must provide at least one (a Nostr relay exposing NIP-35 torrent events).

**Via `config.json`** (generated on first run, then edit it):

```json
{
  "relays": ["wss://u2p.anhkagi.net"]
}
```

**Or via environment variables** (Docker):

```yaml
environment:
  RELAYS: "wss://u2p.anhkagi.net"   # multiple relays: comma-separated
```

| Key (`config.json` / env) | Required | Description |
|---------------------------|----------|-------------|
| `relays` / `RELAYS` | **yes** | Nostr (NIP-35) relays to query. `wss://` (clearnet) or `ws://…onion` (Tor). Multiple = JSON array, or comma-separated list in env. |
| `allowed_pubkeys` / `ALLOWED_PUBKEYS` | no | Trusted publisher pubkeys (hex). Defaults to the U2P network publisher. `[]` = accept any valid-signature event. |
| `web_base_url` / `WEB_BASE_URL` | no | Base URL for a per-result "details" link. Empty = no link. |

> [!NOTE]
> Without a configured relay, the app stops at startup with a message explaining how to add one.

## Compilation Requirements
- Rust 1.85.0+

# Installation

A ready-to-use Docker image is available for Francisca.
To get started with Docker deployment and configuration, see the [dedicated Docker guide](https://francisca.lila.ws/en/installation/docker-guide).

> [!IMPORTANT]
> If you encounter a `Permission denied` error after updating, see the [Permission Management](https://francisca.lila.ws/en/installation/docker-guide#permission-management) section in the Docker guide.

## Docker

To create a custom Docker image with your own optimizations, refer to the [Docker build guide](https://francisca.lila.ws/en/installation/docker-guide).

## Manual Installation

To compile the application from sources, follow the [manual installation guide](https://francisca.lila.ws/en/installation/source-guide).

## IMDB and TMDB configuration

To enable IMDB and TMDB metadata fetching, please follow the instructions in the [TMDB and IMDB support guide](https://francisca.lila.ws/en/tmdb-imdb).

## Tor Support

Francisca can route its Nostr relay connections through Tor to anonymize traffic.

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

## Prowlarr integration

Francisca can be used as a custom indexer for Prowlarr. To set it up, find your AppData directory (located in the `/system/status` page of Prowlarr) and copy the `francisca.yml` file on the repo in the `{your prowlarr appdata path}/Definitions/Custom` folder, you'll probably need to create the `Custom` folder.

Once it's done, restart Prowlarr and go to the indexer settings, you should see Francisca in the list of available indexers.

> [!NOTE]
> Prowlarr doesn't allow custom "Base URL". By default the URL is `http://localhost:8715/`. For Docker Compose setups, use `http://francisca:8715/`. Alternatively, use francisca-dns-redirect.local with custom DNS or hosts file redirection.

## Jackett Integration

Francisca can be used as a custom indexer for Jackett. To set it up, locate your Jackett AppData directory and copy the `francisca.yml` file from the repository into the `{your jackett appdata path}/cardigann/definitions/` folder. You may need to create the `cardigann/definitions/` subfolder if it doesn't exist.

> [!NOTE]
> The LinuxServer Jackett image provides a well-organized folder structure. If you're using a different Docker image, adjust the paths accordingly.

Once complete, restart Jackett and navigate to the indexer settings. You should see Francisca listed among the available indexers.

# API Documentation

The API documentation is available [here](https://francisca.lila.ws/en/api).
