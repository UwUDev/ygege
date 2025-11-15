# Documentation du Workflow de Preview des PRs

## Vue d'ensemble

Le workflow de preview permet aux contributeurs et réviseurs de tester les pull requests sans compiler manuellement les images Docker. Quand une PR est étiquetée avec `preview`, une image Docker est automatiquement compilée et publiée pour faciliter les tests.

## Comment ça fonctionne

### 🏷️ Activation basée sur les labels

Le workflow de preview est **opt-in** en utilisant les labels GitHub :

1. PR créée → Pas d'image de preview (par défaut)
2. Ajout du label `preview` → Le workflow se déclenche et compile l'image Docker
3. Push de nouveaux commits → L'image de preview se met à jour automatiquement
4. Suppression du label `preview` → Le workflow arrête la compilation (les anciennes images restent accessibles)

### 🎯 Pourquoi basé sur les labels ?

- **Économiser les ressources** : Compiler les previews uniquement quand c'est nécessaire
- **Choix du contributeur** : L'auteur décide si une preview est nécessaire
- **Commodité pour la review** : Les réviseurs peuvent demander des previews pour les changements complexes
- **Contrôle des coûts** : Éviter de compiler des images Docker pour chaque PR

## Guide d'utilisation

### Pour les auteurs de PRs

#### Activer la preview pour votre PR

1. Ouvrez votre pull request sur GitHub
2. Dans la barre latérale droite, cliquez sur "Labels"
3. Ajoutez le label `preview`
4. Attendez ~10-15 minutes que la compilation se termine
5. Vérifiez les commentaires de la PR pour les commandes Docker pull

#### Mettre à jour votre preview

Poussez simplement de nouveaux commits ! L'image de preview se met à jour automatiquement quand vous poussez sur la branche de la PR.

#### Désactiver la preview

Supprimez simplement le label `preview` de votre PR.

### Pour les réviseurs

#### Demander une preview

Si vous voulez tester une PR avant de l'approuver :

1. Commentez sur la PR : "Pouvez-vous ajouter le label `preview` pour que je puisse tester ?"
2. Ou ajoutez le label vous-même (si vous avez les permissions)
3. Attendez que la compilation se termine
4. Utilisez les commandes Docker du commentaire du bot

## Tags d'image de preview

Pour la **PR #123** depuis la branche **feature/awesome** :

### Tags créés
- `uwucode/ygege:pr-123` - Tag simple avec numéro de PR (se met à jour à chaque push)
- `uwucode/ygege:pr-123-feature-awesome` - Inclut le nom de la branche
- `ghcr.io/uwudev/ygege:pr-123` - Version GHCR
- `ghcr.io/uwudev/ygege:pr-123-feature-awesome` - GHCR avec la branche

### Quel tag utiliser ?

**Tests rapides** : Utilisez `pr-123` (plus court)
```bash
docker pull uwucode/ygege:pr-123
```

**Plusieurs PRs** : Utilisez `pr-123-feature-awesome` (plus descriptif)
```bash
docker pull uwucode/ygege:pr-123-feature-awesome
```

## Tester une preview

### Test rapide

```bash
# Récupérer l'image de preview
docker pull uwucode/ygege:pr-123

# Vérifier les infos de version
docker run --rm uwucode/ygege:pr-123 --version

# Exécuter la preview
docker run -p 8080:8080 uwucode/ygege:pr-123
```

### Utiliser Docker Compose

Créez `docker-compose.preview.yml` :

```yaml
services:
  ygege-preview:
    image: uwucode/ygege:pr-123
    ports:
      - "8080:8080"
    environment:
      - LOG_LEVEL=debug
    volumes:
      - ./config:/config
```

Exécutez-le :
```bash
docker compose -f docker-compose.preview.yml up
```

### Comparer avec la production

```bash
# Récupérer les deux images
docker pull uwucode/ygege:latest
docker pull uwucode/ygege:pr-123

# Comparer les tailles
docker images | grep ygege

# Exécuter côte à côte
docker run -p 8080:8080 uwucode/ygege:latest  # Production
docker run -p 8081:8080 uwucode/ygege:pr-123  # Preview
```

## Commentaires automatiques sur les PRs

Le workflow poste un commentaire automatisé sur votre PR avec :

### Première compilation
```markdown
## 🐳 Image Docker de Preview Prête !

Votre preview de PR a été compilée et publiée...

### Docker Hub
docker pull uwucode/ygege:pr-123
...
```

### Mises à jour suivantes
Le bot **met à jour le même commentaire** au lieu de spammer de nouveaux.

### Label supprimé
```markdown
## ℹ️ Image Docker de Preview Désactivée

Le label `preview` a été supprimé...
```

## Détails de l'image

### Ce qui est inclus
- **Architectures** : linux/amd64, linux/arm64
- **Infos de build** : SHA du commit, nom de la branche, date de compilation
- **Labels** : Numéro de PR, nom de la branche, URL de la PR
- **Registres** : Docker Hub et GitHub Container Registry

