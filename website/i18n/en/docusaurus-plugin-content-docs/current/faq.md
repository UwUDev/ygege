---
sidebar_position: 5
---

# FAQ - Frequently Asked Questions

Find answers to the most common questions about Francisca.

## General

### What exactly is Francisca?

Francisca is a generic **indexer** for Nostr relays (NIP-35). It transforms any compatible Nostr relay into a source compatible with Prowlarr, Jackett, Sonarr, Radarr and other media management applications. It exposes a REST API that allows searching for torrents and retrieving their magnet links via the **Nostr** protocol (NIP-35).

### Why use Francisca instead of existing Cardigann definitions?

- **Performance**: Written in Rust, 10-20x faster than Python/Node.js scrapers
- **Native Nostr**: Direct connection to Nostr relays, no HTML scraping
- **TMDB/IMDB**: Automatic metadata enrichment
- **Active Maintenance**: Regular updates and community support
- **Multi-architecture**: ARM64/ARMv7 support for NAS and Raspberry Pi

### Is Francisca legal?

Francisca is open-source software that provides a generic technical interface to Nostr relays. Using the relays and downloading copyrighted content depends on your country's legislation. **Use Francisca responsibly and legally.**

### Does Francisca work with other trackers?

Yes. Francisca is general-purpose: it works with any Nostr relay exposing NIP-35 torrent events.

## Installation and Configuration

### Do I need an account?

**It depends on the relay.** Many Nostr relays are **public** — no account or credentials required. Francisca connects directly to the configured Nostr relay.

### What's the difference between Docker Run and Docker Compose?

- **Docker Run**: Single command for quick start, but difficult to maintain
- **Docker Compose**: Reusable configuration file, facilitates updates and management

**Recommendation**: Use Docker Compose for better long-term management.

### Can I install Francisca without Docker?

Yes, you have two options:

