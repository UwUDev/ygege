---
sidebar_position: 1
---

# Intégration Prowlarr

Ygégé peut être utilisé comme indexeur personnalisé pour Prowlarr, permettant d'intégrer YGG Torrent dans votre stack de gestion de médias.

## Prérequis

- Prowlarr installé et fonctionnel
- Ygégé démarré et accessible
- Le fichier `ygege.yml` du dépôt GitHub

## Installation

### 1. Localiser le dossier AppData de Prowlarr

Le chemin du dossier AppData est affiché dans la page `/system/status` de Prowlarr.

Exemples de chemins:
- **Linux/Docker**: `/config` ou `/data`
- **Windows**: `C:\ProgramData\Prowlarr`
- **macOS**: `~/.config/Prowlarr`

### 2. Créer le dossier Custom

Dans le dossier AppData de Prowlarr, naviguez vers `Definitions/` et créez un dossier `Custom` s'il n'existe pas:

```bash
mkdir -p /config/Definitions/Custom
```

### 3. Copier le fichier de définition

Copiez le fichier `ygege.yml` (français par défaut, ou `ygege-en.yml` pour la version anglaise) du dépôt GitHub dans le dossier `Custom`:

```bash
# Télécharger directement depuis GitHub
wget https://raw.githubusercontent.com/UwUDev/ygege/master/ygege.yml \
  -O /config/Definitions/Custom/ygege.yml
```

Ou manuellement:
1. Téléchargez [`ygege.yml`](https://github.com/UwUDev/ygege/blob/master/ygege.yml)
2. Placez-le dans `{appdata}/Definitions/Custom/`

### 4. Redémarrer Prowlarr

Redémarrez Prowlarr pour qu'il détecte le nouvel indexeur:

```bash
# Docker
docker restart prowlarr

# Systemd
systemctl restart prowlarr
```

## Configuration de l'indexeur

### 1. Ajouter l'indexeur

1. Allez dans **Indexers**
2. Cliquez sur le bouton **+** pour ajouter un indexeur
3. Recherchez "Ygégé" dans la liste
4. Cliquez sur "Ygégé"

<!-- ![Prowlarr Add Indexer](/img/prowlarr-add-indexer.png) -->

### 2. Configurer les paramètres

<!-- ![Prowlarr Ygege Configuration](/img/prowlarr-ygege-config.png) -->

| Paramètre | Valeur | Description |
|-----------|--------|-------------|
| **Name** | Ygégé | Nom de l'indexeur |
| **Enable** | ✅ | Activer l'indexeur |
| **URL** | `http://localhost:8715/` | URL de base |
| **API Path** | `/api` | Chemin de l'API |
| **Categories** | Toutes | Catégories à indexer |

:::warning URL de base importante
Prowlarr ne permet **pas** de personnaliser l'URL de base. Utilisez:
- **Installation locale**: `http://localhost:8715/`
- **Docker Compose**: `http://ygege:8715/` (nom du service)
- **DNS personnalisé**: `http://ygege-dns-redirect.local:8715/`
:::

### 3. Configuration Docker Compose

Si Prowlarr et Ygégé sont dans le même `compose.yml`:

```yaml
services:
  prowlarr:
    image: lscr.io/linuxserver/prowlarr:latest
    container_name: prowlarr
    # ... configuration prowlarr
  
  ygege:
    image: uwudev/ygege:latest
    container_name: ygege
    # ... configuration ygege

# Ils sont automatiquement sur le même réseau
# Utilisez http://ygege:8715/ dans Prowlarr
```

### 4. Tester la connexion

1. Cliquez sur **Test** dans la configuration de l'indexeur
2. Prowlarr devrait se connecter avec succès
3. Cliquez sur **Save**

## Utilisation

### Recherche manuelle

1. Allez dans **Search** dans Prowlarr
2. Tapez votre requête de recherche
3. Ygégé apparaîtra dans les résultats

### Synchronisation avec Sonarr/Radarr

Prowlarr synchronisera automatiquement l'indexeur Ygégé avec vos applications \*arr connectées.

## Catégories supportées

Ygégé supporte toutes les catégories YGG:

| Catégorie Prowlarr | Mapping YGG |
|-------------------|-------------|
| Movies | Films |
| TV | Séries TV |
| Audio | Musique |
| PC | Applications |
| XXX | Adulte |
| Other | Autre |

## Troubleshooting

### L'indexeur n'apparaît pas

1. Vérifiez que le fichier `ygege.yml` est bien dans `Definitions/Custom/`
2. Redémarrez Prowlarr
3. Consultez les logs Prowlarr pour les erreurs

### Erreur de connexion

1. Vérifiez que Ygégé est démarré: `curl http://localhost:8715/health`
2. Vérifiez l'URL configurée dans Prowlarr
3. Pour Docker, vérifiez que les conteneurs sont sur le même réseau

### Pas de résultats

1. Vérifiez les logs d'Ygégé: `docker logs ygege`
2. Vérifiez que vos identifiants YGG sont valides
3. Testez directement l'API: `curl http://localhost:8715/api/search?q=test`

## Prochaines étapes

- [Configuration avancée](../configuration)
- [Documentation API](../api)
- [Intégration Jackett](./jackett)