### Build Args
```dockerfile
BUILD_COMMIT=abc123def456...
BUILD_DATE=2025-11-11T12:34:56Z
BUILD_BRANCH=feature/awesome-feature
```

### Cache
- Utilise le cache GitHub Actions pour des recompilations plus rapides
- Met aussi en cache depuis l'image PR précédente si disponible
- Temps de compilation typique : 10-15 minutes (première compilation), 5-10 minutes (mises à jour)

## Déclencheurs du workflow

Le workflow de preview s'exécute quand :

| Événement | Action | Compile ? |
|-----------|--------|-----------|
| PR ouverte | `opened` | ✅ Si a le label `preview` |
| PR mise à jour | `synchronize` | ✅ Si a le label `preview` |
| PR réouverte | `reopened` | ✅ Si a le label `preview` |
| Label ajouté | `labeled` | ✅ Si le label est `preview` |
| Label supprimé | `unlabeled` | ❌ Poste un commentaire de nettoyage |

**Important** : Le workflow compile uniquement si le label `preview` est présent.

## Bonnes pratiques

### Quand utiliser la preview

✅ **Utilisez la preview pour** :
- Les fonctionnalités complexes qui changent le comportement
- Les changements UI/UX nécessitant des tests visuels
- Les améliorations de performance nécessitant des benchmarks
- Les corrections de bugs difficiles à tester localement
- Les changements affectant la configuration Docker

❌ **N'utilisez pas la preview pour** :
- Les corrections simples de documentation
- Les petites fautes de frappe ou le formatage du code
- Les changements avec des tests unitaires complets
- Les corrections de bugs en une ligne

### Pour les auteurs de PRs

1. **Ajoutez tôt** : Si vous savez que les réviseurs voudront tester, ajoutez le label immédiatement
2. **Documentez les changements** : Expliquez quoi tester dans la description de la PR
3. **Nettoyez** : Supprimez le label une fois la PR mergée ou fermée
4. **Testez localement d'abord** : Vérifiez que la compilation Docker fonctionne avant de demander une preview

### Pour les réviseurs

1. **Soyez spécifique** : Dites à l'auteur ce que vous voulez tester
2. **Partagez les résultats** : Commentez sur la PR avec vos résultats de tests
3. **Supprimez quand c'est fait** : Supprimez le label après que les tests soient terminés

## Gestion des ressources

### Coûts de compilation
- Chaque compilation de preview utilise ~10-15 minutes de temps GitHub Actions
- Les compilations multi-arch (amd64 + arm64) utilisent plus de ressources
- Les mises à jour réutilisent le cache, rendant les compilations suivantes plus rapides

### Coûts de stockage
Les images de preview restent sur Docker Hub/GHCR jusqu'à suppression manuelle.

**Suggestions de nettoyage** :
- Supprimez les images de preview après que la PR soit mergée
- Configurez un nettoyage automatisé pour les anciennes images de PR (optionnel)

## Dépannage

### La compilation de preview a échoué

**Problème** : Le workflow affiche un ❌ rouge

**Vérifiez** :
1. Consultez les logs du workflow dans l'onglet "Actions"
2. Cherchez les erreurs de compilation dans l'étape "Build and push preview image"
3. Problèmes courants :
   - Erreurs de syntaxe Dockerfile
   - Dépendances manquantes
   - Manque de mémoire

**Solution** : Corrigez le problème dans votre branche et poussez un nouveau commit

### Impossible de récupérer l'image de preview

**Problème** : `docker pull uwucode/ygege:pr-123` échoue

**Causes possibles** :
1. La compilation n'est pas encore terminée - vérifiez le statut du workflow
2. Mauvais nom de tag - vérifiez le commentaire du bot pour le tag exact
3. Problème d'authentification au registre - essayez de récupérer depuis GHCR à la place

### L'image de preview est obsolète

**Problème** : L'image récupérée ne reflète pas les derniers commits

**Solution** :
```bash
# Forcer la récupération de la dernière version
docker pull uwucode/ygege:pr-123 --no-cache

# Ou supprimer d'abord l'ancienne image
docker rmi uwucode/ygege:pr-123
docker pull uwucode/ygege:pr-123
```

### Le commentaire du bot n'apparaît pas

**Problème** : Pas de commentaire de preview sur la PR

**Vérifiez** :
1. Le workflow s'est-il terminé avec succès ?
2. Vérifiez les logs du workflow pour l'étape "Comment on PR"
3. Le bot peut avoir des problèmes de permissions

**Contournement** : Utilisez les tags manuellement : `uwucode/ygege:pr-{numéro}`

## Considérations de sécurité

### Signature d'image

Les images de preview ne sont **PAS signées** avec Cosign (contrairement aux images de release).

**Pourquoi ?**
- Les previews sont pour les tests, pas la production
- La signature ajoute de la complexité et du temps
- Le workflow de release gère la signature de production

### Secrets

Le workflow de preview utilise :
- `DOCKERHUB_USERNAME` - Login Docker Hub
- `DOCKERHUB_TOKEN` - Token Docker Hub
- `GITHUB_TOKEN` - Fourni automatiquement par GitHub

