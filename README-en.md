# Ygégé

<p align="center">
  <img src="website/img/ygege-logo-text.png" alt="Ygégé Logo" width="400"/>
</p>

- [Français](README-fr.md)

High-performance indexer for YGG Torrent written in Rust

**Key Features**:
- Automatic resolution of the current YGG Torrent domain
- Automated Cloudflare bypass (no manual challenge solving)
- Near-instant search
- Seamless reconnection to expired sessions
- Session caching
- Bypassing lying DNS
- Low memory usage (14.7MB in Linux release mode)
- Modular torrent search (by name, seeders, leechers, comments, release date, etc.)
- Detailed torrent metadata retrieval (description, size, seeders, leechers, etc.)
- Zero external dependencies
- No browser drivers required

## Compilation Requirements
- Rust 1.85.0+
- OpenSSL 3+
- All dependencies required for building [wreq](https://crates.io/crates/wreq)

# Installation

A ready-to-use Docker image is available for Ygégé.
To get started with Docker deployment and configuration, see the [dedicated Docker guide](https://ygege.lila.ws/en/installation/docker-guide).
## Docker

To create a custom Docker image with your own optimizations, refer to the [Docker build guide](https://ygege.lila.ws/en/installation/docker-guide).

## Manual Installation

To compile the application from sources, follow the [manual installation guide](https://ygege.lila.ws/en/installation/source-guide).

## IMDB and TMDB configuration

To enable IMDB and TMDB metadata fetching, please follow the instructions in the [TMDB and IMDB support guide](https://ygege.lila.ws/en/tmdb-imdb).

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

## Cloudflare Bypass
Ygégé uses multiple methods to bypass Cloudflare challenges:

### Primary Method: Leaked IP + Browser Emulation
YGG Torrent enforces a Cloudflare rule using the `account_created=true` cookie to prevent challenges, theoretically validating user accounts so we can just inject this cookie. However, Cloudflare still detects fake HTTPS clients and browser spoofing.

Ygégé uses the [wreq](https://crates.io/crates/wreq) library - an HTTP client based on `reqwest` and `tokio` that replicates 1:1 TLS and HTTP/2 exchanges to mimic legitimate browser behavior.

**Note**: Compatibility broke with Chrome 133 likely due to HTTP/3 integration, which `wreq` doesn't yet simulate.

### Fallback Method: FlareSolverr
When the primary method fails (leaked IP unavailable or detected), Ygégé can automatically fallback to [FlareSolverr](https://github.com/FlareSolverr/FlareSolverr) to solve Cloudflare challenges.

#### FlareSolverr Configuration
To enable FlareSolverr, set the environment variable:
```bash
FLARESOLVERR_URL=http://localhost:8191
```

Or in your `config.json`:
```json
{
  "flaresolverr_url": "http://localhost:8191"
}
```

#### Docker Compose with FlareSolverr
An example Docker Compose configuration is available in `docker-compose-flaresolverr.yml`:

```bash
docker-compose -f docker-compose-flaresolverr.yml up -d
```

This setup automatically launches FlareSolverr alongside Ygégé.

For technical deep dives:
- [TLS fingerprinting explained](https://fingerprint.com/blog/what-is-tls-fingerprinting-transport-layer-security/)
- [HTTP/2 fingerprinting and bypass techniques](https://www.trickster.dev/post/understanding-http2-fingerprinting/)

## Performance test

Query for search:
- Name: `Vaiana 2`
- Sort: `seeders`
- Order: `descending`

|                                      | Number of tests | Total time for all tests | Average time per test |
|--------------------------------------|-----------------|--------------------------|-----------------------|
| Resolution of the current YGG domain |        25       |        3220,378ms        |      128,81512ms      |
| New YGG login                        |        10       |       4881.71361ms       |     488.1713616ms     |
| YGG session restoration              |        10       |       2064.672142ms      |     206.4672142ms     |
| Search                               |       100       |      17621.045874ms      |     176,21045874ms    |

# API Documentation

The API documentation is available [here](https://ygege.lila.ws/en/api).