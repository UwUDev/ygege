<p align="center">
  <img src="website/img/francisca-logo-text.png" alt="Logo Francisca" width="400"/>
</p>

<div align="right">
  <details>
    <summary>🌐 Language</summary>
    <div>
      <div align="center">
        <a href="README.md">Français</a>
        | <a href="README-en.md">English</a>
      </div>
    </div>
  </details>
</div>

Indexeur haute performance *general-purpose* pour les systèmes supportant le protocole Nostr (événements torrent
NIP-35), écrit en Rust

## https://discord.gg/rcsgdzNrvJ

Merci à tous les contributeurs. Merci à Gauvino pour le Docker, la documentation et le nettoyage post-DMCA. Merci à
Gr0lum. Merci à la personne qui a fourni sa base de données de hash. Merci à tous les utilisateurs qui ont permis de
mettre fin à ce site infernal.

Pour ceux qui arrivent ici après la guerre : il y a fort longtemps, ce repo était dédié au contournement de paywalls
sur un site de partage francophone qui ne méritait pas le moindre centime. C'est grâce à vous tous que ce site est enfin
mort, et c'est mieux ainsi. Francisco, Oracle ou même Destroy, ne reviens jamais... tu as tué le partage.

Sur ces belles paroles, et probablement avec ce dernier commit, je vous souhaite une belle journée paisible pendant que
certains se cachent au Maroc. Vive le partage, et surtout vive le partage libre et gratuit. N'oubliez jamais ce qu'il
s'est passé, cela ne doit jamais se reproduire. En espérant de tout coeur voir U2P fonctionner en production un jour.

## [DISCLAIMER LÉGAL](DISCLAIMER-fr.md)

**Caractéristiques principales** :

- Compatible avec n'importe quel relais Nostr exposant des événements torrent NIP-35
- Relais entièrement configurables, aucun compte ni authentification requis
- Classement automatique des relais par latence au démarrage
- Recherche quasi instantanée
- Consommation mémoire faible
- Recherche de torrents très modulaire (par nom, seed, leech, date de publication, etc.)
- Support Tor optionnel pour anonymiser les connexions aux relais
- Intégration TMDB/IMDB pour la résolution par identifiant
- Compatible Prowlarr, Jackett et toutes les applications \*arr

## Configuration (à faire avant le premier lancement)

Francisca ne contient **aucun relais par défaut** : vous devez en fournir au moins un (un relais Nostr exposant des
événements torrent NIP-35).

**Via `config.json`** (généré au premier lancement, à éditer) :

```json
{
    "relays": [
        "wss://u2p.anhkagi.net"
    ]
}
```

**Ou via variables d'environnement** (Docker) :

```yaml
environment:
  RELAYS: "wss://u2p.anhkagi.net"   # plusieurs relais : séparés par des virgules
```

| Clé (`config.json` / env)             | Requis  | Description                                                                                                                                         |
|---------------------------------------|---------|-----------------------------------------------------------------------------------------------------------------------------------------------------|
| `relays` / `RELAYS`                   | **oui** | Relais Nostr (NIP-35) à interroger. `wss://` (clearnet) ou `ws://…onion` (Tor). Plusieurs = tableau JSON, ou liste séparée par des virgules en env. |
| `allowed_pubkeys` / `ALLOWED_PUBKEYS` | non     | Pubkeys de publishers de confiance (hex). Défaut : publisher du réseau U2P. `[]` = accepter tout événement à signature valide.                      |
| `web_base_url` / `WEB_BASE_URL`       | non     | Base URL d'un lien « détails » par résultat. Vide = aucun lien.                                                                                     |

> [!NOTE]
> Sans relais configuré, l'application s'arrête au démarrage avec un message expliquant comment en ajouter.

## Prérequis pour la compilation

- Rust 1.85.0+

# Installation

