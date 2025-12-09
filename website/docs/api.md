---
sidebar_position: 7
sidebar_label: API Documentation
---

# Documentation API

Cette page documente tous les endpoints de l'API Ygégé.

## Base URL

```
http://localhost:8715
```

## Authentification

L'API ne nécessite pas d'authentification directe. L'authentification YGG est gérée automatiquement via la configuration.

## Endpoints disponibles

### 🔍 Recherche

- [`GET /search`](#recherche-de-torrents) - Rechercher des torrents
- [`GET /categories`](#catégories) - Lister les catégories

### 📦 Torrents

- [`GET /torrent/info`](#informations-torrent) - Informations détaillées
- [`GET /torrent/{id}/files`](#fichiers-torrent) - Liste des fichiers
- [`GET /download`](#télécharger-torrent) - Télécharger le fichier .torrent

### 👤 Utilisateur

- [`GET /user`](#informations-utilisateur) - Informations du compte YGG

### ❤️ Santé

- [`GET /health`](#health-check) - Vérification de santé
- [`GET /status`](#status) - Statut du service

---

## Recherche de torrents

### `GET /search`

Recherche des torrents avec filtres avancés.

#### Paramètres de requête

| Paramètre | Type | Requis | Description |
|-----------|------|--------|-------------|
| `q` ou `name` | string | ❌ | Terme de recherche |
| `offset` | number | ❌ | Pagination (défaut: 0) |
| `category` | number | ❌ | ID de catégorie |
| `categories` | string | ❌ | Liste d'IDs séparés par virgules |
| `sub_category` | number | ❌ | ID de sous-catégorie |
| `sort` | string | ❌ | Champ de tri (voir ci-dessous) |
| `order` | string | ❌ | `ascending` ou `descending` |
| `imdbid` | string | ❌ | ID IMDB (ex: tt1234567) |
| `tmdbid` | string | ❌ | ID TMDB |
| `season` | number | ❌ | Numéro de saison (séries TV) |
| `ep` | number | ❌ | Numéro d'épisode (séries TV) |
| `ban_words` | string | ❌ | Mots à exclure (séparés par virgules) |

#### Champs de tri valides

- `name` - Nom du torrent
- `size` - Taille
- `publish_date` - Date de publication
- `completed` - Nombre de téléchargements
- `seed` - Nombre de seeders
- `leech` - Nombre de leechers
- `comments_count` - Nombre de commentaires

#### Exemples

**Recherche simple:**
```bash
curl "http://localhost:8715/search?q=vaiana+2"
```

**Recherche avancée:**
```bash
curl "http://localhost:8715/search?q=vaiana+2&sort=seed&order=descending&category=2178"
```

**Recherche par IMDB:**
```bash
curl "http://localhost:8715/search?imdbid=tt10298810"
```

**Recherche série (saison/épisode):**
```bash
curl "http://localhost:8715/search?q=breaking+bad&season=1&ep=1"
```

#### Réponse

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

#### Codes de réponse

| Code | Description |
|------|-------------|
| 200 | Succès |
| 400 | Paramètres invalides |
| 500 | Erreur serveur |

---

## Catégories

### `GET /categories`

Liste toutes les catégories et sous-catégories disponibles.

#### Exemple

```bash
curl "http://localhost:8715/categories"
```

#### Réponse

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

## Informations torrent

### `GET /torrent/info`

Obtenir les informations détaillées d'un torrent spécifique.

#### Paramètres de requête

| Paramètre | Type | Requis | Description |
|-----------|------|--------|-------------|
| `id` | number | ✅ | ID du torrent |

#### Exemple

```bash
curl "http://localhost:8715/torrent/info?id=1234567"
```

#### Réponse

```json
{
  "id": 1234567,
  "name": "Moana.2.2024.MULTi.TRUEFRENCH.1080p.WEB-DL.H265",
  "description": "Description complète du torrent...",
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

## Fichiers torrent

### `GET /torrent/{id}/files`

Liste tous les fichiers contenus dans un torrent.

#### Paramètres de chemin

| Paramètre | Type | Requis | Description |
|-----------|------|--------|-------------|
| `id` | number | ✅ | ID du torrent |

#### Exemple

```bash
curl "http://localhost:8715/torrent/1234567/files"
```

#### Réponse

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

## Télécharger torrent

### `GET /download`

Télécharge le fichier .torrent.

#### Paramètres de requête

| Paramètre | Type | Requis | Description |
|-----------|------|--------|-------------|
| `id` | number | ✅ | ID du torrent |

#### Exemple

```bash
curl -O "http://localhost:8715/download?id=1234567"
```

#### Réponse

Renvoie le fichier `.torrent` avec le header `Content-Type: application/x-bittorrent`.

---

## Informations utilisateur

### `GET /user`

Obtenir les informations du compte YGG connecté.

#### Exemple

```bash
curl "http://localhost:8715/user"
```

#### Réponse

```json
{
  "username": "votre_username",
  "rank": "Membre",
  "uploaded": 123456789012,
  "downloaded": 98765432109,
  "ratio": 1.25,
  "bonus_points": 1500
}
```

---

## Health Check

### `GET /health`

Vérifie que le service est opérationnel.

#### Exemple

```bash
curl "http://localhost:8715/health"
```

#### Réponse

```
OK
```

#### Codes de réponse

| Code | Description |
|------|-------------|
| 200 | Service opérationnel |
| 503 | Service indisponible |

---

## Status

### `GET /status`

Obtenir le statut détaillé du service et l'état de santé de tous les composants.

#### Exemple

```bash
curl "http://localhost:8715/status"
```

#### Réponse

```json
{
  "auth": "authenticated",
  "domain": "www.**********",
  "domain_dns": "resolves",
  "domain_reachability": "reachable",
  "parsing": "ok",
  "search": "ok",
  "user_info": "ok"
}
```

#### Champs de réponse

| Champ | Description | Valeurs possibles |
|-------|-------------|-------------------|
| `auth` | État de l'authentification YGG | `authenticated`, `failed` |
| `domain` | Domaine YGG actuellement utilisé | URL du domaine |
| `domain_dns` | Résolution DNS du domaine | `resolves`, `failed` |
| `domain_reachability` | Accessibilité du domaine | `reachable`, `unreachable` |
| `parsing` | État du parseur de torrents | `ok`, `error` |
| `search` | État de la fonctionnalité de recherche | `ok`, `error` |
| `user_info` | État de récupération des infos utilisateur | `ok`, `error` |
```

---

## Gestion des erreurs

Toutes les erreurs renvoient un objet JSON:

```json
{
  "error": "Description de l'erreur",
  "code": "ERROR_CODE"
}
```

### Codes d'erreur courants

| Code | Description |
|------|-------------|
| `INVALID_PARAMETERS` | Paramètres de requête invalides |
| `TORRENT_NOT_FOUND` | Torrent introuvable |
| `YGG_ERROR` | Erreur YGG Torrent |
| `AUTH_FAILED` | Échec d'authentification YGG |
| `RATE_LIMITED` | Rate limit atteint |

---

## Limites de débit

Pour éviter le rate limiting de YGG:

- **Recherches**: Limitez à 1 requête par seconde
- **Téléchargements**: Pas de limite stricte

:::warning Rate Limiting
Si vous êtes rate-limité par YGG, vérifiez que vos identifiants sont correctement configurés dans `config.json`.
:::

---

## Exemples complets

### Recherche et téléchargement

```bash
# 1. Rechercher
results=$(curl -s "http://localhost:8715/search?q=vaiana+2")

# 2. Extraire le premier ID
torrent_id=$(echo $results | jq -r '.[0].id')

# 3. Télécharger
curl -O "http://localhost:8715/download?id=$torrent_id"
```

### Avec Python

```python
import requests

# Configuration
BASE_URL = "http://localhost:8715"

# Recherche
response = requests.get(f"{BASE_URL}/search", params={"q": "vaiana 2"})
torrents = response.json()

# Télécharger le premier résultat
if torrents:
    torrent_id = torrents[0]["id"]
    download_url = f"{BASE_URL}/download?id={torrent_id}"
    
    response = requests.get(download_url)
    with open(f"{torrent_id}.torrent", "wb") as f:
        f.write(response.content)
```

---

## Prochaines étapes

- [Configuration](./configuration)
- [Intégration Prowlarr](./integrations/prowlarr)
