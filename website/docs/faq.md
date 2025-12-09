---
sidebar_position: 5
---

# FAQ - Questions fréquentes

Retrouvez ici les réponses aux questions les plus courantes sur Ygégé.

## Général

### Qu'est-ce qu'Ygégé exactement ?

Ygégé est un **indexeur** pour YGG Torrent. Il transforme YGG en une source compatible avec Prowlarr, Jackett, Sonarr, Radarr et autres applications de gestion de médias. Il expose une API REST qui permet de rechercher et télécharger des torrents.

### Pourquoi utiliser Ygégé plutôt que les définitions Cardigann existantes ?

- **Performance** : Écrit en Rust, 10-20x plus rapide que les scrapers Python/Node.js
- **Contournement Cloudflare** : Bypass automatique et intelligent sans navigateur
- **TMDB/IMDB** : Enrichissement automatique des métadonnées
- **Maintenance active** : Mises à jour régulières et support de la communauté
- **Multi-architecture** : Support ARM64/ARMv7 pour NAS et Raspberry Pi

### Ygégé est-il légal ?

Ygégé est un logiciel open-source qui fournit une interface technique vers YGG Torrent. L'utilisation de YGG Torrent et le téléchargement de contenus protégés dépendent de la législation de votre pays. **Utilisez Ygégé de manière responsable et légale.**

### Ygégé fonctionne-t-il avec d'autres trackers ?

Non, Ygégé est spécifiquement conçu pour YGG Torrent uniquement. Pour d'autres trackers, utilisez les indexeurs natifs de Prowlarr/Jackett.

## Installation et configuration

### Dois-je avoir un compte YGG Torrent ?

**Oui, absolument obligatoire.** YGG Torrent est un tracker **privé** qui nécessite des identifiants pour accéder au site. Sans identifiants valides, Ygégé ne pourra pas se connecter.

:::warning Attention au rate-limit
Même avec des identifiants, surveillez le nombre de requêtes pour éviter d'être rate-limité par YGG. En cas de trop nombreuses requêtes en peu de temps, YGG peut temporairement bloquer votre accès.
:::

### Quelle est la différence entre Docker Run et Docker Compose ?

- **Docker Run** : Commande unique pour démarrer rapidement, mais difficile à maintenir
- **Docker Compose** : Fichier de configuration réutilisable, facilite les mises à jour et la gestion

**Recommandation** : Utilisez Docker Compose pour une meilleure gestion à long terme.

### Puis-je installer Ygégé sans Docker ?

Oui, vous avez deux options :

