---
sidebar_position: 5
sidebar_label: Configuration
---

# Configuration

Ce guide détaille toutes les options de configuration disponibles pour Ygégé.

## Fichier config.json

Le fichier de configuration principal est `config.json`. Il doit être placé à la racine du projet (installation manuelle) ou monté via un volume Docker.

### Structure complète

```json
{
    "bind_ip": "0.0.0.0",
    "bind_port": 8715,
    "log_level": "info",
    "tmdb_token": null,
    "use_tor": false,
    "tor_proxy": "127.0.0.1:9050"
}
```

## Options disponibles

### Configuration réseau

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `bind_ip` | string | `0.0.0.0` | Adresse IP d'écoute |
| `bind_port` | number | `8715` | Port d'écoute du serveur |

:::tip Personnaliser le port
Pour éviter les conflits de ports (ex: sur Windows), changez simplement `BIND_PORT` :
```yaml
environment:
  BIND_PORT: "3000"  # Utilise le port 3000 au lieu de 8715
ports:
  - "3000:3000"
```
Le healthcheck s'adapte automatiquement grâce à `$${BIND_PORT:-8715}`.
:::

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

### Support Tor

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `use_tor` | boolean | `false` | Activer le routage des connexions relay via Tor |
| `tor_proxy` | string | `127.0.0.1:9050` | Adresse du proxy SOCKS5 Tor |

:::info
Lorsque `use_tor` est activé, toutes les connexions aux relais Nostr sont routées via le proxy Tor. Tor doit être installé et en cours d'exécution sur votre machine.
:::

<<<<<<< HEAD
=======
### Proxy HTTP sortant

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `proxy_url` | string | `null` | URL du proxy HTTP(S) sortant |
| `proxy_username` | string | `null` | Nom d'utilisateur du proxy |
| `proxy_password` | string | `null` | Mot de passe du proxy |

:::info
Ce proxy s'applique aux requêtes HTTP(S) sortantes, par exemple TMDB/IMDB, ainsi qu'aux connexions WebSocket vers les relais Nostr quand `use_tor` est désactivé. Pour les relais Nostr, utilisez une URL de proxy HTTP de type `http://...`. Si `use_tor` est activé, `tor_proxy` reste prioritaire pour les relais.
:::

>>>>>>> fd3f5c5 (Enhance proxy configuration support for Nostr relay connections and update documentation)
## Variables d'environnement

Toutes les options peuvent également être définies via des variables d'environnement:

| Variable | Équivalent config.json |
|----------|------------------------|
| `BIND_IP` | `bind_ip` |
| `BIND_PORT` | `bind_port` |
| `LOG_LEVEL` | `log_level` |
| `TMDB_TOKEN` | `tmdb_token` |
| `USE_TOR` | `use_tor` |
| `TOR_PROXY` | `tor_proxy` |

:::tip Priorité
Les variables d'environnement ont **priorité** sur le fichier config.json.
:::

## Exemple de configuration complète

### Pour Docker Compose

```yaml
services:
  ygege:
    image: uwucode/ygege:latest
    container_name: ygege
    restart: unless-stopped
    ports:
      - "8715:8715"
    environment:
      LOG_LEVEL: "info"
      TMDB_TOKEN: "votre_token_tmdb"  # Optionnel
      # USE_TOR: "true"               # Optionnel : activer Tor
      # TOR_PROXY: "127.0.0.1:9050"   # Optionnel : proxy Tor alternatif
```

### Pour fichier config.json

```json
{
    "bind_ip": "0.0.0.0",
    "bind_port": 8715,
    "log_level": "info",
    "tmdb_token": "votre_token_tmdb",
    "use_tor": false,
    "tor_proxy": "127.0.0.1:9050"
}
```

## Validation de la configuration

Pour vérifier que votre configuration est correcte, consultez les logs au démarrage:

```bash
docker logs ygege
```

Vous devriez voir:
```
INFO Ygégé v0.x.x (commit: ..., branch: ..., built: ...)
INFO Tor routing disabled — connecting to relays directly
INFO Ranking Nostr relays by latency...
INFO Relay order: 1. wss://relay.ygg.gratis
INFO Categories initialized: 9 top-level categories
```

## Prochaines étapes

- [API Documentation](./api)
- [Intégration Prowlarr](./integrations/prowlarr)
