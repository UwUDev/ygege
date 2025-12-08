---
sidebar_position: 2
---

# Installation avec binaires précompilés

Ce guide explique comment installer et utiliser Ygégé avec les binaires précompilés fournis à chaque release.

## Prérequis

- Système d'exploitation supporté : Linux, Windows, macOS
- Aucune dépendance externe requise (binaires statiques)

## Téléchargement

### Option 1 : Depuis GitHub Releases (Recommandé)

1. Rendez-vous sur la [page des releases](https://github.com/UwUDev/ygege/releases)
2. Téléchargez le binaire correspondant à votre plateforme :
   - **Linux AMD64** : `ygege-linux-x86_64`
   - **Linux ARM64** : `ygege-linux-aarch64`
   - **Linux ARMv7** : `ygege-linux-armv7`
   - **Windows AMD64** : `ygege-windows-x86_64.exe`
   - **macOS Intel** : `ygege-macos-x86_64`
   - **macOS Apple Silicon** : `ygege-macos-aarch64`

### Option 2 : Via wget/curl (Linux/macOS)

```bash
# Remplacez VERSION par la version souhaitée (ex: v1.0.0)
# Remplacez PLATFORM par votre plateforme (ex: linux-x86_64)
wget https://github.com/UwUDev/ygege/releases/download/VERSION/ygege-PLATFORM

# Ou avec curl
curl -L -o ygege https://github.com/UwUDev/ygege/releases/download/VERSION/ygege-PLATFORM
```

## Installation

### Linux / macOS

```bash
# Rendre le binaire exécutable
chmod +x ygege-*

# Déplacer dans un dossier du PATH (optionnel)
sudo mv ygege-* /usr/local/bin/ygege

# Vérifier l'installation
ygege --version
```

### Windows

1. Créez un dossier `C:\Program Files\Ygege\`
2. Déplacez `ygege-windows-x86_64.exe` dans ce dossier
3. Renommez-le en `ygege.exe`
4. Ajoutez le dossier au PATH (optionnel)

## Configuration

### Créer le fichier de configuration

Créez un fichier `config.json` dans le même dossier que le binaire :

```json
{
  "username": "votre_nom_utilisateur_ygg",
  "password": "votre_mot_de_passe",
  "bind_ip": "0.0.0.0",
  "bind_port": 8715,
  "log_level": "info",
  "tmdb_token": null
}
```

:::danger Identifiants obligatoires
YGG Torrent est un tracker privé. Des identifiants valides sont **absolument obligatoires**.
:::

### Configuration via variables d'environnement

Vous pouvez aussi utiliser des variables d'environnement :

```bash
export YGG_USERNAME="votre_nom_utilisateur"
export YGG_PASSWORD="votre_mot_de_passe"
export BIND_PORT="8715"
export LOG_LEVEL="info"
```

## Lancement

### Lancement simple

```bash
# Linux/macOS
./ygege

# Windows (PowerShell)
.\ygege.exe
```

Le serveur démarre sur `http://localhost:8715`

### Lancement en arrière-plan (Linux/macOS)

```bash
# Avec nohup
nohup ./ygege > ygege.log 2>&1 &

# Avec screen
screen -S ygege
./ygege
# Ctrl+A puis D pour détacher
```

### Service systemd (Linux)

Créez `/etc/systemd/system/ygege.service` :

```ini
[Unit]
Description=Ygégé - YGG Torrent Indexer
After=network.target

[Service]
Type=simple
User=votreuser
WorkingDirectory=/opt/ygege
ExecStart=/usr/local/bin/ygege
Restart=on-failure
RestartSec=5s

Environment="YGG_USERNAME=votre_username"
Environment="YGG_PASSWORD=votre_password"

[Install]
WantedBy=multi-user.target
```

Activez et démarrez le service :

```bash
sudo systemctl daemon-reload
sudo systemctl enable ygege
sudo systemctl start ygege
sudo systemctl status ygege
```

### Tâche planifiée Windows

1. Ouvrez le Planificateur de tâches
2. Créez une nouvelle tâche de base
3. Configurez :
   - **Déclencheur** : Au démarrage
   - **Action** : Démarrer un programme → `C:\Program Files\Ygege\ygege.exe`
   - **Conditions** : Décochez "Démarrer uniquement sur secteur"

## Mise à jour

### Méthode manuelle

1. Téléchargez le nouveau binaire depuis les releases
2. Arrêtez Ygégé (`systemctl stop ygege` ou `Ctrl+C`)
3. Remplacez l'ancien binaire
4. Redémarrez (`systemctl start ygege` ou relancez)

### Script de mise à jour (Linux)

```bash
#!/bin/bash
LATEST=$(curl -s https://api.github.com/repos/UwUDev/ygege/releases/latest | grep tag_name | cut -d '"' -f 4)
PLATFORM="linux-x86_64" # Changez selon votre plateforme

echo "Téléchargement de Ygégé $LATEST..."
wget -O ygege.new "https://github.com/UwUDev/ygege/releases/download/$LATEST/ygege-$PLATFORM"

chmod +x ygege.new
sudo systemctl stop ygege
sudo mv ygege.new /usr/local/bin/ygege
sudo systemctl start ygege

echo "Mise à jour terminée vers $LATEST"
```

## Vérification

Testez que le service fonctionne :

```bash
curl http://localhost:8715/health
```

Réponse attendue :
```json
{
  "status": "ok",
  "version": "x.x.x"
}
```

## Dépannage

### "Permission denied" (Linux/macOS)

```bash
chmod +x ygege
```

### "Port déjà utilisé"

Changez le port dans `config.json` ou via la variable `BIND_PORT`.

### Logs en mode debug

```bash
export LOG_LEVEL="debug"
./ygege
```

### Le binaire ne démarre pas sur architectures anciennes

Utilisez la version `noupx` disponible dans les assets des releases (sans compression UPX).

## Compilation depuis les sources

Si aucun binaire précompilé ne correspond à votre plateforme, consultez le [guide de compilation](https://github.com/UwUDev/ygege#building-from-source).

## Prochaines étapes

Une fois Ygégé installé et fonctionnel :

1. [Configurez les options avancées](../configuration)
2. [Intégrez avec Prowlarr](../integrations/prowlarr) ou [Jackett](../integrations/jackett)
3. [Explorez l'API](../api)
