<p align="center">
  <img src="website/img/ygege-logo-text.png" alt="Logo Ygégé" width="400"/>
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

Indexeur haute performance pour [ygg.gratis](https://ygg.gratis) via le protocole Nostr, écrit en Rust

## https://discord.gg/rcsgdzNrvJ

## [DISCLAIMER LÉGAL](DISCLAIMER-fr.md)

**Caractéristiques principales** :
- Connexion au relais Nostr de ygg.gratis (`wss://relay.ygg.gratis`)
- Aucun compte ni identifiant requis — ygg.gratis est public
- Classement automatique des relais par latence au démarrage
- Recherche quasi instantanée
- Consommation mémoire faible
- Recherche de torrents très modulaire (par nom, seed, leech, date de publication, etc.)
- Support Tor optionnel pour anonymiser les connexions aux relais
- Intégration TMDB/IMDB pour la résolution par identifiant
- Compatible Prowlarr, Jackett et toutes les applications \*arr

## Prérequis pour la compilation
- Rust 1.85.0+

# Installation

Une image Docker prête à l'emploi est disponible pour Ygégé.
Pour commencer le déploiement et la configuration de Docker, consultez le [Guide dédié à Docker](https://ygege.lila.ws/installation/docker-guide).

> [!IMPORTANT]
> Si vous rencontrez une erreur `Permission denied` après mise à jour, consultez la section [Gestion des permissions](https://ygege.lila.ws/installation/docker-guide#gestion-des-permissions) du guide Docker.

## Docker

Pour créer une image Docker personnalisée avec vos propres optimisations, consultez le [Guide de création Docker](https://ygege.lila.ws/installation/docker-guide).

## Installation manuelle

Pour compiler l'application à partir des sources, suivez le [Guide d'installation manuel](https://ygege.lila.ws/installation/source-guide).

## Configuration IMDB et TMDB

Pour activer la récupération des métadonnées IMDB et TMDB, veuillez suivre les instructions du [guide d'assistance TMDB et IMDB](https://ygege.lila.ws/tmdb-imdb).

## Support Tor

Ygégé peut router ses connexions aux relais Nostr via Tor pour anonymiser le trafic.

| Variable d'environnement | Défaut | Description |
|--------------------------|--------|-------------|
| `USE_TOR` | `false` | Activer le routage Tor (`true`/`false`) |
| `TOR_PROXY` | `127.0.0.1:9050` | Adresse du proxy SOCKS5 Tor |

Exemple Docker Compose :

```yaml
environment:
  USE_TOR: "true"
  TOR_PROXY: "127.0.0.1:9050"  # Optionnel si valeur par défaut
```

> [!NOTE]
> Tor doit être installé et en cours d'exécution sur votre machine (ou accessible depuis le conteneur) pour que cette option fonctionne.

## Intégration à Prowlarr

Ygégé peut être utilisé comme indexeur personnalisé pour Prowlarr. Pour le mettre en place, trouvez votre répertoire AppData (situé dans la page `/system/status` de Prowlarr) et copiez le fichier `ygege.yml` du repo dans le dossier `{votre chemin appdata prowlarr}/Definitions/Custom`, vous aurez probablement besoin de créer le dossier `Custom`.

Une fois que c'est fait, redémarrez Prowlarr et allez dans les paramètres des indexeurs, vous devriez voir Ygégé dans la liste des indexeurs disponibles.

> [!NOTE]
> Prowlarr ne permet pas de personnaliser le "Base URL". Par défaut, utilisez `http://localhost:8715/`. Pour les configurations Docker Compose, utilisez `http://ygege:8715/`. Alternativement, utilisez ygege-dns-redirect.local avec un DNS personnalisé ou en éditant le fichier hosts.

## Intégration à Jackett

Ygégé peut être utilisé comme indexeur personnalisé pour Jackett. Pour le mettre en place, localisez votre répertoire AppData Jackett et copiez le fichier `ygege.yml` du dépôt dans le dossier `{votre chemin appdata jackett}/cardigann/definitions/`. Vous devrez peut-être créer le sous-dossier `cardigann/definitions/` s'il n'existe pas.

> [!NOTE]
> L'image Docker LinuxServer Jackett fournit une structure de dossiers bien organisée. Si vous utilisez une autre image Docker, adaptez les chemins en conséquence.

Une fois terminé, redémarrez Jackett et accédez aux paramètres des indexeurs. Vous devriez voir Ygégé dans la liste des indexeurs disponibles.

# Documentation

## Documentation utilisateur

La documentation complète est disponible sur [ygege.lila.ws](https://ygege.lila.ws) :
- [Guide de démarrage](https://ygege.lila.ws/getting-started)
- [Installation](https://ygege.lila.ws/installation/docker-guide)
- [Configuration](https://ygege.lila.ws/configuration)
- [Intégrations (Prowlarr/Jackett)](https://ygege.lila.ws/integrations/prowlarr)
- [Documentation de l'API](https://ygege.lila.ws/api)
- [FAQ](https://ygege.lila.ws/faq)

## Documentation développeur

Pour contribuer au projet ou comprendre le fonctionnement interne :
- [Guide de contribution](docs/contribution-fr.md)
- [Pipeline CI/CD](docs/ci_implementation-fr.md)
- [Workflow de preview des PRs](docs/preview_workflow-fr.md)
- [Workflow de release](docs/release_workflow-fr.md)
