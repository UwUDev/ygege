---
sidebar_position: 4
---

# Documentation du Workflow de Release

## Vue d'ensemble

Le workflow de release compile, teste et publie automatiquement les binaires Francisca et les images Docker lorsque vous créez un nouveau tag de version (par exemple, `v1.0.0`).

## Ce qui est compilé

### 📦 Artefacts binaires (16 au total)

Pour chacune des 8 plateformes cibles, nous créons :
- **Binaire normal** : Optimisé mais non compressé
- **Binaire UPX** : Compressé avec UPX pour une taille réduite

#### Plateformes

**Linux GNU** (4 cibles) :
- `x86_64-unknown-linux-gnu` - Intel/AMD 64-bit
- `aarch64-unknown-linux-gnu` - ARM 64-bit
- `armv7-unknown-linux-gnueabihf` - ARM 32-bit (Raspberry Pi, etc.)
- `i686-unknown-linux-gnu` - Intel/AMD 32-bit

**Windows** (2 cibles) :
- `x86_64-pc-windows-msvc` - Windows 64-bit
- `i686-pc-windows-msvc` - Windows 32-bit

**macOS** (2 cibles) :
- `x86_64-apple-darwin` - macOS Intel
- `aarch64-apple-darwin` - macOS Apple Silicon (M1/M2/M3)

### 🐳 Images Docker

Images Docker multi-architecture publiées sur :
- **Docker Hub** : `uwucode/francisca`
- **GitHub Container Registry** : `ghcr.io/uwudev/francisca`

Architectures supportées :
- `linux/amd64` - Intel/AMD 64-bit
- `linux/arm64` - ARM 64-bit

### 🔐 Fonctionnalités de sécurité

- **Signature d'image** avec Cosign (signature sans clé)
- **Génération de SBOM** utilisant Trivy (format CycloneDX)
- **Attestation SBOM** attachée aux images
- **Vérification de signature** avant publication de la release

## Comment créer une release

### 1. Préparer votre code

Assurez-vous que tous les changements sont mergés dans la branche `master` :
```bash
git checkout master
git pull origin master
```

### 2. Créer et pousser un tag

```bash
# Créer un tag (suivez le versioning sémantique)
git tag v1.0.0

# Pousser le tag pour déclencher le workflow de release
git push origin v1.0.0
```

### 3. Surveiller le workflow

Allez sur : `https://github.com/UwUDev/francisca/actions`

Le workflow va :
1. ✅ Générer le changelog depuis les commits
2. ✅ Créer un brouillon de release
3. ✅ Compiler tous les 16 artefacts binaires en parallèle
4. ✅ Uploader les binaires vers le brouillon de release
5. ✅ Compiler et publier les images Docker
6. ✅ Signer les images et créer les SBOMs
7. ✅ Uploader les SBOMs vers le brouillon de release
8. ✅ Vérifier les signatures et attestations
9. ✅ Publier la release (la rendre publique)
10. ✅ Envoyer une notification Discord (si webhook configuré)

### 4. Revoir et personnaliser

Après que le workflow se termine, vous pouvez :
- Éditer les notes de release sur GitHub
- Ajouter des assets supplémentaires manuellement
- Mettre à jour la description

## Conventions de nommage des tags

### Releases stables
```bash
v1.0.0        # Majeur.Mineur.Patch
v2.1.3
```

### Pré-releases
```bash
v1.0.0-rc.1      # Candidat de release
```

**Note** : Utilisez le versionnement sémantique standard. Les tags Docker `:latest` seront automatiquement assignés aux releases stables.

## Tags Docker

