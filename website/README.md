# Documentation Ygégé

Ce dossier contient la documentation complète de Ygégé construite avec Docusaurus.

## 🚀 Développement local

### Prérequis

- [Bun](https://bun.sh/) installé

### Installation

```bash
cd website
bun install
```

### Démarrer le serveur de développement

```bash
bun start
```

Le site sera accessible sur [http://localhost:3000/ygege/](http://localhost:3000/ygege/)

### Build de production

```bash
bun run build
```

Les fichiers statiques seront générés dans le dossier `build/`.

### Tester le build de production

```bash
bun run serve
```

## 🌍 Internationalisation

La documentation est disponible en deux langues :

- **Français** (par défaut) : `docs/`
- **Anglais** : `i18n/en/docusaurus-plugin-content-docs/current/`

### Ajouter/Modifier des traductions

1. Modifiez les fichiers dans `docs/` (français)
2. Créez/modifiez les fichiers correspondants dans `i18n/en/docusaurus-plugin-content-docs/current/`

### Générer les fichiers de traduction JSON

```bash
bun run write-translations --locale en
```

## 📦 Structure

```
website/
├── docs/                          # Documentation française
│   ├── intro.md
│   ├── getting-started.md
│   ├── faq.md
│   ├── docker/
│   ├── configuration/
│   ├── integrations/
│   └── api/
├── i18n/
│   └── en/                        # Traductions anglaises
│       └── docusaurus-plugin-content-docs/
│           └── current/           # Même structure que docs/
├── src/
│   └── css/                       # Styles personnalisés
├── static/                        # Fichiers statiques (images, etc.)
├── docusaurus.config.ts           # Configuration Docusaurus
└── sidebars.ts                    # Configuration sidebar
```

## 🚀 Déploiement GitHub Pages

Le déploiement est automatique via GitHub Actions :

1. **Push sur `develop` ou `master`** : déclenche le workflow
2. **Build** : Compile la documentation avec Bun
3. **Deploy** : Déploie sur GitHub Pages

### URL de production

La documentation sera accessible sur : **https://uwudev.github.io/ygege/**

### Configuration manuelle GitHub Pages

Si nécessaire, configurez GitHub Pages dans les paramètres du repository :

1. Allez dans **Settings** → **Pages**
2. Source : **GitHub Actions**
3. La configuration est déjà dans `.github/workflows/deploy-docs.yml`

## 📝 Ajouter une nouvelle page

### En français

1. Créez un fichier `.md` dans `docs/` ou un sous-dossier
2. Ajoutez le front matter :

```markdown
---
sidebar_position: 1
---

# Titre de la page

Contenu...
```

### En anglais

1. Créez le même fichier dans `i18n/en/docusaurus-plugin-content-docs/current/`
2. Traduisez le contenu

## 🔧 Configuration

### Modifier l'URL de base

Éditez `docusaurus.config.ts` :

```typescript
{
  url: 'https://uwudev.github.io',
  baseUrl: '/ygege/',
}
```

### Modifier les couleurs du thème

Éditez `src/css/custom.css` :

```css
:root {
  --ifm-color-primary: #2e8555;
}
```

## 📚 Pages disponibles

### Français

- **Introduction** : Vue d'ensemble de Ygégé
- **Guide de démarrage** : Installation et configuration pas à pas
- **FAQ** : Questions fréquentes
- **Docker** : Guide d'installation avec Docker
- **Configuration** : Options de configuration
- **Intégrations** :
  - Prowlarr
  - Jackett
- **API** : Documentation complète de l'API REST

### Anglais

Toutes les pages françaises sont traduites en anglais.

## 🛠️ Commandes utiles

```bash
# Développement
bun start                          # Serveur de développement
bun start -- --locale en          # Serveur en anglais uniquement

# Build
bun run build                      # Build production (toutes les langues)
bun run build -- --locale fr      # Build français uniquement

# Utilitaires
bun run clear                      # Nettoyer le cache
bun run write-translations         # Générer les fichiers de traduction
bun run serve                      # Servir le build de production

# Formatting
bun run format                     # Formatter le code (si configuré)
```

## 🐛 Dépannage

### Le serveur ne démarre pas

```bash
# Nettoyer et réinstaller
rm -rf node_modules .docusaurus
bun install
```

### Erreurs de build

```bash
# Vérifier les liens cassés
bun run build

# Les erreurs de liens cassés apparaîtront dans la console
```

### Problèmes de traduction

```bash
# Régénérer les fichiers de traduction
bun run write-translations --locale en
```

## 📖 Ressources

- [Documentation Docusaurus](https://docusaurus.io/)
- [Guide i18n](https://docusaurus.io/docs/i18n/introduction)
- [Markdown Features](https://docusaurus.io/docs/markdown-features)

## 🤝 Contribution

Pour contribuer à la documentation :

1. Fork le repository
2. Créez une branche : `git checkout -b docs/ma-nouvelle-page`
3. Ajoutez/modifiez la documentation
4. Testez localement : `bun start`
5. Commit : `git commit -m "docs: ajout de ..."`
6. Push et créez une Pull Request

### Guidelines

- Utilisez un langage clair et simple
- Ajoutez des exemples de code quand c'est pertinent
- Traduisez toujours en français ET en anglais
- Testez les liens avant de commit
- Utilisez les admonitions (:::tip, :::warning, etc.) pour les informations importantes

## 📄 Licence

La documentation est sous la même licence que le projet Ygégé.
