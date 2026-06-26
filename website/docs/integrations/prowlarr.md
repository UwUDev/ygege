---
sidebar_position: 1
---

# Intégration Prowlarr

Francisca peut être utilisé comme indexeur personnalisé pour Prowlarr, permettant d'intégrer U2P system dans votre stack de gestion de médias.

## Prérequis

- Prowlarr installé et fonctionnel
- Francisca démarré et accessible
- Le fichier `francisca.yml` du dépôt GitHub

## Installation

### 1. Localiser le dossier AppData de Prowlarr

Le chemin du dossier AppData est affiché dans la page `/system/status` de Prowlarr.

![Prowlarr Status](/img/prowlarr-status.png)

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

Copiez le fichier `francisca.yml` (français par défaut, ou `francisca-en.yml` pour la version anglaise) du dépôt GitHub dans le dossier `Custom`:

```bash
# Télécharger directement depuis GitHub
wget https://raw.githubusercontent.com/UwUDev/francisca/master/francisca.yml \
  -O /config/Definitions/Custom/francisca.yml
```

Ou manuellement:
1. Téléchargez [`francisca.yml`](https://github.com/UwUDev/francisca/blob/master/francisca.yml)
2. Placez-le dans `{appdata}/Definitions/Custom/`

### 4. Redémarrer Prowlarr

Redémarrez Prowlarr pour qu'il détecte le nouvel indexeur:

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

<Tabs groupId="runtime">
  <TabItem value="docker" label="Docker" default>

```bash
docker restart prowlarr
```

  </TabItem>
  <TabItem value="systemd" label="Systemd">

```bash
systemctl restart prowlarr
```

  </TabItem>
</Tabs>

## Configuration de l'indexeur

### 1. Ajouter l'indexeur

1. Allez dans **Indexers**
2. Cliquez sur le bouton **+** pour ajouter un indexeur
3. Recherchez "Francisca" dans la liste
4. Cliquez sur "Francisca"

![Prowlarr Add Indexer](/img/prowlarr-add-indexer.png)

### 2. Configurer les paramètres

![Prowlarr Francisca Configuration](/img/prowlarr-francisca-config.png)

| Paramètre | Valeur | Description |
|-----------|--------|-------------|
| **Name** | Francisca | Nom de l'indexeur |
| **Enable** | ✅ | Activer l'indexeur |
| **URL** | `http://localhost:8715/` | URL de base |
| **API Path** | `/api` | Chemin de l'API |
| **Categories** | Toutes | Catégories à indexer |

:::warning URL de base importante
Prowlarr ne permet **pas** de personnaliser l'URL de base. Utilisez:
- **Installation locale**: `http://localhost:8715/`
- **Docker Compose**: `http://francisca:8715/` (nom du service)
- **DNS personnalisé**: `http://francisca-dns-redirect.local:8715/`
:::

### 3. Configuration Docker Compose

Si Prowlarr et Francisca sont dans le même `compose.yml`:

```yaml
services:
  prowlarr:
    image: lscr.io/linuxserver/prowlarr:latest
    container_name: prowlarr
    # ... configuration prowlarr
  
  francisca:
    image: uwucode/francisca:latest
    container_name: francisca
    # ... configuration francisca

# Ils sont automatiquement sur le même réseau
# Utilisez http://francisca:8715/ dans Prowlarr
```

### 4. Tester la connexion

1. Cliquez sur **Test** dans la configuration de l'indexeur
2. Prowlarr devrait se connecter avec succès
3. Cliquez sur **Save**

## Utilisation

### Recherche manuelle

1. Allez dans **Search** dans Prowlarr
2. Tapez votre requête de recherche
3. Francisca apparaîtra dans les résultats

### Synchronisation avec Sonarr/Radarr

Prowlarr synchronisera automatiquement l'indexeur Francisca avec vos applications \*arr connectées.

## Catégories supportées

Francisca supporte toutes les catégories disponibles:

| Catégorie Prowlarr | Mapping catégories |
|-------------------|-------------|
| Movies | Films |
| TV | Séries TV |
| Audio | Musique |
| PC | Applications |
| XXX | Adulte |
| Other | Autre |

## Troubleshooting

### L'indexeur n'apparaît pas

1. Vérifiez que le fichier `francisca.yml` est bien dans `Definitions/Custom/`
2. Redémarrez Prowlarr
3. Consultez les logs Prowlarr pour les erreurs

### Erreur de connexion

1. Vérifiez que Francisca est démarré: `curl http://localhost:8715/health`
2. Vérifiez l'URL configurée dans Prowlarr
3. Pour Docker, vérifiez que les conteneurs sont sur le même réseau

### Pas de résultats

1. Vérifiez les logs de Francisca: `docker logs francisca`
2. Vérifiez que le relais Nostr est accessible: `curl http://localhost:8715/status`
3. Testez directement l'API: `curl "http://localhost:8715/search?q=test"`

## Prochaines étapes

- [Configuration avancée](../configuration)
- [Documentation API](../api)
- [Intégration Jackett](./jackett)
