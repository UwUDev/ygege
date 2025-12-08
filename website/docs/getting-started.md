---
sidebar_position: 2
---

# Guide de démarrage

Ce guide vous accompagne pas à pas dans l'installation et la configuration d'Ygégé, de l'installation initiale jusqu'à l'intégration avec vos applications de gestion de médias.

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
À chaque release, des **binaires pré-compilés** sont fournis pour plusieurs plateformes (Linux, Windows, macOS). Téléchargez-les directement depuis la [page des releases](https://github.com/UwUDev/ygege/releases).
:::

👉 Pour compiler vous-même, voir le [README GitHub](https://github.com/UwUDev/ygege#building-from-source)

## Installation rapide (Docker Compose)

### Étape 1 : Créer le dossier de configuration

```bash
mkdir -p ~/ygege/config
cd ~/ygege
```

### Étape 2 : Créer le fichier compose.yml

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
      # Identifiants YGG Torrent (OBLIGATOIRE)
      YGG_USERNAME: "votre_nom_utilisateur"
      YGG_PASSWORD: "votre_mot_de_passe"
      
      # Configuration optionnelle
      LOG_LEVEL: "info"
      BIND_IP: "0.0.0.0"
      BIND_PORT: "8715"
    
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
docker compose logs -f ygege

# Tester l'API
curl http://localhost:8715/health
```

Vous devriez voir :
```
[INFO] Configuration chargée avec succès
[INFO] Connexion à YGG Torrent...
[INFO] Authentification réussie
[INFO] Serveur démarré sur 0.0.0.0:8715
```

## Configuration de base

### Identifiants YGG Torrent

:::danger IMPORTANT
YGG Torrent est un tracker **privé**. Des identifiants valides sont **absolument obligatoires** pour utiliser Ygégé. Sans eux, Ygégé ne pourra pas se connecter.
:::

Vous avez deux options pour configurer vos identifiants :

**Option 1 : Variables d'environnement (Recommandé)**
```yaml
environment:
  YGG_USERNAME: "votre_nom_utilisateur"
  YGG_PASSWORD: "votre_mot_de_passe"
```

**Option 2 : Fichier config.json**
```json
{
    "username": "votre_nom_utilisateur",
    "password": "votre_mot_de_passe",
    "bind_ip": "0.0.0.0",
    "bind_port": 8715,
    "log_level": "info"
}
```

### Ports réseau

Par défaut, Ygégé écoute sur le port **8715**. Si ce port est déjà utilisé :

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

Une fois Ygégé configuré, intégrez-le avec vos applications :

### Prowlarr (Recommandé)

Prowlarr synchronise automatiquement les indexeurs avec Sonarr, Radarr, Lidarr, etc.

1. Téléchargez le fichier [`ygege.yml`](https://github.com/UwUDev/ygege/blob/master/ygege.yml)
2. Placez-le dans `{prowlarr_appdata}/Definitions/Custom/`
3. Redémarrez Prowlarr
4. Ajoutez l'indexeur Ygégé dans Prowlarr

👉 [Guide complet Prowlarr](./integrations/prowlarr)

### Jackett

Alternative à Prowlarr, plus simple mais nécessite une configuration manuelle.

1. Téléchargez le fichier [`ygege.yml`](https://github.com/UwUDev/ygege/blob/master/ygege.yml)
2. Placez-le dans `{jackett_appdata}/cardigann/definitions/`
3. Redémarrez Jackett
4. Ajoutez l'indexeur Ygégé dans Jackett

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
   docker compose logs ygege
   ```

2. Vérifiez que le port 8715 est libre :
   ```bash
   # Linux/Mac
   lsof -i :8715
   
   # Windows
   netstat -ano | findstr :8715
   ```

### Erreur d'authentification YGG

```
[ERROR] Échec d'authentification YGG
```

**Solutions :**
- Vérifiez vos identifiants YGG
- Connectez-vous sur le site YGG pour vérifier votre compte
- Vérifiez que votre compte n'est pas banni ou suspendu

### Pas de résultats de recherche

**Causes possibles :**
1. Identifiants YGG non configurés → Vous êtes rate-limité
2. Problème de connexion à YGG → Vérifiez les logs
3. Catégories mal configurées → Vérifiez la configuration Prowlarr/Jackett

### Erreur "Connection refused"

Le service n'est pas accessible :

1. Vérifiez que le conteneur est en cours d'exécution :
   ```bash
   docker ps | grep ygege
   ```

2. Vérifiez que le port est bien exposé :
   ```bash
   docker compose ps
   ```

3. Testez depuis le conteneur lui-même :
   ```bash
   docker exec ygege curl http://localhost:8715/health
   ```

## Mises à jour

### Avec Docker Compose

```bash
# Télécharger la dernière image
docker compose pull

# Redémarrer avec la nouvelle image
docker compose up -d

# Nettoyer les anciennes images
docker image prune -f
```

### Vérifier la version installée

```bash
curl http://localhost:8715/status | jq '.version'
```

## Prochaines étapes

Maintenant qu'Ygégé est installé et configuré :

1. 📖 **[Configurez Prowlarr](./integrations/prowlarr)** - Synchronisation automatique avec vos applications \*arr
2. 🔧 **[Configuration avancée](./configuration)** - TMDB/IMDB, logging, etc.
3. 📡 **[Découvrez l'API](./api)** - Utilisez Ygégé dans vos propres scripts
4. 🐳 **[Options Docker avancées](./installation/docker-guide)** - Tags, architectures, health checks

## Besoin d'aide ?

- 📚 Consultez la [documentation complète](/)
- 🐛 [Ouvrez une issue sur GitHub](https://github.com/UwUDev/ygege/issues)
- 💬 Lisez les [issues existantes](https://github.com/UwUDev/ygege/issues?q=is%3Aissue)

:::tip Contribution
Ygégé est open-source ! N'hésitez pas à contribuer sur [GitHub](https://github.com/UwUDev/ygege).
:::
