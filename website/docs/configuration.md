---
sidebar_position: 5
sidebar_label: Configuration
---

# Configuration

Ce guide détaille toutes les options de configuration disponibles pour Ygégé.

## Fichier config.json

Le fichier de configuration principal est `config.json`. Il doit être placé dans le dossier `/config` (Docker) ou à la racine du projet (installation manuelle).

### Structure complète

```json
{
    "username": "votre_nom_utilisateur",
    "password": "votre_mot_de_passe",
    "bind_ip": "0.0.0.0",
    "bind_port": 8715,
    "log_level": "info",
    "tmdb_token": null
}
```

## Options disponibles

### Authentification YGG

| Paramètre | Type | Requis | Description |
|-----------|------|--------|-------------|
| `username` | string | ✅ | Nom d'utilisateur YGG Torrent |
| `password` | string | ✅ | Mot de passe YGG Torrent |

:::warning Attention
YGG Torrent est un tracker privé. Des identifiants valides sont **obligatoires** pour que Ygégé puisse se connecter.
:::

### Configuration réseau

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `bind_ip` | string | `0.0.0.0` | Adresse IP d'écoute |
| `bind_port` | number | `8715` | Port d'écoute du serveur |

### Logging

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `log_level` | string | `info` | Niveau de verbosité des logs |

Niveaux disponibles:
- `trace` : Maximum de détails (développement)
- `debug` : Informations de débogage
- `info` : Informations générales
- `warn` : Avertissements uniquement
- `error` : Erreurs uniquement

### Métadonnées TMDB/IMDB

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `tmdb_token` | string | `null` | Token API TMDB (optionnel) |

:::info
Lorsque `tmdb_token` est configuré, les résolveurs **TMDB et IMDB** sont automatiquement activés ensemble.
:::

Pour configurer TMDB/IMDB, consultez le [guide d'intégration TMDB/IMDB](./tmdb-imdb).

## Variables d'environnement

Toutes les options peuvent également être définies via des variables d'environnement:

| Variable | Équivalent config.json |
|----------|------------------------|
| `YGG_USERNAME` | `username` |
| `YGG_PASSWORD` | `password` |
| `BIND_IP` | `bind_ip` |
| `BIND_PORT` | `bind_port` |
| `LOG_LEVEL` | `log_level` |
| `TMDB_TOKEN` | `tmdb_token` |

:::tip Priorité
Les variables d'environnement ont **priorité** sur le fichier config.json.
:::

## Exemple de configuration complète

### Pour Docker Compose

```yaml
services:
  ygege:
    image: uwudev/ygege:latest
    container_name: ygege
    restart: unless-stopped
    ports:
      - "8715:8715"
    volumes:
      - ./config:/config
    environment:
      YGG_USERNAME: "mon_username"
      YGG_PASSWORD: "mon_password"
      LOG_LEVEL: "info"
      TMDB_TOKEN: "votre_token_tmdb"
```

### Pour fichier config.json

```json
{
    "username": "mon_username",
    "password": "mon_password",
    "bind_ip": "0.0.0.0",
    "bind_port": 8715,
    "log_level": "info",
    "tmdb_token": "votre_token_tmdb"
}
```

## Validation de la configuration

Pour vérifier que votre configuration est correcte, consultez les logs au démarrage:

```bash
docker logs ygege
```

Vous devriez voir:
```
[INFO] Configuration chargée avec succès
[INFO] Connexion à YGG Torrent...
[INFO] Authentification réussie
[INFO] Serveur démarré sur 0.0.0.0:8715
```

## Prochaines étapes

- [API Documentation](./api)
- [Intégration Prowlarr](./integrations/prowlarr)
