---
sidebar_position: 7
sidebar_label: API Documentation
---

# API Documentation

This page documents all Ygégé API endpoints.

## Base URL

```
http://localhost:8715
```

## Authentication

The API does not require direct authentication. YGG authentication is automatically handled via configuration.

## Available Endpoints

### 🔍 Search

- [`GET /search`](#torrent-search) - Search for torrents
- [`GET /categories`](#categories) - List categories

### 📦 Torrents

- [`GET /torrent/info`](#torrent-information) - Detailed information
- [`GET /torrent/{id}/files`](#torrent-files) - File list
- [`GET /download`](#download-torrent) - Download .torrent file

### 👤 User

- [`GET /user`](#user-information) - YGG account information

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
| `offset` | number | ❌ | Pagination (default: 0) |
| `category` | number | ❌ | Category ID |
| `categories` | string | ❌ | Comma-separated list of IDs |
| `sub_category` | number | ❌ | Subcategory ID |
| `sort` | string | ❌ | Sort field (see below) |
| `order` | string | ❌ | `ascending` or `descending` |
| `imdbid` | string | ❌ | IMDB ID (e.g. tt1234567) |
| `tmdbid` | string | ❌ | TMDB ID |
| `season` | number | ❌ | Season number (TV series) |
| `ep` | number | ❌ | Episode number (TV series) |
| `ban_words` | string | ❌ | Words to exclude (comma-separated) |

#### Valid Sort Fields

- `name` - Torrent name
- `size` - Size
- `publish_date` - Publication date
- `completed` - Download count
- `seed` - Seeders count
- `leech` - Leechers count
- `comments_count` - Comments count

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
    "id": 1234567,
    "name": "Moana.2.2024.MULTi.TRUEFRENCH.1080p.WEB-DL.H265",
    "category_id": 2178,
    "size": 3189013217,
    "completed": 15624,
    "seed": 933,
    "leech": 0,
    "comments_count": 43,
    "age_stamp": 1738044926,
    "info_url": "/torrent/info?id=1234567",
    "download": "/torrent/1234567",
    "url": "https://www.yggtorrent.top/engine/download_torrent?id=1234567"
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

## Torrent Information

### `GET /torrent/info`

Get detailed information about a specific torrent.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | number | ✅ | Torrent ID |

#### Example

```bash
curl "http://localhost:8715/torrent/info?id=1234567"
```

#### Response

```json
{
  "id": 1234567,
  "name": "Moana.2.2024.MULTi.TRUEFRENCH.1080p.WEB-DL.H265",
  "description": "Complete torrent description...",
  "category_id": 2178,
  "uploader": "Username",
  "upload_date": "2024-01-01T12:00:00Z",
  "size": 3189013217,
  "completed": 15624,
  "seeders": 933,
  "leechers": 0,
  "files": 5,
  "imdb": "tt10298810",
  "tmdb": "447277"
}
```

---

## Torrent Files

### `GET /torrent/{id}/files`

List all files contained in a torrent.

#### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | number | ✅ | Torrent ID |

#### Example

```bash
curl "http://localhost:8715/torrent/1234567/files"
```

#### Response

```json
[
  {
    "name": "Moana.2.2024.1080p.WEB-DL.mkv",
    "size": 3000000000
  },
  {
    "name": "Subs/french.srt",
    "size": 150000
  }
]
```

---

## Download Torrent

### `GET /download`

Download the .torrent file.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | number | ✅ | Torrent ID |

#### Example

```bash
curl -O "http://localhost:8715/download?id=1234567"
```

#### Response

Returns the `.torrent` file with `Content-Type: application/x-bittorrent` header.

---

## User Information

### `GET /user`

Get information about the connected YGG account.

#### Example

```bash
curl "http://localhost:8715/user"
```

#### Response

```json
{
  "username": "your_username",
  "rank": "Member",
  "uploaded": 123456789012,
  "downloaded": 98765432109,
  "ratio": 1.25,
  "bonus_points": 1500
}
```

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
  "status": "running",
  "version": "0.6.2",
  "uptime": 3600,
  "ygg_connected": true,
  "last_request": "2024-12-08T10:30:00Z"
}
```

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
| `YGG_ERROR` | YGG Torrent error |
| `AUTH_FAILED` | YGG authentication failed |
| `RATE_LIMITED` | Rate limit reached |

---

## Rate Limiting

To avoid YGG rate limiting:

- **Searches**: Limit to 1 request per second
- **Downloads**: No strict limit

:::warning Rate Limiting
If you're being rate-limited by YGG, verify that your credentials are correctly configured in `config.json`.
:::

---

## Complete Examples

### Search and Download

```bash
# 1. Search
results=$(curl -s "http://localhost:8715/search?q=moana+2")

# 2. Extract first ID
torrent_id=$(echo $results | jq -r '.[0].id')

# 3. Download
curl -O "http://localhost:8715/download?id=$torrent_id"
```

### With Python

```python
import requests

# Configuration
BASE_URL = "http://localhost:8715"

# Search
response = requests.get(f"{BASE_URL}/search", params={"q": "moana 2"})
torrents = response.json()

# Download first result
if torrents:
    torrent_id = torrents[0]["id"]
    download_url = f"{BASE_URL}/download?id={torrent_id}"
    
    response = requests.get(download_url)
    with open(f"{torrent_id}.torrent", "wb") as f:
        f.write(response.content)
```

---

## Next Steps

- [Configuration](./configuration)
- [Prowlarr Integration](./integrations/prowlarr)
