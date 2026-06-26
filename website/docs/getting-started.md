---
sidebar_position: 2
---

# Guide de démarrage

Ce guide vous accompagne pas à pas dans l'installation et la configuration de Francisca, de l'installation initiale jusqu'à l'intégration avec vos applications de gestion de médias.

## Choix de la méthode d'installation

### Docker (Recommandé)

**Avantages :**
- Installation en une commande
- Mises à jour simplifiées
- Isolation complète
- Multi-architecture (AMD64, ARM64, ARMv7)

**Pour qui ?**
- Utilisateurs avec Docker déjà installé
- NAS Synology, QNAP, etc.
- Serveurs Linux
- Utilisateurs Windows avec WSL2

👉 [Guide Docker](./installation/docker-guide)

### Installation manuelle (Avancé)

**Avantages :**
- Contrôle total
- Pas de dépendance à Docker
- Performance native

**Pour qui ?**
- Développeurs
- Serveurs sans Docker
- Utilisateurs expérimentés

:::tip Binaires précompilés disponibles
À chaque release, des **binaires pré-compilés** sont fournis pour plusieurs plateformes (Linux, Windows, macOS). Téléchargez-les directement depuis la [page des releases](https://github.com/UwUDev/francisca/releases).
:::

👉 Pour compiler vous-même, voir le [README GitHub](https://github.com/UwUDev/francisca#building-from-source)

## Installation rapide (Docker Compose)

### Étape 1 : Créer le dossier de configuration

```bash
mkdir -p ~/francisca/config
cd ~/francisca
```

### Étape 2 : Créer le fichier compose.yml

```yaml
services:
  francisca:
    image: uwucode/francisca:latest
    container_name: francisca
    restart: unless-stopped
    ports:
      - "8715:8715"
    environment:
      LOG_LEVEL: "info"
      BIND_IP: "0.0.0.0"
      BIND_PORT: "8715"
      # TMDB_TOKEN: "votre_token_tmdb"  # Optionnel : pour recherche par TMDB/IMDB ID
      # USE_TOR: "true"               # Optionnel : activer le routage Tor

    # Health check pour vérifier le bon fonctionnement
    healthcheck:
      test: ["CMD-SHELL", "curl -f http://localhost:8715/health || exit 1"]
      interval: 1m30s
      timeout: 10s
      retries: 3
      start_period: 30s
```

### Étape 3 : Démarrer le service

```bash
docker compose up -d
```

### Étape 4 : Vérifier le fonctionnement

```bash
# Vérifier les logs
docker compose logs -f francisca

# Tester l'API
curl http://localhost:8715/health
```

Vous devriez voir :
```
INFO Francisca v0.x.x (commit: ..., branch: ..., built: ...)
INFO Using Nostr relay: wss://relay.example.org
INFO Categories initialized: 9 top-level categories
```

Vous pouvez également accéder à la page d'informations dans votre navigateur : `http://localhost:8715/`

![Page d'informations Francisca](/img/francisca-info.png)

Cette page affiche en temps réel l'état de tous les composants de Francisca :
- Connexion au relais Nostr
- Fonctionnement de la recherche
- Intégration TMDB/IMDB

## Configuration de base

:::info Aucune authentification requise
De nombreux relais Nostr sont **publics**. Selon le relais, aucun compte ni identifiant n'est nécessaire pour utiliser Francisca.
:::

### Ports réseau

Par défaut, Francisca écoute sur le port **8715**. Si ce port est déjà utilisé :

```yaml
ports:
  - "9090:8715"  # Utilise le port 9090 sur votre machine
```

Ou modifiez le port dans la configuration :
```yaml
environment:
  BIND_PORT: "9090"
ports:
  - "9090:9090"
```

## Intégration avec vos applications

Une fois Francisca configuré, intégrez-le avec vos applications :

### Prowlarr (Recommandé)

Prowlarr synchronise automatiquement les indexeurs avec Sonarr, Radarr, Lidarr, etc.

1. Téléchargez le fichier [`francisca.yml`](https://github.com/UwUDev/francisca/blob/master/francisca.yml)
2. Placez-le dans `{prowlarr_appdata}/Definitions/Custom/`
3. Redémarrez Prowlarr
4. Ajoutez l'indexeur Francisca dans Prowlarr

👉 [Guide complet Prowlarr](./integrations/prowlarr)

### Jackett

Alternative à Prowlarr, plus simple mais nécessite une configuration manuelle.

1. Téléchargez le fichier [`francisca.yml`](https://github.com/UwUDev/francisca/blob/master/francisca.yml)
2. Placez-le dans `{jackett_appdata}/cardigann/definitions/`
3. Redémarrez Jackett
4. Ajoutez l'indexeur Francisca dans Jackett

👉 [Guide complet Jackett](./integrations/jackett)

### Utilisation directe de l'API

Vous pouvez aussi utiliser l'API REST directement :

```bash
# Rechercher un torrent
curl "http://localhost:8715/search?q=breaking+bad&season=1&ep=1"

# Télécharger un torrent
curl -O "http://localhost:8715/download?id=1234567"
```

👉 [Documentation API complète](./api)

## Dépannage rapide

### Le service ne démarre pas

1. Vérifiez les logs :
   ```bash
   docker compose logs francisca
   ```

2. Vérifiez que le port 8715 est libre :
   ```bash
   # Linux/Mac
   lsof -i :8715
   
   # Windows
   netstat -ano | findstr :8715
   ```

### Pas de résultats de recherche

**Causes possibles :**
1. Le relais Nostr est inaccessible → Vérifiez les logs (`INFO Using Nostr relay: ...`)
2. Requête trop spécifique → Essayez avec moins de mots-clés
3. Catégories mal configurées → Vérifiez la configuration Prowlarr/Jackett

### Erreur "Connection refused"

Le service n'est pas accessible :

1. Vérifiez que le conteneur est en cours d'exécution :
   ```bash
   docker ps | grep francisca
   ```

2. Vérifiez que le port est bien exposé :
   ```bash
   docker compose ps
   ```

3. Testez depuis le conteneur lui-même :
   ```bash
   docker exec francisca curl http://localhost:8715/health
   ```

## Mises à jour

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

<Tabs groupId="installation-method">
  <TabItem value="docker-compose" label="Docker Compose" default>

```bash
# Télécharger la dernière image
docker compose pull

# Redémarrer avec la nouvelle image
docker compose up -d

# Nettoyer les anciennes images
docker image prune -f
```

  </TabItem>
  <TabItem value="docker-run" label="Docker Run">

```bash
# Arrêter le conteneur actuel
docker stop francisca
docker rm francisca

# Télécharger la dernière image
docker pull uwucode/francisca:latest

# Recréer le conteneur avec la même commande qu'à l'installation
# (réutilisez votre commande docker run)

# Nettoyer les anciennes images
docker image prune -f
```

  </TabItem>
  <TabItem value="binary" label="Binary">

```bash
# Arrêter Francisca
sudo systemctl stop francisca

# Télécharger la nouvelle version
wget https://github.com/UwUDev/francisca/releases/latest/download/francisca-linux-amd64

# Remplacer le binaire
sudo mv francisca-linux-amd64 /usr/local/bin/francisca
sudo chmod +x /usr/local/bin/francisca

# Redémarrer
sudo systemctl start francisca
```

  </TabItem>
</Tabs>

### Vérifier la version installée

```bash
curl http://localhost:8715/status | jq '.version'
```

## Prochaines étapes

Maintenant qu'Francisca est installé et configuré :

1. 📖 **[Configurez Prowlarr](./integrations/prowlarr)** - Synchronisation automatique avec vos applications \*arr
2. 🔧 **[Configuration avancée](./configuration)** - TMDB/IMDB, logging, etc.
3. 📡 **[Découvrez l'API](./api)** - Utilisez Francisca dans vos propres scripts
4. 🐳 **[Options Docker avancées](./installation/docker-guide)** - Tags, architectures, health checks

## Besoin d'aide ?

- 📚 Consultez la [documentation complète](/)
- 🐛 [Ouvrez une issue sur GitHub](https://github.com/UwUDev/francisca/issues)
- 💬 Lisez les [issues existantes](https://github.com/UwUDev/francisca/issues?q=is%3Aissue)

:::tip Contribution
Francisca est open-source ! N'hésitez pas à contribuer sur [GitHub](https://github.com/UwUDev/francisca).
:::
