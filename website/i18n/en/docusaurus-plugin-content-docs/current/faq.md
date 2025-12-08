---
sidebar_position: 5
---

# FAQ - Frequently Asked Questions

Find answers to the most common questions about Ygégé.

## General

### What exactly is Ygégé?

Ygégé is an **indexer** for YGG Torrent. It transforms YGG into a source compatible with Prowlarr, Jackett, Sonarr, Radarr and other media management applications. It exposes a REST API that allows searching and downloading torrents.

### Why use Ygégé instead of existing Cardigann definitions?

- **Performance**: Written in Rust, 10-20x faster than Python/Node.js scrapers
- **Cloudflare Bypass**: Automatic and intelligent bypass without browser
- **TMDB/IMDB**: Automatic metadata enrichment
- **Active Maintenance**: Regular updates and community support
- **Multi-architecture**: ARM64/ARMv7 support for NAS and Raspberry Pi

### Is Ygégé legal?

Ygégé is open-source software that provides a technical interface to YGG Torrent. Using YGG Torrent and downloading copyrighted content depends on your country's legislation. **Use Ygégé responsibly and legally.**

### Does Ygégé work with other trackers?

No, Ygégé is specifically designed for YGG Torrent only. For other trackers, use Prowlarr/Jackett's native indexers.

## Installation and Configuration

### Do I need a YGG Torrent account?

**Yes, absolutely required.** YGG Torrent is a **private** tracker that requires credentials to access the site. Without valid credentials, Ygégé cannot connect.

:::warning Rate-limit warning
Even with credentials, monitor your request count to avoid being rate-limited by YGG. If you make too many requests in a short time, YGG may temporarily block your access.
:::

### What's the difference between Docker Run and Docker Compose?

- **Docker Run**: Single command for quick start, but difficult to maintain
- **Docker Compose**: Reusable configuration file, facilitates updates and management

**Recommendation**: Use Docker Compose for better long-term management.

### Can I install Ygégé without Docker?

Yes, you have two options:

