# API Doc

- [Search Torrents `/search`](#search-torrents-search)
- [Categories `/categories`](#categories-categories)
- [Download Torrent `/download`](#download-torrent-download)
- [User info `/user`](#user-info-user)

## Search Torrents `/search`

### Endpoint

```
GET /search
```

### Description

Search for torrents with filters such as name, category, offset, sorting, and ordering. Returns a JSON array containing
torrent objects that match the criteria.

### Query Parameters

| Parameter    | Type   | Description                                           |
|--------------|--------|-------------------------------------------------------|
| name         | string | Partial or full name of the torrent to search for.    |
| offset       | number | Pagination offset (default: 0).                       |
| category     | number | Category ID to filter torrents.                       |
| sub_category | number | Sub-category ID to filter torrents.                   |
| sort         | enum   | Sort field (`name`, `size`, `age_stamp`, etc.).       |
| order        | enum   | Sort order (`asc`, `desc`).                           |
| imdbid       | string | **NOT IMPLEMENTED YET** - IMDB ID to filter torrents. |
| tmdbid       | string | **NOT IMPLEMENTED YET** - TMDB ID to filter torrents. |

#### Valid Sort Fields

- `name`
- `size`
- `age_stamp`
- `completed`
- `seed`
- `leech`
- `comments_count`

#### Valid Order Values

- `ascending`
- `descending`
-

### Example Request

```
GET /search?q=vaiana+2&name=vaiana+2&sort=seed&order=desc
```

### Response

Returns a JSON array of objects with the following fields:

```json
[
    {
        "age_stamp": 1738044926,
        "category_id": 2178,
        "comments_count": 43,
        "completed": 15624,
        "download": "/torrent/xxxxxxx",
        "id": xxxxxxx,
        "leech": 0,
        "name": "Moana.2.2024.MULTi.TRUEFRENCH.1080p.WEB-DL.Dolby.Atmos.7.1.H265-Slay3R (Vaiana 2)",
        "seed": 933,
        "size": 3189013217,
        "url": "https://www.yggtorrent.top/engine/download_torrent?id=xxxxxxx"
    },
    ...
]
```

### Torrent Object

| Field          | Type   | Description                               |
|----------------|--------|-------------------------------------------|
| category_id    | number | Torrent category ID.                      |
| name           | string | Torrent name/title.                       |
| id             | number | Unique torrent identifier.                |
| comments_count | number | Number of comments on the torrent.        |
| age_stamp      | number | Creation timestamp (Unix epoch, seconds). |
| size           | number | Torrent size (bytes).                     |
| completed      | number | Number of completed downloads.            |
| seed           | number | Number of seeders.                        |
| leech          | number | Number of leechers.                       |

### Error Response

- Returns HTTP 400 with an error message if parameters are invalid.
- Returns HTTP 500 for server errors (most likely due to password change or website availability issues).

## Categories `/categories`

### Endpoint

```
GET /categories
```

### Description

Return a JSON array of categories. Each category contains an `id`, `name` and a `sub_categories` array which can contain
nested categories with the same shape.

This endpoint is useful to populate UI dropdowns or to map category ids found in the `/search` results to human-readable
names.

### Response

Returns a JSON array of category objects. Example:

```json
[
    {
        "id": "2145",
        "name": "Films & vidéos",
        "sub_categories": [
            {
                "id": "2178",
                "name": "Animation",
                "sub_categories": []
            },
            {
                "id": "2179",
                "name": "Action",
                "sub_categories": []
            }
        ]
    },
    {
        "id": "2300",
        "name": "Séries",
        "sub_categories": []
    }
]
```

### Category Object

| Field          | Type   | Description                                    |
|----------------|--------|------------------------------------------------|
| id             | string | Category identifier (string to preserve IDs).  |
| name           | string | Human readable category name.                  |
| sub_categories | array  | Array of child categories (same object shape). |

### Error Response

- Returns HTTP 500 for server errors.

---

## Download Torrent `/download`

### Endpoint

```
GET /download/{id}
```

### Description

Download the torrent file for the specified torrent ID.

### Response

Returns the torrent file as a binary response with `application/x-bittorrent` content type.

### Error Response

- Returns HTTP 400 if the torrent ID is invalid.
- Returns HTTP 404 if the torrent ID does not exist.
- Returns HTTP 500 for server errors (most likely due to password change or website availability issues).

## User info `/user`

### Endpoint

```
GET /user
```

### Description

Retrieve information about the authenticated user. like ratio, uploaded and downloaded data, key etc.

### Response

Returns a JSON object with the following fields *(nome are nullable)*:

```json
{
    "age": 23,
    "avatar_url": "https://www.yggtorrent.top/files/avatars/xxxx.jpg",
    "comments_count": 0,
    "country": "France",
    "country_code": "FR",
    "downloaded": 781394137579,
    "email": "ygogo@example",
    "gender": "female",
    "join_date": "07/09/2022",
    "last_activity": "7 minutes",
    "passkey": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    "rank": "Utilisateur",
    "ratio": 17.8844356536865,
    "reputation_score": 0,
    "torrents_count": 0,
    "uploaded": 13974792789032,
    "username": "Ygege"
}
```

### User Object

| Field            | Type              | Description                              |
|------------------|-------------------|------------------------------------------|
| username         | string            | User's username.                         |
| rank             | string            | User's rank.                             |
| join_date        | string dd/mm/yyy  | Date the user joined.                    |
| last_activity    | string            | Last activity time.                      |
| torrents_count   | number            | Number of torrents uploaded by the user. |
| comments_count   | number            | Number of comments made by the user.     |
| reputation_score | number            | User's reputation score.                 |
| passkey          | string            | User's unique passkey.                   |
| uploaded         | number            | Total data uploaded (bytes).             |
| downloaded       | number            | Total data downloaded (bytes).           |
| ratio            | number (float)    | User's upload/download ratio.            |
| avatar_url       | string (URL)      | URL to the user's avatar image.          |
| email            | string (email)    | User's email address.                    |
| age              | number (nullable) | User's age.                              |
| gender           | string (nullable) | User's gender.                           |
| country          | string (nullable) | User's country.                          |
| country_code     | string (nullable) | User's country code (ISO 3166 format).   |

### Error Response

- Returns HTTP 500 for server errors (most likely due to password change or website availability issues).