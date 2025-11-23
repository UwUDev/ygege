# Documentation de l'API

- [Recherche de torrents `/search`](#recherche-de-torrents-search)
- [Informations du torrent `/torrent/info`](#informations-du-torrent-torrentinfo)
- [Fichiers du torrent `/torrent/{id}/files`](#fichiers-du-torrent-torrentidfiles)
- [Catégories `/categories`](#catégories-categories)
- [Télécharger un torrent `/download`](#télécharger-un-torrent-download)
- [Informations utilisateur `/user`](#informations-utilisateur-user)
- [Vérification de santé `/health`](#vérification-de-santé-health)
- [Statut du service `/status`](#statut-du-service-status)

## Recherche de torrents `/search`

### Endpoint

```
GET /search
```

### Description

Recherche des torrents avec des filtres tels que le nom, la catégorie, l'offset, le tri et l'ordre. Renvoie un tableau
JSON contenant
les objets torrent qui correspondent aux critères.

### Paramètres de requête

| Paramètre    | Type   | Description                                          |
|--------------|--------|------------------------------------------------------|
| name \| q    | string | Nom partiel ou complet du torrent à rechercher.      |
| offset       | number | Offset de pagination (par défaut : 0).               |
| category     | number | ID de la catégorie pour filtrer les torrents.        |
| sub_category | number | ID de la sous-catégorie pour filtrer les torrents.   |
| sort         | enum   | Champ de tri (`name`, `size`, `publish_date`, etc.). |
| order        | enum   | Ordre du tri (`ascending`, `descending`).            |
| imdbid       | string | ID IMDB pour chercher directement les torrents liés. |
| tmdbid       | string | ID TMDB pour chercher directement les torrents liés. |
| ban_words    | string | Liste de mots interdits séparés par des virgules.    |

#### Champs de tri valides

- `name`
- `size`
- `publish_date`
- `completed`
- `seed`
- `leech`
- `comments_count`

#### Valeurs d'ordre valides

- `ascending`
- `descending`

### Exemple de requête

```
GET /search?q=vaiana+2&name=vaiana+2&sort=seed&order=desc
```

### Réponse

Renvoie un tableau JSON d'objets avec les champs suivants :

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

### Objet Torrent

| Champ          | Type   | Description                                   |
|----------------|--------|-----------------------------------------------|
| category_id    | number | ID de la catégorie du torrent.                |
| name           | string | Nom/titre du torrent.                         |
| id             | number | Identifiant unique du torrent.                |
| comments_count | number | Nombre de commentaires sur le torrent.        |
| age_stamp      | number | Timestamp de création (epoch Unix, sec).      |
| size           | number | Taille du torrent (octets).                   |
| completed      | number | Nombre de téléchargements complétés.          |
| seed           | number | Nombre de seeders.                            |
| leech          | number | Nombre de leechers.                           |
| info_url       | string | L'URL de l'endpoint d'information du torrent. |

### Réponses d'erreur

- Renvoie HTTP 400 avec un message d'erreur si les paramètres sont invalides.
- Renvoie HTTP 500 pour les erreurs serveur (probablement dû à un changement de mot de passe ou la disponibilité du
  site).

## Informations du torrent `/torrent/info`

### Endpoint

```
GET /torrent/info/{path:.*}
```

### Description

Récupère les informations détaillées sur un torrent spécifique en fournissant son URL d'information.
Cette URL peut être obtenue à partir du champ `info_url` du point de terminaison [
`/search`](#recherche-de-torrents-search).

### Réponse

Renvoie un objet JSON avec les champs suivants :

```json
{
    "author_id": 9466376,
    "author_name": "XenOxRox",
    "completed": 1673,
    "created_at": 1752452280,
    "hash": "df3e21046e5c7c8d863d92be724451e0af0bae03",
    "id": 1343675,
    "keywords": [
        "Multi (Français inclus)",
        "Aventure",
        "Simulation"
    ],
    "leech": 2,
    "seed": 125,
    "text_description": "The Sims 4: ...",
    "html_description": "\u003Cdiv class=\"default\" style=\"text-align:center !important\"\u003E\n\t\t\t\t\t\t\t\t\u003Cp\u003E\u003Cfont size=\"6\"\u003E\u003Cfont color=\"#aa0000\"\u003E\u003Cb\u003EThe Sims 4...",
    "flat_tree": [
        {
            "path": "The Sims 4 [FitGirl Repack]/fg-02.bin",
            "size": 1691245634
        },
        {
            "path": "The Sims 4 [FitGirl Repack]/MD5/fitgirl-bins.md5",
            "size": 364
        },
        ...
    ],
    "tree": {
        "Directory": {
            "children": [
                {
                    "File": {
                        "name": "Verify BIN files before installation.bat",
                        "size": 69
                    }
                },
                {
                    "Directory": {
                        "children": [
                            {
                                "File": {
                                    "name": "QuickSFV.EXE",
                                    "size": 103424
                                }
                            },
                            ...
                        ],
                        "name": "MD5",
                        "size": 103943
                    }
                },
                ...
            ],
            "name": "The Sims 4 [FitGirl Repack]",
            "size": 44708420269
        }
    }
}
```

### Objet d'Information du Torrent

| Champ            | Type             | Description                                                           |
|------------------|------------------|-----------------------------------------------------------------------|
| id               | number           | Identifiant unique du torrent.                                        |
| author_id        | number           | Identifiant de l'auteur/uploader du torrent.                          |
| author_name      | string           | Nom de l'auteur/uploader du torrent.                                  |
| created_at       | number           | Timestamp de création (epoch Unix, sec).                              |
| hash             | string           | Hash unique du torrent.                                               |
| completed        | number           | Nombre de téléchargements complétés.                                  |
| seed             | number           | Nombre de seeders.                                                    |
| leech            | number           | Nombre de leechers.                                                   |
| keywords         | array of strings | Mots-clés associés au torrent.                                        |
| text_description | string           | Description en texte brut du torrent.                                 |
| html_description | string (HTML)    | Description en HTML du torrent.                                       |
| flat_tree        | array of objects | Tableau plat des fichiers dans le torrent (voir ci-dessus).           |
| tree             | nested object    | Structure arborescente des fichiers dans le torrent (voir ci-dessus). |

### Réponses d'erreur

- Renvoie HTTP 400 si l'ID du torrent est invalide.
- Renvoie HTTP 404 si l'ID du torrent n'existe pas.
- Renvoie HTTP 500 pour les erreurs serveur

## Fichiers du torrent `/torrent/{id}/files`

### Endpoint

```
GET /torrent/{id:[0-9]+}/files
```

### Description

Récupère uniquement les informations de structure de fichiers pour un torrent spécifique par son ID. Cet endpoint
renvoie la structure arborescente, la liste des fichiers aplatie et la taille totale sans métadonnées supplémentaires
comme l'auteur, la description, etc.

### Paramètres de chemin

| Paramètre | Type   | Description                         |
|-----------|--------|-------------------------------------|
| id        | number | Identifiant unique du torrent (ID). |

### Exemple de requête

```
GET /torrent/1343675/files
```

### Réponse

Renvoie un objet JSON avec les champs suivants :

```json
{
    "flat_tree": [
        {
            "path": "The Sims 4 [FitGirl Repack]/fg-02.bin",
            "size": 1691245634
        },
        {
            "path": "The Sims 4 [FitGirl Repack]/MD5/fitgirl-bins.md5",
            "size": 364
        },
        ...
    ],
    "tree": {
        "Directory": {
            "children": [
                {
                    "File": {
                        "name": "Verify BIN files before installation.bat",
                        "size": 69
                    }
                },
                {
                    "Directory": {
                        "children": [
                            {
                                "File": {
                                    "name": "QuickSFV.EXE",
                                    "size": 103424
                                }
                            },
                            ...
                        ],
                        "name": "MD5",
                        "size": 103943
                    }
                },
                ...
            ],
            "name": "The Sims 4 [FitGirl Repack]",
            "size": 44708420269
        }
    },
    "name": "The Sims 4 [FitGirl Repack]",
    "total_size": 44708420269
}
```

### Objet de Réponse

| Champ      | Type   | Description                                                       |
|------------|--------|-------------------------------------------------------------------|
| tree       | object | Structure arborescente des fichiers du torrent.                   |
| flat_tree  | array  | Tableau plat des fichiers avec leurs chemins complets et tailles. |
| name       | string | Nom/titre du torrent.                                             |
| total_size | number | Taille totale de tous les fichiers du torrent (en octets).        |

### Réponses d'erreur

- Renvoie HTTP 400 si l'ID du torrent est invalide.
- Renvoie HTTP 404 si le torrent n'existe pas.
- Renvoie HTTP 500 pour les erreurs serveur.

## Catégories `/categories`

### Endpoint

```
GET /categories
```

### Description

Retourne un tableau JSON de catégories. Chaque catégorie contient un `id`, un `name` et un tableau `sub_categories` qui
peut contenir des catégories imbriquées ayant la même structure.

Cet endpoint est utile pour peupler des menus déroulants dans une UI ou pour mapper les `category_id` renvoyés par
`/search` vers des noms lisibles par l'humain.

### Réponse

Renvoie un tableau JSON d'objets catégorie. Exemple :

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

### Objet Catégorie

| Champ          | Type   | Description                                                  |
|----------------|--------|--------------------------------------------------------------|
| id             | string | Identifiant de la catégorie (string pour préserver les IDs). |
| name           | string | Nom lisible de la catégorie.                                 |
| sub_categories | array  | Tableau des sous-catégories (même forme d'objet).            |

### Réponses d'erreur

- Renvoie HTTP 500 pour les erreurs serveur.

---

## Télécharger un torrent `/download`

### Endpoint

```
GET /download/{id}
```

### Description

Télécharge le fichier torrent pour l'ID de torrent spécifié.

### Réponse

Renvoie le fichier torrent en binaire avec le type de contenu `application/x-bittorrent`.

### Réponses d'erreur

- Renvoie HTTP 400 si l'ID du torrent est invalide.
- Renvoie HTTP 404 si l'ID du torrent n'existe pas.
- Renvoie HTTP 500 pour les erreurs serveur (probablement dû à un changement de mot de passe ou la disponibilité du
  site).

---

## Informations utilisateur `/user`

### Endpoint

```
GET /user
```

### Description

Récupère les informations sur l'utilisateur authentifié : ratio, upload/download, passkey, etc.

### Réponse

Renvoie un objet JSON avec les champs suivants *(certains champs peuvent être nuls)* :

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

### Objet Utilisateur

| Champ            | Type              | Description                                     |
|------------------|-------------------|-------------------------------------------------|
| username         | string            | Nom d'utilisateur.                              |
| rank             | string            | Rang de l'utilisateur.                          |
| join_date        | string dd/mm/yyyy | Date d'inscription.                             |
| last_activity    | string            | Dernière activité.                              |
| torrents_count   | number            | Nombre de torrents uploadés par l'utilisateur.  |
| comments_count   | number            | Nombre de commentaires faits par l'utilisateur. |
| reputation_score | number            | Score de réputation de l'utilisateur.           |
| passkey          | string            | Passkey unique de l'utilisateur.                |
| uploaded         | number            | Total upload (octets).                          |
| downloaded       | number            | Total download (octets).                        |
| ratio            | number (float)    | Ratio upload/download.                          |
| avatar_url       | string (URL)      | URL vers l'avatar de l'utilisateur.             |
| email            | string (email)    | Email de l'utilisateur.                         |
| age              | number (nullable) | Âge de l'utilisateur.                           |
| gender           | string (nullable) | Genre (nullable).                               |
| country          | string (nullable) | Pays (nullable).                                |
| country_code     | string (nullable) | Code pays (ISO 3166).                           |

### Réponse d'erreur

- Renvoie HTTP 500 pour les erreurs serveur (probablement dû à un changement de mot de passe ou la disponibilité du
  site).

## Vérification de santé `/health`

### Endpoint

```
GET /health
```

### Description

Vérifie l'état de santé du service API.

### Réponse

Renvoie "OK" avec un statut HTTP 200 si le service fonctionne correctement.

## Statut du service `/status`

### Endpoint

```
GET /status
```

### Description

Récupère l'état actuel du service ygege.

### Réponse

Renvoie un objet JSON avec les champs suivants :

```json
{
    "auth": "authenticated",
    "domain": "www.yggtorrent.top",
    "domain_dns": "resolves",
    "domain_reachability": "reachable",
    "parsing": "ok",
    "search": "ok",
    "user_info": "ok"
}
```

### Objet de Statut

| Champ               | Type   | Description                                                         |
|---------------------|--------|---------------------------------------------------------------------|
| auth                | string | Statut d'authentification ("authenticated" ou "unauthenticated").   |
| domain              | string | Domaine actuellement utilisé par le service.                        |
| domain_dns          | string | Statut de résolution DNS ("resolves" ou "does_not_resolve").        |
| domain_reachability | string | Statut d'accessibilité ("reachable" ou "unreachable").              |
| parsing             | string | Statut de l'analyse ("ok" ou "failed").                             |
| search              | string | Statut de la fonctionnalité de recherche ("ok" ou "failed").        |
| user_info           | string | Statut de la fonctionnalité d'infos utilisateur ("ok" ou "failed"). |

### Réponse d'erreur

- Renvoie HTTP 500 pour les erreurs serveur (cela ne devrait JAMAIS arriver).