Pour la version `v1.2.3` :
- `uwucode/francisca:1.2.3` - Version complète
- `uwucode/francisca:1.2` - Version mineure
- `uwucode/francisca:1` - Version majeure
- `uwucode/francisca:stable` - Tag stable
- `uwucode/francisca:latest` - Dernière stable (seulement si ce n'est pas une pré-release)

## Secrets requis

Configurez-les dans les paramètres de votre dépôt GitHub :

### Obligatoires
- `DOCKERHUB_USERNAME` - Votre nom d'utilisateur Docker Hub
- `DOCKERHUB_TOKEN` - Token d'accès Docker Hub (créer sur hub.docker.com)

### Optionnels
- `DISCORD_WEBHOOK` - URL du webhook Discord pour les notifications

## Dépannage

### La compilation échoue sur une architecture spécifique

Vérifiez les logs du job pour cette plateforme spécifique :
1. Allez dans l'onglet Actions
2. Cliquez sur l'exécution du workflow qui a échoué
3. Cliquez sur le job qui a échoué (par ex., "Build Linux GNU (aarch64-unknown-linux-gnu)")
4. Consultez les logs

Problèmes courants :
- Dépendances de cross-compilation manquantes
- Erreurs de code spécifiques à l'architecture
- Problèmes de linkeur

### Le push Docker échoue

**Problème** : `unauthorized: authentication required`

**Solution** : Vérifiez que les secrets `DOCKERHUB_USERNAME` et `DOCKERHUB_TOKEN` sont correctement configurés.

### La compression UPX échoue

Certains binaires peuvent échouer la compression UPX. Le workflow échouera si UPX retourne une erreur.

**Contournement** : Vous pouvez modifier le workflow pour utiliser `continue-on-error: true` pour l'étape UPX.

### La vérification de signature échoue

Cela signifie que les images n'ont pas été signées correctement. Vérifiez :
1. L'installation de Cosign a réussi
2. Le token OIDC a été obtenu
3. Les images ont été poussées avec succès

## Assets de release

Chaque release inclut :

### Fichiers binaires
```
francisca-x86_64-unknown-linux-gnu          # 8 binaires normaux
francisca-x86_64-unknown-linux-gnu-upx      # 8 binaires compressés UPX
francisca-x86_64-pc-windows-msvc.exe
francisca-x86_64-pc-windows-msvc-upx.exe
... (et ainsi de suite pour toutes les plateformes)
```

### Fichiers SBOM
```
francisca-ghcr-image-v1.0.0.sbom           # SBOM pour l'image GHCR
francisca-dockerhub-image-v1.0.0.sbom      # SBOM pour l'image Docker Hub
```

## Informations de version

Tous les binaires incluent des informations de version intégrées :
- SHA du commit de build
- Date de build
- Branche/tag de build

Voir les infos de version :
```bash
./francisca --version
```

## Durée du workflow

Temps approximatifs :
- **Génération du changelog** : ~10 secondes
- **Compilations binaires** : 5-15 minutes (parallèle)
- **Compilation Docker** : 10-20 minutes
- **Signature & vérification** : 2-5 minutes
- **Total** : ~20-40 minutes

## Annuler une release

Si vous devez arrêter une release en cours :

1. Annulez le workflow dans GitHub Actions
2. Supprimez le brouillon de release s'il a été créé
3. Supprimez le tag localement et à distance :
```bash
git tag -d v1.0.0
git push origin :refs/tags/v1.0.0
```

## Bonnes pratiques

1. **Testez avant de tagger** : Exécutez d'abord le workflow CI sur `develop`
2. **Utilisez le versioning sémantique** : Suivez le format MAJEUR.MINEUR.PATCH
3. **Écrivez de bons messages de commit** : Ils apparaîtront dans le changelog
4. **Revoyez le brouillon** : Vérifiez toujours le brouillon de release avant qu'il ne soit public
5. **Ne forcez pas les push de tags** : Les tags doivent être immuables

## Intégration avec le workflow CI

Le workflow de release est séparé du workflow CI :

| Workflow | Déclencheur | Objectif | Artefacts |
|----------|-------------|----------|-----------|
| **CI** | Push sur les branches | Test & preview | Artefacts temporaires de 7 jours |
| **Release** | Push de tags | Release production | Release GitHub permanente |

## Questions ?

- **Pourquoi 16 binaires ?** Pour supporter toutes les plateformes majeures (8 plateformes × 2 versions)
- **Pourquoi les versions UPX ?** Taille de téléchargement réduite pour les utilisateurs avec bande passante limitée
- **Pourquoi signer les images Docker ?** Sécurité et intégrité de la chaîne d'approvisionnement
- **Pourquoi deux registres ?** Docker Hub pour l'accès public, GHCR comme backup
- **Puis-je sauter certaines plateformes ?** Oui, supprimez-les simplement de la matrice dans le workflow

## Exemple : Processus complet de release

```bash
# 1. Finir votre fonctionnalité
git checkout develop
git commit -am "feat: ajouter une fonctionnalité géniale"
git push origin develop

# 2. Vérifier que le CI passe
# Vérifier : https://github.com/UwUDev/francisca/actions

# 3. Merger vers master
git checkout master
git merge develop
git push origin master

# 4. Créer le tag de release
git tag v1.2.0
git push origin v1.2.0

# 5. Surveiller le workflow de release
# Vérifier : https://github.com/UwUDev/francisca/actions

# 6. Célébrer ! 🎉
```

---

**Dernière mise à jour** : 11 novembre 2025  
**Version du workflow** : 2.0 (Multi-arch avec auto-upload)