1. **Pre-compiled binaries (Recommended)**: Download the binary for your platform from [GitHub releases](https://github.com/UwUDev/francisca/releases)
2. **Manual compilation**: Install Rust and compile from source

See the [build guide](https://github.com/UwUDev/francisca#building-from-source) for more details.

### Is port 8715 mandatory?

No, you can use any available port. Simply modify:

```yaml
environment:
  BIND_PORT: "9090"
ports:
  - "9090:9090"
```

## Integrations

### Prowlarr or Jackett, which to choose?

**Prowlarr (recommended)**:
- ✅ Automatic synchronization with Sonarr/Radarr/Lidarr
- ✅ Modern interface
- ✅ Better performance
- ❌ More complex initial configuration

**Jackett**:
- ✅ Simpler configuration
- ✅ Stable and proven
- ❌ Manual synchronization with \*arr apps
- ❌ Dated interface

### Can I use both Prowlarr AND Jackett at the same time?

Technically yes, but it's **not recommended**. This would create duplicates in your Sonarr/Radarr applications. Choose only one.

### Does Francisca work directly with Sonarr/Radarr?

No. Sonarr and Radarr require an intermediary indexer (Prowlarr or Jackett). Francisca doesn't support the Torznab protocol directly.

**Recommended workflow**:
```
Francisca → Prowlarr → Sonarr/Radarr/Lidarr
```

### How do I update the francisca.yml file?

The `francisca.yml` file defines the indexer for Prowlarr/Jackett. When it's updated on GitHub:

1. Download the new version
2. Replace the old file in `Definitions/Custom/`
3. Restart Prowlarr/Jackett

:::tip Notifications
Watch [GitHub releases](https://github.com/UwUDev/francisca/releases) to be notified of updates.
:::

## Performance and Limits

### How many requests can I make?

With a public relay, there is no credential-based limit. However, to avoid overloading the Nostr relay, avoid excessive automated searches.

:::info Rate-limit
Space out your requests (approximately 1 per second) to avoid overloading the Nostr relay.
:::

### Does Francisca cache results?

No, each search queries the Nostr relay in real-time. This ensures always up-to-date results.

### Can the Nostr relay be unavailable?

Yes, in rare cases:
- The configured Nostr relay is temporarily unavailable
- Temporary network issue

**Solution**: Check logs, wait a few minutes, and retry.

### What's the load on the relay?

Francisca optimizes requests:
- 1 request = 1 NIP-50 filter sent to the Nostr relay
- No spam or abusive requests

## Common Problems

### "Rate limited" / "No results"

**Possible causes**:
1. The Nostr relay is temporarily overloaded
2. Too many requests sent in a short time

**Solutions**:
1. Check logs: `docker logs francisca`
2. Wait a few minutes before retrying
3. Reduce the frequency of automated searches
4. Restart Francisca

### "Connection refused" on localhost:8715

**Possible causes**:
1. Francisca is not started
2. The port is different
3. Firewall issue

**Diagnosis**:
```bash
docker ps | grep francisca        # Check container is running
docker logs francisca             # See errors
curl http://localhost:8715/health  # Test API
```

### No results in Prowlarr/Jackett

**Checklist**:
- [ ] Francisca is running: `curl http://localhost:8715/health`
- [ ] Correct URL in Prowlarr/Jackett (`http://localhost:8715/` or `http://francisca:8715/`)
- [ ] `francisca.yml` file up to date
- [ ] Prowlarr/Jackett restarted after adding the file
- [ ] Nostr relay accessible: `curl http://localhost:8715/status`

### Error 503 "Service Unavailable"

The Nostr relay is temporarily unavailable. Wait and retry.

### Downloads won't start

Francisca provides **magnet links** (not `.torrent` files). Actual downloading is handled by:
- Your BitTorrent client (qBittorrent, Transmission, etc.)
- Sonarr/Radarr (if configured with a torrent client)

Check your BitTorrent client configuration and ensure Prowlarr/Jackett is configured to use magnets.

## Docker and Deployment

### Can I use Francisca on older architectures (NAS, embedded systems)?

**Yes!** If you encounter segmentation faults on older architectures or certain NAS (like Synology), use the `uwucode/francisca:noupx` image compiled without UPX compression:

```yaml
image: uwucode/francisca:noupx
```

This version is compatible with systems that don't support UPX-compressed binaries.

### Does Francisca work on Raspberry Pi?

Yes! Docker images support ARMv7 and ARM64:
- Raspberry Pi 3/4/5: ✅ Full support
- Architecture: ARM64 or ARMv7

### How do I update Francisca?

**With Docker Compose**:
```bash
docker compose pull
docker compose up -d
docker image prune -f
```

**With Docker Run**:
```bash
docker stop francisca
docker rm francisca
docker pull uwucode/francisca:latest
# Re-run docker run command
```

### Can I run multiple Francisca instances?

Yes, but **it's generally not necessary**. If you do:
- Use different ports
- Use different container names

### How do I backup my configuration?

Simply backup the `./config` folder:

```bash
# Backup
tar -czf francisca-backup.tar.gz ./config

# Restore
tar -xzf francisca-backup.tar.gz
```

## TMDB and IMDB

### How do I enable TMDB/IMDB?

1. Create an account on [TMDB](https://www.themoviedb.org/)
2. Generate an API token in your account settings
3. Configure Francisca:

```yaml
environment:
  TMDB_TOKEN: "your_tmdb_token"
```

:::info
When `TMDB_TOKEN` is configured, both **TMDB and IMDB** resolvers are automatically enabled together.
:::

### What are TMDB/IMDB metadata used for?

They automatically enrich results with:
- Official titles
- Posters and images
- Ratings and popularity
- Exact matches for Sonarr/Radarr

### Is TMDB/IMDB mandatory?

No, it's **optional**. Francisca works perfectly without it. Metadata simply improves search accuracy.

## Security

### Does Francisca expose my personal data?

No. Francisca doesn't collect, store, or transmit any personal data. It only communicates with:
- The configured Nostr relays (for searches)
- TMDB (if configured, for metadata)

### Should I expose Francisca on the Internet?

**No, it's not recommended.** Francisca is designed for local network (LAN) use. If you must expose it:
- Use a reverse proxy (Nginx, Traefik)
- Add authentication (Basic Auth, OAuth)
- Use HTTPS

### Can Francisca be hacked?

Like any software, vulnerabilities can exist. To minimize risks:
- Update regularly
- Don't expose on the Internet without protection
- Use an isolated network if possible

## Support and Contribution

### Where do I report a bug?

Open an issue on GitHub: [github.com/UwUDev/francisca/issues](https://github.com/UwUDev/francisca/issues)

Include:
- Francisca version
- Relevant logs
- Configuration
- Steps to reproduce

### How can I contribute to the project?

- 🐛 Report bugs
- 📖 Improve documentation
- 💻 Propose pull requests
- ⭐ Star on GitHub

See the [contribution guide](https://github.com/UwUDev/francisca/blob/develop/CONTRIBUTING.md).

### Is Francisca actively maintained?

Yes! Check the [commit history](https://github.com/UwUDev/francisca/commits) and [releases](https://github.com/UwUDev/francisca/releases) to see recent activity.

## Other Questions

### What's the difference between Docker tags?

| Tag | Description | Usage |
|-----|-------------|-------|
| `latest` | Latest stable version | Production (recommended) |
| `stable` | Alias for `latest` | Production |
| `noupx` | Without UPX compression | Synology NAS |
| `0.6.2` | Specific version | Version locking |
| `develop` | Development version | Testing only |

### Can I use Francisca commercially?

Francisca is under an open-source license. Check the [LICENSE](https://github.com/UwUDev/francisca/blob/develop/LICENSE) for details. Commercial use also depends on the ToS of the Nostr relay you use.

### Does Francisca collect statistics?

No. No telemetry, no tracking. Francisca is 100% private and runs entirely locally.

---

**Your question isn't listed?** Check the [complete documentation](/) or open an [issue on GitHub](https://github.com/UwUDev/francisca/issues).
