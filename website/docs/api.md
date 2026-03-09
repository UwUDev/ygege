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

L'API ne nécessite aucune authentification. ygg.gratis est un tracker public — aucun compte ni identifiant n'est requis.

## Endpoints disponibles

### 🔍 Recherche

- [`GET /search`](#recherche-de-torrents) - Rechercher des torrents
- [`GET /categories`](#catégories) - Lister les catégories

### 📦 Torrents

- [`GET /torrent/{id}`](#télécharger-torrent) - Redirection vers le lien magnet

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
| `category` | number | ❌ | ID de catégorie |
| `categories` | string | ❌ | Liste d'IDs séparés par virgules |
| `sort` | string | ❌ | Champ de tri (voir ci-dessous) |
| `order` | string | ❌ | `ascending` ou `descending` |
| `imdbid` | string | ❌ | ID IMDB (ex: tt1234567) |
| `tmdbid` | string | ❌ | ID TMDB |
| `season` | number | ❌ | Numéro de saison (séries TV) |
| `ep` | number | ❌ | Numéro d'épisode (séries TV) |
| `ban_words` | string | ❌ | Mots à exclure (séparés par virgules) |
| `quote_search` | boolean | ❌ | Recherche étendue (permet de matcher plus de résultats) |

#### Champs de tri valides

- `name` - Nom du torrent
- `size` - Taille
- `publish_date` - Date de publication
- `completed` - Nombre de téléchargements
- `seed` - Nombre de seeders
- `leech` - Nombre de leechers

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
    "link": "https://ygg.gratis/engine/torrent?id=abc123def456"
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

## Télécharger torrent

### `GET /torrent/{id}`

Redirige (HTTP 302) vers le lien magnet du torrent. Utilisé automatiquement par Prowlarr/Jackett lors du téléchargement.

#### Paramètres de chemin

| Paramètre | Type | Requis | Description |
|-----------|------|--------|-------------|
| `id` | string | ✅ | ID du torrent (identifiant Nostr) |

#### Exemple

```bash
# Suivre la redirection pour obtenir le magnet
curl -L "http://localhost:8715/torrent/abc123def456"

# Ou récupérer uniquement l'URL de redirection
curl -I "http://localhost:8715/torrent/abc123def456"
```

#### Réponse

Redirige vers le lien magnet avec un statut HTTP `302 Found`.

```
HTTP/1.1 302 Found
Location: magnet:?xt=urn:btih:...&dn=...&tr=...
```

:::tip
Le champ `magnet` est directement disponible dans la réponse `/search`, ce qui permet de l'utiliser sans appeler cet endpoint.
:::

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

Obtenir le statut détaillé du service.

#### Exemple

```bash
curl "http://localhost:8715/status"
```

#### Réponse

```json
{
  "relay": "wss://relay.ygg.gratis",
  "search": "ok",
  "parsing": "ok",
  "tmdb_integration": "disabled"
}
```

#### Champs de réponse

| Champ | Description | Valeurs possibles |
|-------|-------------|-------------------|
| `relay` | Relais Nostr principal utilisé | URL WebSocket |
| `search` | État de la fonctionnalité de recherche | `ok`, `failed` |
| `parsing` | État du parseur d'événements Nostr | `ok`, `empty`, `n/a` |
| `tmdb_integration` | État de l'intégration TMDB | `enabled`, `disabled` |

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
| `RELAY_ERROR` | Erreur de connexion au relais Nostr |
| `RATE_LIMITED` | Rate limit atteint |

---

## Limites de débit

- **Recherches** : Limitez à 1 requête par seconde pour éviter de surcharger le relais

---

## Exemples complets

### Recherche et utilisation du magnet

```bash
# 1. Rechercher
results=$(curl -s "http://localhost:8715/search?q=vaiana+2")

# 2. Extraire le lien magnet du premier résultat
magnet=$(echo $results | jq -r '.[0].magnet')

# 3. Ouvrir avec votre client torrent
echo "$magnet"
```

### Avec Python

```python
import requests

BASE_URL = "http://localhost:8715"

# Recherche
response = requests.get(f"{BASE_URL}/search", params={"q": "vaiana 2"})
torrents = response.json()

# Utiliser le magnet du premier résultat
if torrents:
    magnet = torrents[0]["magnet"]
    print(f"Magnet: {magnet}")

    # Ou laisser Prowlarr/Jackett gérer le téléchargement via /torrent/{id}
    torrent_id = torrents[0]["id"]
    redirect = requests.get(f"{BASE_URL}/torrent/{torrent_id}", allow_redirects=False)
    print(f"Magnet (via redirect): {redirect.headers['Location']}")
```

---

## Prochaines étapes

- [Configuration](./configuration)
- [Intégration Prowlarr](./integrations/prowlarr)
