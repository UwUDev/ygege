---
sidebar_position: 1
---

# Installation avec Docker

Ygégé est disponible sous forme d'image Docker officielle multi-architecture. Ce guide explique comment déployer et configurer le service.

## Prérequis

- [Docker](https://docs.docker.com/get-docker/) installé sur votre système
- [Docker Compose](https://docs.docker.com/compose/install/) (recommandé pour une gestion simplifiée)
- Un compte YGG Torrent valide

## Installation rapide

### Avec Docker Run

```bash
docker run -d \
  --name ygege \
  -p 8715:8715 \
  -v ./config:/config \
  -e YGG_USERNAME="votre_nom_utilisateur" \
  -e YGG_PASSWORD="votre_mot_de_passe" \
  uwudev/ygege:latest
```

### Avec Docker Compose

Créez un fichier `compose.yml`:

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
      YGG_USERNAME: "votre_nom_utilisateur"
      YGG_PASSWORD: "votre_mot_de_passe"
      LOG_LEVEL: "debug"
    healthcheck:
      test: ["CMD-SHELL", "curl --fail http://localhost:8715/health || exit 1"]
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

Créez un fichier `config/config.json`:

```json
{
    "username": "votre_nom_utilisateur_ygg",
    "password": "votre_mot_de_passe",
    "bind_ip": "0.0.0.0",
    "bind_port": 8715,
    "log_level": "debug"
}
```

### Avec variables d'environnement

Les variables suivantes sont supportées:

| Variable | Description | Défaut |
|----------|-------------|--------|
| `YGG_USERNAME` | Nom d'utilisateur YGG | - |
| `YGG_PASSWORD` | Mot de passe YGG | - |
| `BIND_IP` | Adresse IP d'écoute | `0.0.0.0` |
| `BIND_PORT` | Port d'écoute | `8715` |
| `LOG_LEVEL` | Niveau de log (trace, debug, info, warn, error) | `info` |

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
    image: uwudev/ygege:noupx
    # ... reste de la configuration
```

## Vérification

Une fois le conteneur démarré, vérifiez qu'il fonctionne:

```bash
curl http://localhost:8715/health
```

Vous devriez recevoir une réponse `OK`.

## Prochaines étapes

- [Configuration avancée](../configuration)
- [Intégration avec Prowlarr](../integrations/prowlarr)
- [Intégration avec Jackett](../integrations/jackett)
