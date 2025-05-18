# Ygégé

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
- All dependencies required for building [rquest](https://crates.io/crates/rquest)

# Installation

A ready-to-use Docker image is available for Ygégé.
To get started with Docker deployment and configuration, see the [dedicated Docker guide](docs/docker-guide.md).

## Docker

To create a custom Docker image with your own optimizations, refer to the [Docker build guide](docs/docker-dev.md).

## Manual Installation

To compile the application from sources, follow the [manual installation guide](docs/source-guide.md).

## Prowlarr integration

Ygégé can be used as a custom indexer for Prowlarr. To set it up, find your AppData directory (located in the `/system/status` page of Prowlarr) and copy the `ygege.yml` file on the repo in the `{your prowlarr appdata path}/Definitions/Custom` folder, you'll probably need to create the `Custom` folder.

Once it's done, restart Prowlarr and go to the indexer settings, you should see Ygégé in the list of available indexers.

> [!NOTE]  
> Prowlarr don't allow custom "Base URL", by defaul/t the URL is `http://localhost:8715/` but you can also choose ygege-dns-redirect.local and redirect it on your desired server IP/Domain with custom DNS or by editing you system hosts file


## Cloudflare Bypass
Ygégé bypasses Cloudflare challenges without browsers or third-party services.

YGG Torrent enforces a Cloudflare rule using the `account_created=true` cookie to prevent challenges, theoretically validating user accounts so we can just inject this cookie. However, Cloudflare still detects fake HTTPS clients and browser spoofing.

Ygégé uses the [rquest](https://crates.io/crates/rquest) library - an HTTP client based on `reqwest` and `tokio` that replicates 1:1 TLS and HTTP/2 exchanges to mimic legitimate browser behavior.

**Note**: Compatibility broke with Chrome 133 likely due to HTTP/3 integration, which `rquest` doesn't yet simulate.

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