### Confiance

⚠️ **Les images de preview ne doivent être utilisées que pour les tests**
- Ne les utilisez pas en production
- Ne faites pas confiance aux previews de contributeurs inconnus
- Revoyez toujours le code de la PR avant d'exécuter les images de preview

## Comparaison : CI vs Preview vs Release

| Workflow | Déclencheur | Tags Docker | Signé ? | Cas d'usage |
|----------|-------------|-------------|---------|-------------|
| **CI** | Push sur les branches | `develop`, `beta`, `master` | ❌ | Tests de développement |
| **Preview** | PR avec label | `pr-123` | ❌ | Tests de PR |
| **Release** | Tags de version | `v1.2.3`, `latest`, `stable` | ✅ | Production |

## Exemples

### Exemple 1 : Tester une PR de fonctionnalité

```bash
# Le réviseur ajoute le label preview à la PR #456

# Attendre la compilation (~10 min)

# Récupérer et tester
docker pull uwucode/ygege:pr-456

# Exécuter avec config personnalisée
docker run -p 8080:8080 \
  -v $(pwd)/test-config:/config \
  uwucode/ygege:pr-456

# Tester la fonctionnalité
curl http://localhost:8080/api/endpoint

# Laisser un retour sur la PR
# Supprimer le label preview quand c'est fait
```

### Exemple 2 : Auteur fournissant une preview

```bash
# L'auteur ouvre la PR #789 avec une nouvelle optimisation Docker

# L'auteur ajoute le label preview
# Ajoute un commentaire : "Image de preview disponible pour tester les améliorations de performance"

# Le workflow compile et commente avec les tags

# Les réviseurs testent et confirment les gains de performance

# PR mergée, label preview supprimé
```

### Exemple 3 : Comparer des previews

```bash
# Tester plusieurs PRs côte à côte
docker pull uwucode/ygege:pr-111  # Fonctionnalité A
docker pull uwucode/ygege:pr-222  # Fonctionnalité B

# Exécuter Fonctionnalité A sur le port 8081
docker run -d -p 8081:8080 --name preview-a uwucode/ygege:pr-111

# Exécuter Fonctionnalité B sur le port 8082
docker run -d -p 8082:8080 --name preview-b uwucode/ygege:pr-222

# Comparer les fonctionnalités
curl http://localhost:8081/api/test
curl http://localhost:8082/api/test

# Nettoyer
docker stop preview-a preview-b
docker rm preview-a preview-b
```

## FAQ

### Q : Chaque PR obtient-elle une preview ?

**R** : Non ! Seulement les PRs avec le label `preview`. Cela économise les ressources.

### Q : Combien de temps prend une compilation de preview ?

**R** : 10-15 minutes pour la première compilation, 5-10 minutes pour les mises à jour (grâce au cache).

### Q : Puis-je avoir des previews sur plusieurs branches ?

**R** : Oui ! Chaque PR obtient son propre tag unique (pr-123, pr-456, etc.).

### Q : Qu'arrive-t-il aux images de preview après que la PR soit mergée ?

**R** : Elles restent sur Docker Hub/GHCR jusqu'à suppression manuelle. Pensez à les nettoyer périodiquement.

### Q : Puis-je tester les images ARM ?

**R** : Oui ! Les images de preview sont multi-arch (amd64 + arm64).

### Q : Pourquoi ne pas simplement compiler localement ?

**R** : Vous pouvez ! Mais les images de preview sont pratiques pour les réviseurs qui ne veulent pas cloner et compiler.

### Q : Les images de preview incluent-elles le code le plus récent ?

**R** : Oui ! Elles sont compilées depuis le SHA de commit exact dans la PR.

## Trucs et astuces

### Accès rapide à la preview

Créez un alias :
```bash
alias ygege-pr='docker run --rm -p 8080:8080 uwucode/ygege:pr-'

# Utilisation
ygege-pr123  # Exécute la preview de la PR #123
```

### Script de test auto-mise à jour

```bash
#!/bin/bash
PR_NUM=$1

while true; do
  echo "Récupération de la dernière pr-$PR_NUM..."
  docker pull uwucode/ygege:pr-$PR_NUM
  
  echo "Redémarrage du conteneur..."
  docker stop ygege-preview 2>/dev/null
  docker rm ygege-preview 2>/dev/null
  docker run -d --name ygege-preview -p 8080:8080 uwucode/ygege:pr-$PR_NUM
  
  echo "Preview mise à jour ! Attente de 5 minutes..."
  sleep 300
done
```

### Tests multi-réviseurs

Utilisez Docker Compose avec variable d'environnement :
```bash
export PR_NUMBER=123
docker compose up
```

```yaml
# docker-compose.yml
services:
  ygege:
    image: uwucode/ygege:pr-${PR_NUMBER:-develop}
    ports:
      - "8080:8080"
```

---

**Dernière mise à jour** : 11 novembre 2025  
**Version du workflow** : 2.0 (Previews de PR basées sur les labels)
