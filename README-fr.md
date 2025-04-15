# Ygégé

- [English](README.md)

Indexeur haute performance pour YGG Torrent écrit en Rust 

**Caractéristiques principales** :
- Résolution automatique du domaine actuel de YGG Torrent
- Bypass Cloudflare automatisé (sans résolution manuelle)
- Recherche quasi instantanée
- Reconnexion transparente aux sessions expirées
- Caching des sessions
- Contournement des DNS menteurs
- Consommation mémoire faible (14.7Mo en mode release sur Linux)
- Recherche de torrents très modulaire (par nom, seed, leech, commentaires, date de publication, etc.)
- Recuperation des informations complémentaires sur les torrents (description, taille, nombre de seeders, leechers, etc.)
- Pas de dépendances externes
- Pas de drivers de navigateur

## Prérequis pour la compilation
- Rust 1.85.0+
- OpenSSL 3+
- Toutes les dépendances requises pour la compilation de [rquest](https://crates.io/crates/rquest)

## Installation

`TODO`

Pour les fans de Docker, n'hésitez pas à contribuer au projet en m'aidant à créer une image Docker.

## Contournement cloudflare
Pour contourner le défi de Cloudflare, Ygégé n'utilise pas de navigateur ni de services tiers.

Une règle cloudflare est appliquée sur le site YGG Torrent pour empêcher l'apparition du challenge cloudflare via le cookie `account_created=true` censé garantir que l'utilisateur a un compte valide et est connecté.

Mais ce n'est pas si simple, Cloudflare vous surveille toujours et détecte les faux clients HTTPS et les faux navigateurs.

Pour contourner cela, Ygégé utilise la librairie [rquest](https://crates.io/crates/rquest) qui est un client HTTP basé sur `reqwest` et `tokio` permettant de reproduire 1:1 l'échange TLS et HTTP/2 avec le serveur afin de simuler un vrai navigateur.

J'ai aussi remarqué que cela ne passait plus à partir de Chrome 133, sûrement à cause de l'integration de HTTP/3 dans Chrome qui n'est pas encore simulée par `rquest`.

Je recommande aux curieux [cet article](https://fingerprint.com/blog/what-is-tls-fingerprinting-transport-layer-security/) qui explique comment fonctionne le fingerprinting TLS et [cet autre article](https://www.trickster.dev/post/understanding-http2-fingerprinting/) qui explique comment fonctionne le fingerprinting HTTP/2 et comment il est possible de le contourner.

## Test de performance

Query pour la recherche:
- Nom: `Vaiana 2`
- Tri: `seeders`
- Ordre: `descendant`

|                                     | Nombre de tests | Temps total de tous les tests | Temps moyen par test |
|-------------------------------------|-----------------|-------------------------------|----------------------|
| Résolution du domaine actuel de YGG |        25       |           3220,378ms          |      128,81512ms     |
| Nouvelle connection YGG             |        10       |          4881.71361ms         |     488.1713616ms    |
| Restoration de session YGG          |        10       |         2064.672142ms         |     206.4672142ms    |
| Recherche                           |       100       |         17621.045874ms        |    176,21045874ms    |