---
sidebar_position: 2
---

# Intégration Jackett

Ygégé peut être utilisé comme indexeur personnalisé pour Jackett via le système Cardigann.

## Prérequis

- Jackett installé et fonctionnel
- Ygégé démarré et accessible
- Le fichier `ygege.yml` du dépôt GitHub

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

Téléchargez et copiez le fichier `ygege.yml`:

```bash
# Télécharger depuis GitHub
wget https://raw.githubusercontent.com/UwUDev/ygege/master/ygege.yml \
  -O /config/cardigann/definitions/ygege.yml
```

Ou manuellement:
1. Téléchargez [`ygege.yml`](https://github.com/UwUDev/ygege/blob/master/ygege.yml)
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
3. Recherchez "Ygégé" dans la liste
4. Cliquez sur le bouton **+** à côté de Ygégé

<!-- TODO: Ajouter screenshot de la liste Jackett avec Ygégé -->
<!-- ![Jackett Add Indexer](/img/jackett-add-indexer.png) -->

### 2. Configurer les paramètres

<!-- TODO: Ajouter screenshot du formulaire de configuration Ygégé dans Jackett -->
<!-- ![Jackett Ygege Configuration](/img/jackett-ygege-config.png) -->

Dans la fenêtre de configuration, saisissez:

| Paramètre | Valeur | Description |
|-----------|--------|-------------|
| **Indexer URL** | `http://localhost:8715` | URL de base d'Ygégé |
| **Username** | Votre username YGG | Optionnel (si non dans config) |
| **Password** | Votre password YGG | Optionnel (si non dans config) |

:::info Configuration centralisée
Si vous avez déjà configuré les identifiants dans le `config.json` d'Ygégé, vous n'avez pas besoin de les ressaisir ici.
:::

### 3. Tester la connexion

1. Cliquez sur **OK** pour sauvegarder
2. Jackett testera automatiquement la connexion
3. Un message de succès devrait apparaître

## Configuration Docker Compose

Si Jackett et Ygégé sont dans le même `compose.yml`:

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
  
  ygege:
    image: uwucode/ygege:latest
    container_name: ygege
    volumes:
      - ./config:/config
    ports:
      - "8715:8715"
    environment:
      YGG_USERNAME: "votre_username"
      YGG_PASSWORD: "votre_password"
    restart: unless-stopped
```

Dans ce cas, utilisez `http://ygege:8715` comme URL dans la configuration Jackett.

## Utilisation

### Recherche manuelle

1. Dans Jackett, allez sur la page d'accueil
2. Utilisez la barre de recherche
3. Ygégé apparaîtra dans les résultats

### Intégration avec Sonarr/Radarr

1. Copiez l'URL Torznab depuis Jackett (cliquez sur **Copy Torznab Feed**)
2. Dans Sonarr/Radarr, ajoutez Jackett comme indexeur
3. Collez l'URL Torznab
4. Les résultats d'Ygégé seront automatiquement intégrés

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

Ygégé supporte plusieurs paramètres de recherche:

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
1. Vérifiez que `ygege.yml` est dans `cardigann/definitions/`
2. Vérifiez les permissions du fichier (doit être lisible)
3. Redémarrez Jackett
4. Consultez les logs: `docker logs jackett`

### Erreur de connexion

**Solution:**
1. Vérifiez qu'Ygégé est démarré:
   ```bash
   curl http://localhost:8715/health
   ```
2. Vérifiez l'URL configurée (localhost vs nom du conteneur)
3. Pour Docker, vérifiez que les conteneurs sont sur le même réseau

### Pas de résultats de recherche

**Solution:**
1. Testez directement l'API d'Ygégé:
   ```bash
   curl "http://localhost:8715/api/search?q=test"
   ```
2. Vérifiez les logs d'Ygégé:
   ```bash
   docker logs ygege
   ```
3. Vérifiez que vos identifiants YGG sont valides

### Rate limiting YGG

**Solution:**
- Assurez-vous que les identifiants YGG sont configurés
- Vérifiez le fichier `config.json` ou les variables d'environnement
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
