---
sidebar_position: 5
---

# FAQ - Questions fréquentes

Retrouvez ici les réponses aux questions les plus courantes sur Francisca.

## Général

### Qu'est-ce qu'Francisca exactement ?

Francisca est un **indexeur** générique pour les relais Nostr (NIP-35). Il transforme n'importe quel relais Nostr compatible en une source compatible avec Prowlarr, Jackett, Sonarr, Radarr et autres applications de gestion de médias. Il expose une API REST qui permet de rechercher des torrents et d'obtenir leurs liens magnets via le protocole **Nostr** (NIP-35).

### Pourquoi utiliser Francisca plutôt que les définitions Cardigann existantes ?

- **Performance** : Écrit en Rust, 10-20x plus rapide que les scrapers Python/Node.js
- **Nostr natif** : Connexion directe aux relais Nostr, sans scraping HTML
- **TMDB/IMDB** : Enrichissement automatique des métadonnées
- **Maintenance active** : Mises à jour régulières et support de la communauté
- **Multi-architecture** : Support ARM64/ARMv7 pour NAS et Raspberry Pi

### Francisca est-il légal ?

Francisca est un logiciel open-source qui fournit une interface technique générique vers les relais Nostr. L'utilisation des relais et le téléchargement de contenus protégés dépendent de la législation de votre pays. **Utilisez Francisca de manière responsable et légale.**

### Francisca fonctionne-t-il avec d'autres trackers ?

Oui. Francisca est *general-purpose* : il fonctionne avec n'importe quel relais Nostr exposant des événements torrent NIP-35.

## Installation et configuration

### Dois-je avoir un compte ?

**Ça dépend du relais.** De nombreux relais Nostr sont **publics** — aucun compte ni identifiant requis. Francisca se connecte directement au relais Nostr configuré.

### Quelle est la différence entre Docker Run et Docker Compose ?

- **Docker Run** : Commande unique pour démarrer rapidement, mais difficile à maintenir
- **Docker Compose** : Fichier de configuration réutilisable, facilite les mises à jour et la gestion

**Recommandation** : Utilisez Docker Compose pour une meilleure gestion à long terme.

### Puis-je installer Francisca sans Docker ?

Oui, vous avez deux options :

