---
sidebar_position: 2
---

# Intégration Jackett

Francisca peut être utilisé comme indexeur personnalisé pour Jackett via le système Cardigann.

## Prérequis

- Jackett installé et fonctionnel
- Francisca démarré et accessible
- Le fichier `francisca.yml` du dépôt GitHub

## Installation

### 1. Localiser le dossier AppData de Jackett

Le chemin dépend de votre installation:

| Installation | Chemin AppData |
|--------------|----------------|
| **LinuxServer Docker** | `/config` |
| **Windows** | `C:\ProgramData\Jackett` |
| **Linux** | `~/.config/Jackett` |
| **macOS** | `~/Library/Application Support/Jackett` |

### 2. Créer la structure Cardigann

Dans le dossier AppData, créez la structure `cardigann/definitions/` si elle n'existe pas:

```bash
mkdir -p /config/cardigann/definitions
```

### 3. Copier le fichier de définition

Téléchargez et copiez le fichier `francisca.yml`:

```bash
# Télécharger depuis GitHub
wget https://raw.githubusercontent.com/UwUDev/francisca/master/francisca.yml \
  -O /config/cardigann/definitions/francisca.yml
```

Ou manuellement:
1. Téléchargez [`francisca.yml`](https://github.com/UwUDev/francisca/blob/master/francisca.yml)
2. Placez-le dans `{appdata}/cardigann/definitions/`

:::tip LinuxServer Docker
L'image LinuxServer de Jackett fournit déjà une structure de dossiers bien organisée. Si vous utilisez une autre image Docker, adaptez les chemins en conséquence.
:::

### 4. Redémarrer Jackett

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

<Tabs groupId="runtime">
  <TabItem value="docker" label="Docker" default>

```bash
docker restart jackett
```

  </TabItem>
  <TabItem value="systemd" label="Systemd">

```bash
systemctl restart jackett
```

  </TabItem>
</Tabs>

## Configuration de l'indexeur

### 1. Ajouter l'indexeur

1. Ouvrez l'interface Jackett
2. Cliquez sur **Add indexer**
3. Recherchez "Francisca" dans la liste
4. Cliquez sur le bouton **+** à côté de Francisca

<!-- TODO: Ajouter screenshot de la liste Jackett avec Francisca -->
<!-- ![Jackett Add Indexer](/img/jackett-add-indexer.png) -->

### 2. Configurer les paramètres

<!-- TODO: Ajouter screenshot du formulaire de configuration Francisca dans Jackett -->
<!-- ![Jackett Francisca Configuration](/img/jackett-francisca-config.png) -->

Dans la fenêtre de configuration, saisissez:

| Paramètre | Valeur | Description |
|-----------|--------|-------------|
| **Indexer URL** | `http://localhost:8715` | URL de base de Francisca |

:::info Configuration centralisée
Si vous avez déjà configuré les identifiants dans le `config.json` de Francisca, vous n'avez pas besoin de les ressaisir ici.
:::

### 3. Tester la connexion

1. Cliquez sur **OK** pour sauvegarder
2. Jackett testera automatiquement la connexion
3. Un message de succès devrait apparaître

## Configuration Docker Compose

Si Jackett et Francisca sont dans le même `compose.yml`:

```yaml
services:
  jackett:
    image: lscr.io/linuxserver/jackett:latest
    container_name: jackett
    volumes:
      - ./jackett:/config
    ports:
      - "9117:9117"
    restart: unless-stopped
  
  francisca:
    image: uwucode/francisca:latest
    container_name: francisca
    volumes:
      - ./config:/config
    ports:
      - "8715:8715"
    environment:
      LOG_LEVEL: "info"
    restart: unless-stopped
```

Dans ce cas, utilisez `http://francisca:8715` comme URL dans la configuration Jackett.

## Utilisation

### Recherche manuelle

1. Dans Jackett, allez sur la page d'accueil
2. Utilisez la barre de recherche
3. Francisca apparaîtra dans les résultats

### Intégration avec Sonarr/Radarr

1. Copiez l'URL Torznab depuis Jackett (cliquez sur **Copy Torznab Feed**)
2. Dans Sonarr/Radarr, ajoutez Jackett comme indexeur
3. Collez l'URL Torznab
4. Les résultats de Francisca seront automatiquement intégrés

## Catégories supportées

| ID Catégorie | Nom | Description |
|--------------|-----|-------------|
| 2000 | Movies | Films |
| 5000 | TV | Séries TV |
| 3000 | Audio | Musique |
| 4000 | PC | Applications/Logiciels |
| 6000 | XXX | Contenu adulte |
| 8000 | Other | Autres |

## Recherche avancée

Francisca supporte plusieurs paramètres de recherche:

### Par nom
```
Vaiana 2
```

### Par catégorie
Sélectionnez les catégories dans l'interface Jackett

### Par saison/épisode (TV)
```
Breaking Bad S01E01
```

### Par IMDB ID
```
tt0903747
```

## Troubleshooting

### L'indexeur n'apparaît pas dans la liste

**Solution:**
1. Vérifiez que `francisca.yml` est dans `cardigann/definitions/`
2. Vérifiez les permissions du fichier (doit être lisible)
3. Redémarrez Jackett
4. Consultez les logs: `docker logs jackett`

### Erreur de connexion

**Solution:**
1. Vérifiez qu'Francisca est démarré:
   ```bash
   curl http://localhost:8715/health
   ```
2. Vérifiez l'URL configurée (localhost vs nom du conteneur)
3. Pour Docker, vérifiez que les conteneurs sont sur le même réseau

### Pas de résultats de recherche

**Solution:**
1. Testez directement l'API de Francisca:
   ```bash
   curl "http://localhost:8715/api/search?q=test"
   ```
2. Vérifiez les logs de Francisca:
   ```bash
   docker logs francisca
   ```
3. Vérifiez que le relais Nostr est accessible: `curl http://localhost:8715/status`

### Aucun résultat

**Solution:**
- Vérifiez que le relais Nostr est accessible : `curl http://localhost:8715/status`
- Consultez la [documentation de configuration](../configuration)

## Comparaison Prowlarr vs Jackett

| Fonctionnalité | Prowlarr | Jackett |
|----------------|----------|---------|
| Synchronisation \*arr | ✅ Automatique | ❌ Manuel |
| Interface moderne | ✅ | ❌ |
| Configuration | Plus complexe | Plus simple |
| Performance | Meilleure | Bonne |
| **Recommandation** | **Préféré** | Alternative |

:::tip Recommandation
Nous recommandons **Prowlarr** pour une meilleure intégration avec Sonarr/Radarr.
:::

## Prochaines étapes

- [Intégration Prowlarr](./prowlarr)
- [Configuration avancée](../configuration)
- [Documentation API](../api)
