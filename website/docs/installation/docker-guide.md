---
sidebar_position: 1
---

# Installation avec Docker

Ygégé est disponible sous forme d'image Docker officielle multi-architecture. Ce guide explique comment déployer et configurer le service.

## Prérequis

- [Docker](https://docs.docker.com/get-docker/) installé sur votre système
- [Docker Compose](https://docs.docker.com/compose/install/) (recommandé pour une gestion simplifiée)

## Installation rapide

### Avec Docker Run

```bash
docker run -d \
  --name ygege \
  -p 8715:8715 \
  uwucode/ygege:latest
```

### Avec Docker Compose

Créez un fichier `compose.yml`:

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
      # TMDB_TOKEN: "votre_token_tmdb"  # Optionnel
      # RELAY_URL: "wss://relay.ygg.gratis"  # Optionnel
    healthcheck:
      test: ["CMD-SHELL", "curl --fail http://localhost:$${BIND_PORT:-8715}/health || exit 1"]
      interval: 1m30s
      timeout: 20s
      retries: 3
      start_period: 10s
```

Puis démarrez le service:

```bash
docker compose up -d
```

## Configuration

### Avec fichier config.json

Créez un fichier `config.json` et montez-le en lecture seule:

```json
{
    "bind_ip": "0.0.0.0",
    "bind_port": 8715,
    "log_level": "info",
    "tmdb_token": null,
    "relay_url": null
}
```

### Avec variables d'environnement

Les variables suivantes sont supportées:

| Variable | Description | Défaut |
|----------|-------------|--------|
| `BIND_IP` | Adresse IP d'écoute | `0.0.0.0` |
| `BIND_PORT` | Port d'écoute | `8715` |
| `LOG_LEVEL` | Niveau de log (trace, debug, info, warn, error) | `info` |
| `TMDB_TOKEN` | Token API TMDB (optionnel) | - |
| `RELAY_URL` | URL du relais Nostr (optionnel) | `wss://relay.ygg.gratis` |

## Tags Docker disponibles

| Tag | Description |
|-----|-------------|
| `latest` | Dernière version stable |
| `stable` | Alias de latest |
| `noupx` | Version sans compression UPX (pour Synology) |
| `0.6.2` | Version spécifique |
| `develop` | Version de développement |

### Pour les systèmes avec architectures anciennes

Si vous rencontrez des erreurs de segmentation (segfault) sur des architectures anciennes ou certains NAS (comme Synology), utilisez l'image `noupx`:

```yaml
services:
  ygege:
    image: uwucode/ygege:noupx
    # ... reste de la configuration
```

## Vérification

Une fois le conteneur démarré, vérifiez qu'il fonctionne:

```bash
curl http://localhost:8715/health
```

Vous devriez recevoir une réponse `OK`.

## Sécurité

### Utilisateur non-root

L'image Docker Ygégé s'exécute par défaut avec un utilisateur non-root (UID 10001) pour des raisons de sécurité. Cela garantit:

- ✅ Compatibilité avec les politiques de sécurité Docker et Kubernetes
- ✅ Protection contre les escalades de privilèges
- ✅ Conformité aux meilleures pratiques de sécurité des conteneurs

### Gestion des permissions

Ygégé ne nécessite pas de volume persistant pour les sessions puisque le tracker est public.

### Exécution avec un UID personnalisé

Si vous souhaitez exécuter le conteneur avec un UID/GID spécifique (par exemple pour correspondre à votre utilisateur hôte):

```bash
docker run -d \
  --name ygege \
  --user 1000:1000 \
  -p 8715:8715 \
  -v ./config:/app/sessions \
  -v ./config.json:/app/config.json \
  uwucode/ygege:latest
```

Ou avec Docker Compose:

```yaml
services:
  ygege:
    image: uwucode/ygege:latest
    user: "1000:1000"  # Votre UID:GID
    # ... reste de la configuration
```

:::tip
Assurez-vous que les volumes montés ont les permissions appropriées pour l'utilisateur spécifié:
```bash
sudo chown -R 1000:1000 ./config ./sessions
```
:::

### Exécution en root (non recommandé)

:::danger Avertissement de sécurité
L'exécution en root n'est **pas recommandée** et peut présenter des risques de sécurité. Utilisez cette option uniquement si vous comprenez les implications.
:::

Si vous devez absolument exécuter le conteneur en root:

**Docker Run**:
```bash
docker run -d \
  --name ygege \
  --user 0:0 \
  -p 8715:8715 \
  uwucode/ygege:latest
```

**Docker Compose**:
```yaml
services:
  ygege:
    image: uwucode/ygege:latest
    container_name: ygege
    user: "0:0"  # Root
    restart: unless-stopped
    ports:
      - "8715:8715"
```

Avec cette configuration, vous n'aurez plus de problèmes de permissions, mais vous perdez les avantages de sécurité du mode non-root.

## Prochaines étapes

- [Configuration avancée](../configuration)
- [Intégration avec Prowlarr](../integrations/prowlarr)
- [Intégration avec Jackett](../integrations/jackett)