1. **Binaires pré-compilés (Recommandé)** : Téléchargez le binaire pour votre plateforme depuis les [releases GitHub](https://github.com/UwUDev/francisca/releases)
2. **Compilation manuelle** : Installez Rust et compilez depuis les sources

Voir le [guide de compilation](https://github.com/UwUDev/francisca#building-from-source) pour plus de détails.

### Le port 8715 est-il obligatoire ?

Non, vous pouvez utiliser n'importe quel port libre. Modifiez simplement :

```yaml
environment:
  BIND_PORT: "9090"
ports:
  - "9090:9090"
```

## Intégrations

### Prowlarr ou Jackett, lequel choisir ?

**Prowlarr (recommandé)** :
- ✅ Synchronisation automatique avec Sonarr/Radarr/Lidarr
- ✅ Interface moderne
- ✅ Meilleures performances
- ❌ Configuration initiale plus complexe

**Jackett** :
- ✅ Configuration plus simple
- ✅ Stable et éprouvé
- ❌ Synchronisation manuelle avec les \*arr
- ❌ Interface datée

### Puis-je utiliser Prowlarr ET Jackett en même temps ?

Techniquement oui, mais ce n'est **pas recommandé**. Cela créerait des doublons dans vos applications Sonarr/Radarr. Choisissez-en un seul.

### Francisca fonctionne-t-il avec Sonarr/Radarr directement ?

Non. Sonarr et Radarr nécessitent un indexeur intermédiaire (Prowlarr ou Jackett). Francisca ne supporte pas le protocole Torznab directement.

**Workflow recommandé** :
```
Francisca → Prowlarr → Sonarr/Radarr/Lidarr
```

### Comment mettre à jour le fichier francisca.yml ?

Le fichier `francisca.yml` définit l'indexeur pour Prowlarr/Jackett. Lorsqu'il est mis à jour sur GitHub :

1. Téléchargez la nouvelle version
2. Remplacez l'ancien fichier dans `Definitions/Custom/`
3. Redémarrez Prowlarr/Jackett

:::tip Notifications
Surveillez les [releases GitHub](https://github.com/UwUDev/francisca/releases) pour être notifié des mises à jour.
:::

## Performances et limites

### Combien de requêtes puis-je faire ?

Avec un relais public, il n'y a pas de limite liée aux identifiants. Cependant, pour ne pas surcharger le relais Nostr, il est recommandé de ne pas abuser des recherches automatisées.

:::info Rate-limit
Espacez vos requêtes (environ 1 par seconde) pour éviter de surcharger le relais Nostr.
:::

### Francisca met en cache les résultats ?

Non, chaque recherche interroge le relais Nostr en temps réel. Cela garantit des résultats toujours à jour.

### Le relais Nostr peut-il être inaccessible ?

Oui, dans de rares cas :
- Le relais Nostr configuré est temporairement indisponible
- Problème réseau temporaire

**Solution** : Vérifiez les logs, attendez quelques minutes, et réessayez.

### Quelle est la charge sur le relais ?

Francisca optimise les requêtes :
- 1 requête = 1 filtre NIP-50 envoyé au relais Nostr
- Pas de spam ou de requêtes abusives

## Problèmes courants

### "Rate limited" / "Aucun résultat"

**Causes possibles** :
1. Le relais Nostr est temporairement surchargé
2. Trop de requêtes envoyées en peu de temps

**Solutions** :
1. Vérifiez les logs : `docker logs francisca`
2. Attendez quelques minutes avant de réessayer
3. Réduisez la fréquence des recherches automatisées
4. Redémarrez Francisca

### "Connection refused" sur localhost:8715

**Causes possibles** :
1. Francisca n'est pas démarré
2. Le port est différent
3. Problème de firewall

**Diagnostic** :
```bash
docker ps | grep francisca        # Vérifier que le conteneur tourne
docker logs francisca             # Voir les erreurs
curl http://localhost:8715/health  # Tester l'API
```

### Aucun résultat dans Prowlarr/Jackett

**Checklist** :
- [ ] Francisca est démarré : `curl http://localhost:8715/health`
- [ ] URL correcte dans Prowlarr/Jackett (`http://localhost:8715/` ou `http://francisca:8715/`)
- [ ] Fichier `francisca.yml` à jour
- [ ] Prowlarr/Jackett redémarré après ajout du fichier
- [ ] Relais Nostr accessible : `curl http://localhost:8715/status`

### Erreur 503 "Service Unavailable"

Le relais Nostr est temporairement indisponible. Attendez et réessayez.

### Les téléchargements ne démarrent pas

Francisca fournit des **liens magnet** (pas de fichiers `.torrent`). Le téléchargement effectif est géré par :
- Votre client BitTorrent (qBittorrent, Transmission, etc.)
- Sonarr/Radarr (si configuré avec un client torrent)

Vérifiez la configuration de votre client BitTorrent et assurez-vous que Prowlarr/Jackett est configuré pour utiliser les magnets.

## Docker et déploiement

### Puis-je utiliser Francisca sur des architectures anciennes (NAS, systèmes embarqués) ?

**Oui !** Si vous rencontrez des erreurs de segmentation (segfault) sur des architectures anciennes ou certains NAS (comme Synology), utilisez l'image `uwucode/francisca:noupx` compilée sans compression UPX :

```yaml
image: uwucode/francisca:noupx
```

Cette version est compatible avec les systèmes qui ne supportent pas les binaires compressés avec UPX.

### Francisca fonctionne-t-il sur Raspberry Pi ?

Oui ! Les images Docker supportent ARMv7 et ARM64 :
- Raspberry Pi 3/4/5 : ✅ Support complet
- Architecture : ARM64 ou ARMv7

### Comment mettre à jour Francisca ?

**Avec Docker Compose** :
```bash
docker compose pull
docker compose up -d
docker image prune -f
```

**Avec Docker Run** :
```bash
docker stop francisca
docker rm francisca
docker pull uwucode/francisca:latest
# Relancer la commande docker run
```

### Puis-je exécuter plusieurs instances de Francisca ?

Oui, mais **ce n'est généralement pas nécessaire**. Si vous le faites :
- Utilisez des ports différents
- Utilisez des noms de conteneurs différents

### Comment sauvegarder ma configuration ?

Sauvegardez simplement le dossier `./config` :

```bash
# Sauvegarder
tar -czf francisca-backup.tar.gz ./config

# Restaurer
tar -xzf francisca-backup.tar.gz
```

## TMDB et IMDB

### Comment activer TMDB/IMDB ?

1. Créez un compte sur [TMDB](https://www.themoviedb.org/)
2. Générez un token API dans les paramètres de votre compte
3. Configurez Francisca :

```yaml
environment:
  TMDB_TOKEN: "votre_token_tmdb"
```

:::info
Lorsque `TMDB_TOKEN` est configuré, les résolveurs **TMDB et IMDB** sont automatiquement activés ensemble.
:::

### À quoi servent les métadonnées TMDB/IMDB ?

Elles enrichissent automatiquement les résultats avec :
- Titres officiels
- Affiches et images
- Notes et popularité
- Correspondances exactes pour Sonarr/Radarr

### TMDB/IMDB est-il obligatoire ?

Non, c'est **optionnel**. Francisca fonctionne parfaitement sans. Les métadonnées améliorent simplement la précision des recherches.

## Sécurité

### Francisca expose-t-il mes données personnelles ?

Non. Francisca ne collecte, ne stocke ni ne transmet aucune donnée personnelle. Il communique uniquement avec :
- Les relais Nostr configurés (pour les recherches)
- TMDB (si configuré, pour les métadonnées)

### Dois-je exposer Francisca sur Internet ?

**Non, ce n'est pas recommandé.** Francisca est conçu pour être utilisé en réseau local (LAN). Si vous devez l'exposer :
- Utilisez un reverse proxy (Nginx, Traefik)
- Ajoutez une authentification (Basic Auth, OAuth)
- Utilisez HTTPS

### Francisca peut-il être piraté ?

Comme tout logiciel, des vulnérabilités peuvent exister. Pour minimiser les risques :
- Mettez à jour régulièrement
- N'exposez pas sur Internet sans protection
- Utilisez un réseau isolé si possible

## Support et contribution

### Où signaler un bug ?

Ouvrez une issue sur GitHub : [github.com/UwUDev/francisca/issues](https://github.com/UwUDev/francisca/issues)

Incluez :
- Version de Francisca
- Logs pertinents
- Configuration
- Étapes pour reproduire

### Comment contribuer au projet ?

- 🐛 Signaler des bugs
- 📖 Améliorer la documentation
- 💻 Proposer des pull requests
- ⭐ Mettre une étoile sur GitHub

Voir le [guide de contribution](https://github.com/UwUDev/francisca/blob/develop/CONTRIBUTING.md).

### Francisca est-il maintenu activement ?

Oui ! Consultez l'[historique des commits](https://github.com/UwUDev/francisca/commits) et les [releases](https://github.com/UwUDev/francisca/releases) pour voir l'activité récente.

## Autres questions

### Quelle est la différence entre les tags Docker ?

| Tag | Description | Usage |
|-----|-------------|-------|
| `latest` | Dernière version stable | Production (recommandé) |
| `stable` | Alias de `latest` | Production |
| `noupx` | Sans compression UPX | Synology NAS |
| `0.6.2` | Version spécifique | Blocage de version |
| `develop` | Version de développement | Tests uniquement |

### Puis-je utiliser Francisca commercialement ?

Francisca est sous licence open-source. Vérifiez la [LICENSE](https://github.com/UwUDev/francisca/blob/develop/LICENSE) pour les détails. L'utilisation commerciale dépend également des CGU du relais Nostr que vous utilisez.

### Francisca collecte-t-il des statistiques ?

Non. Aucune télémétrie, aucun tracking. Francisca est 100% privé et fonctionne entièrement en local.

---

**Votre question n'est pas listée ?** Consultez la [documentation complète](/) ou ouvrez une [issue sur GitHub](https://github.com/UwUDev/francisca/issues).