Une image Docker prête à l'emploi est disponible pour Francisca.
Pour commencer le déploiement et la configuration de Docker, consultez
le [Guide dédié à Docker](https://francisca.lila.ws/installation/docker-guide).

> [!IMPORTANT]
> Si vous rencontrez une erreur `Permission denied` après mise à jour, consultez la
> section [Gestion des permissions](https://francisca.lila.ws/installation/docker-guide#gestion-des-permissions) du
> guide
> Docker.

## Docker

Pour créer une image Docker personnalisée avec vos propres optimisations, consultez
le [Guide de création Docker](https://francisca.lila.ws/installation/docker-guide).

## Installation manuelle

Pour compiler l'application à partir des sources, suivez
le [Guide d'installation manuel](https://francisca.lila.ws/installation/source-guide).

## Configuration IMDB et TMDB

Pour activer la récupération des métadonnées IMDB et TMDB, veuillez suivre les instructions
du [guide d'assistance TMDB et IMDB](https://francisca.lila.ws/tmdb-imdb).

## Support Tor

Francisca peut router ses connexions aux relais Nostr via Tor pour anonymiser le trafic.

| Variable d'environnement | Défaut           | Description                             |
|--------------------------|------------------|-----------------------------------------|
| `USE_TOR`                | `false`          | Activer le routage Tor (`true`/`false`) |
| `TOR_PROXY`              | `127.0.0.1:9050` | Adresse du proxy SOCKS5 Tor             |

Exemple Docker Compose :

```yaml
environment:
  USE_TOR: "true"
  TOR_PROXY: "127.0.0.1:9050"  # Optionnel si valeur par défaut
```

> [!NOTE]
> Tor doit être installé et en cours d'exécution sur votre machine (ou accessible depuis le conteneur) pour que cette
> option fonctionne.

## Intégration à Prowlarr

Francisca peut être utilisé comme indexeur personnalisé pour Prowlarr. Pour le mettre en place, trouvez votre répertoire
AppData (situé dans la page `/system/status` de Prowlarr) et copiez le fichier `francisca.yml` du repo dans le dossier
`{votre chemin appdata prowlarr}/Definitions/Custom`, vous aurez probablement besoin de créer le dossier `Custom`.

Une fois que c'est fait, redémarrez Prowlarr et allez dans les paramètres des indexeurs, vous devriez voir Francisca
dans la liste des indexeurs disponibles.

> [!NOTE]
> Prowlarr ne permet pas de personnaliser le "Base URL". Par défaut, utilisez `http://localhost:8715/`. Pour les
> configurations Docker Compose, utilisez `http://francisca:8715/`. Alternativement, utilisez
> francisca-dns-redirect.local
> avec un DNS personnalisé ou en éditant le fichier hosts.

## Intégration à Jackett

Francisca peut être utilisé comme indexeur personnalisé pour Jackett. Pour le mettre en place, localisez votre
répertoire AppData Jackett et copiez le fichier `francisca.yml` du dépôt dans le dossier
`{votre chemin appdata jackett}/cardigann/definitions/`. Vous devrez peut-être créer le sous-dossier
`cardigann/definitions/` s'il n'existe pas.

> [!NOTE]
> L'image Docker LinuxServer Jackett fournit une structure de dossiers bien organisée. Si vous utilisez une autre image
> Docker, adaptez les chemins en conséquence.

Une fois terminé, redémarrez Jackett et accédez aux paramètres des indexeurs. Vous devriez voir Francisca dans la liste
des indexeurs disponibles.

# Documentation

## Documentation utilisateur

La documentation complète est disponible sur [francisca.lila.ws](https://francisca.lila.ws) :

- [Guide de démarrage](https://francisca.lila.ws/getting-started)
- [Installation](https://francisca.lila.ws/installation/docker-guide)
- [Configuration](https://francisca.lila.ws/configuration)
- [Intégrations (Prowlarr/Jackett)](https://francisca.lila.ws/integrations/prowlarr)
- [Documentation de l'API](https://francisca.lila.ws/api)
- [FAQ](https://francisca.lila.ws/faq)

## Documentation développeur

Pour contribuer au projet ou comprendre le fonctionnement interne :

- [Guide de contribution](docs/contribution-fr.md)
- [Pipeline CI/CD](docs/ci_implementation-fr.md)
- [Workflow de preview des PRs](docs/preview_workflow-fr.md)
- [Workflow de release](docs/release_workflow-fr.md)
