---
sidebar_position: 7
sidebar_label: API Documentation
---

# API Documentation

This page documents all Francisca API endpoints.

## Base URL

```
http://localhost:8715
```

## Authentication

The API requires no authentication. Depending on the Nostr relay used (often public), no account or credentials needed.

## Available Endpoints

### 🔍 Search

- [`GET /search`](#torrent-search) - Search for torrents
- [`GET /categories`](#categories) - List categories

### 📦 Torrents

- [`GET /torrent/{id}`](#download-torrent) - Redirect to magnet link

### ❤️ Health

- [`GET /health`](#health-check) - Health check
- [`GET /status`](#status) - Service status

---

## Torrent Search

### `GET /search`

Search for torrents with advanced filters.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `q` or `name` | string | ❌ | Search term |
| `category` | number | ❌ | Category ID |
| `categories` | string | ❌ | Comma-separated list of IDs |
| `sort` | string | ❌ | Sort field (see below) |
| `order` | string | ❌ | `ascending` or `descending` |
| `imdbid` | string | ❌ | IMDB ID (e.g. tt1234567) |
| `tmdbid` | string | ❌ | TMDB ID |
| `season` | number | ❌ | Season number (TV series) |
| `ep` | number | ❌ | Episode number (TV series) |
| `ban_words` | string | ❌ | Words to exclude (comma-separated) |
| `quote_search` | boolean | ❌ | Extended search (allows matching more results) |

#### Valid Sort Fields

- `name` - Torrent name
- `size` - Size
- `publish_date` - Publication date
- `completed` - Download count
- `seed` - Seeders count
- `leech` - Leechers count

#### Examples

**Simple search:**
```bash
curl "http://localhost:8715/search?q=moana+2"
```

**Advanced search:**
```bash
curl "http://localhost:8715/search?q=moana+2&sort=seed&order=descending&category=2178"
```

**Search by IMDB:**
```bash
curl "http://localhost:8715/search?imdbid=tt10298810"
```

**Series search (season/episode):**
```bash
curl "http://localhost:8715/search?q=breaking+bad&season=1&ep=1"
```

#### Response

```json
[
  {
    "id": "abc123def456",
    "name": "Moana.2.2024.MULTi.TRUEFRENCH.1080p.WEB-DL.H265",
    "category_id": 2178,
    "size": 3189013217,
    "completed": 15624,
    "seed": 933,
    "leech": 0,
    "file_count": 3,
    "age_stamp": 1738044926,
    "magnet": "magnet:?xt=urn:btih:...&dn=Moana.2.2024...&tr=...",
    "link": "https://relay.example.org/engine/torrent?id=abc123def456"
  }
]
```

#### Response Codes

| Code | Description |
|------|-------------|
| 200 | Success |
| 400 | Invalid parameters |
| 500 | Server error |

---

## Categories

### `GET /categories`

List all available categories and subcategories.

#### Example

```bash
curl "http://localhost:8715/categories"
```

#### Response

```json
[
  {
    "id": 2145,
    "name": "Film/Vidéo",
    "subcategories": [
      {
        "id": 2178,
        "name": "Film/Vidéo - Animation"
      },
      {
        "id": 2179,
        "name": "Film/Vidéo - Animation Série"
      }
    ]
  }
]
```

---

## Download Torrent

### `GET /torrent/{id}`

Redirects (HTTP 302) to the torrent's magnet link. Used automatically by Prowlarr/Jackett when downloading.

#### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | string | ✅ | Torrent ID (Nostr event ID) |

#### Example

```bash
# Follow the redirect to get the magnet
curl -L "http://localhost:8715/torrent/abc123def456"

# Or retrieve only the redirect URL
curl -I "http://localhost:8715/torrent/abc123def456"
```

#### Response

Redirects to the magnet link with HTTP `302 Found` status.

```
HTTP/1.1 302 Found
Location: magnet:?xt=urn:btih:...&dn=...&tr=...
```

:::tip
The `magnet` field is directly available in the `/search` response, allowing you to use it without calling this endpoint.
:::

---

## Health Check

### `GET /health`

Check if the service is operational.

#### Example

```bash
curl "http://localhost:8715/health"
```

#### Response

```
OK
```

#### Response Codes

| Code | Description |
|------|-------------|
| 200 | Service operational |
| 503 | Service unavailable |

---

## Status

### `GET /status`

Get detailed service status.

#### Example

```bash
curl "http://localhost:8715/status"
```

#### Response

```json
{
  "relay": "wss://relay.example.org",
  "search": "ok",
  "parsing": "ok",
  "tmdb_integration": "disabled"
}
```

#### Response Fields

| Field | Description | Possible Values |
|-------|-------------|-----------------|
| `relay` | Primary Nostr relay in use | WebSocket URL |
| `search` | Search functionality status | `ok`, `failed` |
| `parsing` | Nostr event parser status | `ok`, `empty`, `n/a` |
| `tmdb_integration` | TMDB integration status | `enabled`, `disabled` |

---

## Error Handling

All errors return a JSON object:

```json
{
  "error": "Error description",
  "code": "ERROR_CODE"
}
```

### Common Error Codes

| Code | Description |
|------|-------------|
| `INVALID_PARAMETERS` | Invalid query parameters |
| `TORRENT_NOT_FOUND` | Torrent not found |
| `RELAY_ERROR` | Nostr relay connection error |
| `RATE_LIMITED` | Rate limit reached |

---

## Rate Limiting

- **Searches**: Limit to 1 request per second to avoid overloading the relay

---

## Complete Examples

### Search and Use Magnet

```bash
# 1. Search
results=$(curl -s "http://localhost:8715/search?q=moana+2")

# 2. Extract magnet from first result
magnet=$(echo $results | jq -r '.[0].magnet')

# 3. Open with your torrent client
echo "$magnet"
```

### With Python

```python
import requests

BASE_URL = "http://localhost:8715"

# Search
response = requests.get(f"{BASE_URL}/search", params={"q": "moana 2"})
torrents = response.json()

# Use magnet from first result
if torrents:
    magnet = torrents[0]["magnet"]
    print(f"Magnet: {magnet}")

    # Or let Prowlarr/Jackett handle download via /torrent/{id}
    torrent_id = torrents[0]["id"]
    redirect = requests.get(f"{BASE_URL}/torrent/{torrent_id}", allow_redirects=False)
    print(f"Magnet (via redirect): {redirect.headers['Location']}")
```

---

## Next Steps

- [Configuration](./configuration)
- [Prowlarr Integration](./integrations/prowlarr)