1. **Binaires pré-compilés (Recommandé)** : Téléchargez le binaire pour votre plateforme depuis les [releases GitHub](https://github.com/UwUDev/ygege/releases)
2. **Compilation manuelle** : Installez Rust et compilez depuis les sources

Voir le [guide de compilation](https://github.com/UwUDev/ygege#building-from-source) pour plus de détails.

### Le port 8715 est-il obligatoire ?

Non, vous pouvez utiliser n'importe quel port libre. Modifiez simplement :

```yaml
environment:
  BIND_PORT: "9090"
ports:
  - "9090:9090"
```

### Ygégé stocke-t-il mes identifiants en clair ?

Les identifiants sont stockés dans le fichier `config.json` ou les variables d'environnement. **Nous recommandons d'utiliser les variables d'environnement Docker** et de protéger l'accès à votre serveur.

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

### Ygégé fonctionne-t-il avec Sonarr/Radarr directement ?

Non. Sonarr et Radarr nécessitent un indexeur intermédiaire (Prowlarr ou Jackett). Ygégé ne supporte pas le protocole Torznab directement.

**Workflow recommandé** :
```
Ygégé → Prowlarr → Sonarr/Radarr/Lidarr
```

### Comment mettre à jour le fichier ygege.yml ?

Le fichier `ygege.yml` définit l'indexeur pour Prowlarr/Jackett. Lorsqu'il est mis à jour sur GitHub :

1. Téléchargez la nouvelle version
2. Remplacez l'ancien fichier dans `Definitions/Custom/`
3. Redémarrez Prowlarr/Jackett

:::tip Notifications
Surveillez les [releases GitHub](https://github.com/UwUDev/ygege/releases) pour être notifié des mises à jour.
:::

## Performances et limites

### Combien de requêtes puis-je faire par jour ?

YGG Torrent étant un tracker privé, vous devez **obligatoirement** avoir des identifiants valides pour utiliser Ygégé.

:::warning Rate-limit
YGG peut appliquer des limites de taux (rate-limit) si vous effectuez trop de requêtes en peu de temps. Il est recommandé de :
- Ne pas abuser des recherches automatisées
- Espacer les requêtes lorsque possible
- Surveiller les logs pour détecter d'éventuels messages de rate-limit
:::

### Ygégé met en cache les résultats ?

Non, chaque recherche interroge YGG en temps réel. Cela garantit des résultats toujours à jour.

### Le contournement Cloudflare peut-il échouer ?

Oui, dans de rares cas :
- Changement de la protection Cloudflare par YGG
- Problème réseau temporaire
- Bannissement IP (très rare)

**Solution** : Vérifiez les logs, attendez quelques minutes, et réessayez.

### Quelle est la charge sur YGG ?

Ygégé optimise les requêtes et respecte les bonnes pratiques :
- 1 requête = 1 recherche sur YGG
- Pas de spam ou de requêtes abusives
- User-Agent personnalisé pour identification

## Problèmes courants

### "Rate limited" / "Trop de requêtes"

**Causes possibles** :
1. Identifiants YGG non configurés ou invalides
2. Trop de requêtes envoyées en peu de temps vers YGG

**Solutions** :
1. Vérifiez `YGG_USERNAME` et `YGG_PASSWORD`
2. Testez vos identifiants sur le site YGG
3. Si identifiants OK, attendez quelques minutes avant de réessayer
4. Réduisez la fréquence des recherches automatisées
5. Redémarrez Ygégé

### "Connection refused" sur localhost:8715

**Causes possibles** :
1. Ygégé n'est pas démarré
2. Le port est différent
3. Problème de firewall

**Diagnostic** :
```bash
docker ps | grep ygege        # Vérifier que le conteneur tourne
docker logs ygege             # Voir les erreurs
curl http://localhost:8715/health  # Tester l'API
```

### Aucun résultat dans Prowlarr/Jackett

**Checklist** :
- [ ] Ygégé est démarré : `curl http://localhost:8715/health`
- [ ] Identifiants YGG configurés
- [ ] URL correcte dans Prowlarr/Jackett (`http://localhost:8715/` ou `http://ygege:8715/`)
- [ ] Fichier `ygege.yml` à jour
- [ ] Prowlarr/Jackett redémarré après ajout du fichier

### Erreur 503 "Service Unavailable"

YGG Torrent est temporairement indisponible ou en maintenance. Attendez que le site soit de nouveau accessible.

### Les téléchargements ne démarrent pas

Ygégé fournit uniquement les fichiers `.torrent`. Le téléchargement effectif est géré par :
- Votre client BitTorrent (qBittorrent, Transmission, etc.)
- Sonarr/Radarr (si configuré avec un client torrent)

Vérifiez la configuration de votre client BitTorrent.

## Docker et déploiement

### Puis-je utiliser Ygégé sur des architectures anciennes (NAS, systèmes embarqués) ?

**Oui !** Si vous rencontrez des erreurs de segmentation (segfault) sur des architectures anciennes ou certains NAS (comme Synology), utilisez l'image `uwucode/ygege:noupx` compilée sans compression UPX :

```yaml
image: uwucode/ygege:noupx
```

Cette version est compatible avec les systèmes qui ne supportent pas les binaires compressés avec UPX.

### Ygégé fonctionne-t-il sur Raspberry Pi ?

Oui ! Les images Docker supportent ARMv7 et ARM64 :
- Raspberry Pi 3/4/5 : ✅ Support complet
- Architecture : ARM64 ou ARMv7

### Comment mettre à jour Ygégé ?

**Avec Docker Compose** :
```bash
docker compose pull
docker compose up -d
docker image prune -f
```

**Avec Docker Run** :
```bash
docker stop ygege
docker rm ygege
docker pull uwucode/ygege:latest
# Relancer la commande docker run
```

### Puis-je exécuter plusieurs instances d'Ygégé ?

Oui, mais **ce n'est généralement pas nécessaire**. Si vous le faites :
- Utilisez des ports différents
- Utilisez des noms de conteneurs différents
- Chaque instance aura ses propres identifiants YGG

### Comment sauvegarder ma configuration ?

Sauvegardez simplement le dossier `./config` :

```bash
# Sauvegarder
tar -czf ygege-backup.tar.gz ./config

# Restaurer
tar -xzf ygege-backup.tar.gz
```

## TMDB et IMDB

### Comment activer TMDB/IMDB ?

1. Créez un compte sur [TMDB](https://www.themoviedb.org/)
2. Générez un token API dans les paramètres de votre compte
3. Configurez Ygégé :

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

Non, c'est **optionnel**. Ygégé fonctionne parfaitement sans. Les métadonnées améliorent simplement la précision des recherches.

## Sécurité

### Ygégé expose-t-il mes données personnelles ?

Non. Ygégé ne collecte, ne stocke ni ne transmet aucune donnée personnelle. Il communique uniquement avec :
- YGG Torrent (pour les recherches)
- TMDB (si configuré, pour les métadonnées)

### Dois-je exposer Ygégé sur Internet ?

**Non, ce n'est pas recommandé.** Ygégé est conçu pour être utilisé en réseau local (LAN). Si vous devez l'exposer :
- Utilisez un reverse proxy (Nginx, Traefik)
- Ajoutez une authentification (Basic Auth, OAuth)
- Utilisez HTTPS

### Ygégé peut-il être piraté ?

Comme tout logiciel, des vulnérabilités peuvent exister. Pour minimiser les risques :
- Mettez à jour régulièrement
- N'exposez pas sur Internet sans protection
- Utilisez un réseau isolé si possible

## Support et contribution

### Où signaler un bug ?

Ouvrez une issue sur GitHub : [github.com/UwUDev/ygege/issues](https://github.com/UwUDev/ygege/issues)

Incluez :
- Version d'Ygégé
- Logs pertinents
- Configuration (sans vos identifiants !)
- Étapes pour reproduire

### Comment contribuer au projet ?

- 🐛 Signaler des bugs
- 📖 Améliorer la documentation
- 💻 Proposer des pull requests
- ⭐ Mettre une étoile sur GitHub

Voir le [guide de contribution](https://github.com/UwUDev/ygege/blob/develop/CONTRIBUTING.md).

### Ygégé est-il maintenu activement ?

Oui ! Consultez l'[historique des commits](https://github.com/UwUDev/ygege/commits) et les [releases](https://github.com/UwUDev/ygege/releases) pour voir l'activité récente.

## Autres questions

### Quelle est la différence entre les tags Docker ?

| Tag | Description | Usage |
|-----|-------------|-------|
| `latest` | Dernière version stable | Production (recommandé) |
| `stable` | Alias de `latest` | Production |
| `noupx` | Sans compression UPX | Synology NAS |
| `0.6.2` | Version spécifique | Blocage de version |
| `develop` | Version de développement | Tests uniquement |

### Puis-je utiliser Ygégé commercialement ?

Ygégé est sous licence open-source. Vérifiez la [LICENSE](https://github.com/UwUDev/ygege/blob/develop/LICENSE) pour les détails. L'utilisation commerciale dépend également des CGU de YGG Torrent.

### Ygégé collecte-t-il des statistiques ?

Non. Aucune télémétrie, aucun tracking. Ygégé est 100% privé et fonctionne entièrement en local.

---

**Votre question n'est pas listée ?** Consultez la [documentation complète](/) ou ouvrez une [issue sur GitHub](https://github.com/UwUDev/ygege/issues).