1. **Pre-compiled binaries (Recommended)**: Download the binary for your platform from [GitHub releases](https://github.com/UwUDev/ygege/releases)
2. **Manual compilation**: Install Rust and compile from source

See the [build guide](https://github.com/UwUDev/ygege#building-from-source) for more details.

### Is port 8715 mandatory?

No, you can use any available port. Simply modify:

```yaml
environment:
  BIND_PORT: "9090"
ports:
  - "9090:9090"
```

### Does Ygégé store my credentials in clear text?

Credentials are stored in the `config.json` file or environment variables. **We recommend using Docker environment variables** and protecting access to your server.

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

### Does Ygégé work directly with Sonarr/Radarr?

No. Sonarr and Radarr require an intermediary indexer (Prowlarr or Jackett). Ygégé doesn't support the Torznab protocol directly.

**Recommended workflow**:
```
Ygégé → Prowlarr → Sonarr/Radarr/Lidarr
```

### How do I update the ygege.yml file?

The `ygege.yml` file defines the indexer for Prowlarr/Jackett. When it's updated on GitHub:

1. Download the new version
2. Replace the old file in `Definitions/Custom/`
3. Restart Prowlarr/Jackett

:::tip Notifications
Watch [GitHub releases](https://github.com/UwUDev/ygege/releases) to be notified of updates.
:::

## Performance and Limits

### How many requests can I make per day?

YGG Torrent being a private tracker, you **must** have valid credentials to use Ygégé.

:::warning Rate-limit
YGG may enforce rate-limiting if you make too many requests in a short time. It is recommended to:
- Avoid excessive automated searches
- Space out requests when possible
- Monitor logs to detect any rate-limit messages
:::

### Does Ygégé cache results?

No, each search queries YGG in real-time. This ensures always up-to-date results.

### Can the Cloudflare bypass fail?

Yes, in rare cases:
- Change in Cloudflare protection by YGG
- Temporary network issue
- IP ban (very rare)

**Solution**: Check logs, wait a few minutes, and retry.

### What's the load on YGG?

Ygégé optimizes requests and follows best practices:
- 1 request = 1 search on YGG
- No spam or abusive requests
- Custom User-Agent for identification

## Common Problems

### "Rate limited" / "Too many requests"

**Possible causes**:
1. YGG credentials not configured or invalid
2. Too many requests sent to YGG in a short time

**Solutions**:
1. Check `YGG_USERNAME` and `YGG_PASSWORD`
2. Test your credentials on the YGG website
3. If credentials are OK, wait a few minutes before retrying
4. Reduce the frequency of automated searches
5. Restart Ygégé

### "Connection refused" on localhost:8715

**Possible causes**:
1. Ygégé is not started
2. The port is different
3. Firewall issue

**Diagnosis**:
```bash
docker ps | grep ygege        # Check container is running
docker logs ygege             # See errors
curl http://localhost:8715/health  # Test API
```

### No results in Prowlarr/Jackett

**Checklist**:
- [ ] Ygégé is running: `curl http://localhost:8715/health`
- [ ] YGG credentials configured
- [ ] Correct URL in Prowlarr/Jackett (`http://localhost:8715/` or `http://ygege:8715/`)
- [ ] `ygege.yml` file up to date
- [ ] Prowlarr/Jackett restarted after adding the file

### Error 503 "Service Unavailable"

YGG Torrent is temporarily unavailable or under maintenance. Wait until the site is accessible again.

### Downloads won't start

Ygégé only provides `.torrent` files. Actual downloading is handled by:
- Your BitTorrent client (qBittorrent, Transmission, etc.)
- Sonarr/Radarr (if configured with a torrent client)

Check your BitTorrent client configuration.

## Docker and Deployment

### Can I use Ygégé on older architectures (NAS, embedded systems)?

**Yes!** If you encounter segmentation faults on older architectures or certain NAS (like Synology), use the `uwucode/ygege:noupx` image compiled without UPX compression:

```yaml
image: uwucode/ygege:noupx
```

This version is compatible with systems that don't support UPX-compressed binaries.

### Does Ygégé work on Raspberry Pi?

Yes! Docker images support ARMv7 and ARM64:
- Raspberry Pi 3/4/5: ✅ Full support
- Architecture: ARM64 or ARMv7

### How do I update Ygégé?

**With Docker Compose**:
```bash
docker compose pull
docker compose up -d
docker image prune -f
```

**With Docker Run**:
```bash
docker stop ygege
docker rm ygege
docker pull uwudev/ygege:latest
# Re-run docker run command
```

### Can I run multiple Ygégé instances?

Yes, but **it's generally not necessary**. If you do:
- Use different ports
- Use different container names
- Each instance will have its own YGG credentials

### How do I backup my configuration?

Simply backup the `./config` folder:

```bash
# Backup
tar -czf ygege-backup.tar.gz ./config

# Restore
tar -xzf ygege-backup.tar.gz
```

## TMDB and IMDB

### How do I enable TMDB/IMDB?

1. Create an account on [TMDB](https://www.themoviedb.org/)
2. Generate an API token in your account settings
3. Configure Ygégé:

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

No, it's **optional**. Ygégé works perfectly without it. Metadata simply improves search accuracy.

## Security

### Does Ygégé expose my personal data?

No. Ygégé doesn't collect, store, or transmit any personal data. It only communicates with:
- YGG Torrent (for searches)
- TMDB (if configured, for metadata)

### Should I expose Ygégé on the Internet?

**No, it's not recommended.** Ygégé is designed for local network (LAN) use. If you must expose it:
- Use a reverse proxy (Nginx, Traefik)
- Add authentication (Basic Auth, OAuth)
- Use HTTPS

### Can Ygégé be hacked?

Like any software, vulnerabilities can exist. To minimize risks:
- Update regularly
- Don't expose on the Internet without protection
- Use an isolated network if possible

## Support and Contribution

### Where do I report a bug?

Open an issue on GitHub: [github.com/UwUDev/ygege/issues](https://github.com/UwUDev/ygege/issues)

Include:
- Ygégé version
- Relevant logs
- Configuration (without your credentials!)
- Steps to reproduce

### How can I contribute to the project?

- 🐛 Report bugs
- 📖 Improve documentation
- 💻 Propose pull requests
- ⭐ Star on GitHub

See the [contribution guide](https://github.com/UwUDev/ygege/blob/develop/CONTRIBUTING.md).

### Is Ygégé actively maintained?

Yes! Check the [commit history](https://github.com/UwUDev/ygege/commits) and [releases](https://github.com/UwUDev/ygege/releases) to see recent activity.

## Other Questions

### What's the difference between Docker tags?

| Tag | Description | Usage |
|-----|-------------|-------|
| `latest` | Latest stable version | Production (recommended) |
| `stable` | Alias for `latest` | Production |
| `noupx` | Without UPX compression | Synology NAS |
| `0.6.2` | Specific version | Version locking |
| `develop` | Development version | Testing only |

### Can I use Ygégé commercially?

Ygégé is under an open-source license. Check the [LICENSE](https://github.com/UwUDev/ygege/blob/develop/LICENSE) for details. Commercial use also depends on YGG Torrent's ToS.

### Does Ygégé collect statistics?

No. No telemetry, no tracking. Ygégé is 100% private and runs entirely locally.

---

**Your question isn't listed?** Check the [complete documentation](/) or open an [issue on GitHub](https://github.com/UwUDev/ygege/issues).
