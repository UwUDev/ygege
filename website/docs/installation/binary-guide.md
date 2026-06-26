---
sidebar_position: 2
---

# Installation avec binaires précompilés

Ce guide explique comment installer et utiliser Francisca avec les binaires précompilés fournis à chaque release.

## Prérequis

- Système d'exploitation supporté : Linux, Windows, macOS
- Aucune dépendance externe requise (binaires statiques)

## Téléchargement

### Option 1 : Depuis GitHub Releases (Recommandé)

1. Rendez-vous sur la [page des releases](https://github.com/UwUDev/francisca/releases)
2. Téléchargez le binaire correspondant à votre plateforme :
   - **Linux AMD64** : `francisca-linux-x86_64`
   - **Linux ARM64** : `francisca-linux-aarch64`
   - **Linux ARMv7** : `francisca-linux-armv7`
   - **Windows AMD64** : `francisca-windows-x86_64.exe`
   - **macOS Intel** : `francisca-macos-x86_64`
   - **macOS Apple Silicon** : `francisca-macos-aarch64`

### Option 2 : Via wget/curl (Linux/macOS)

```bash
# Remplacez VERSION par la version souhaitée (ex: v1.0.0)
# Remplacez PLATFORM par votre plateforme (ex: linux-x86_64)
wget https://github.com/UwUDev/francisca/releases/download/VERSION/francisca-PLATFORM

# Ou avec curl
curl -L -o francisca https://github.com/UwUDev/francisca/releases/download/VERSION/francisca-PLATFORM
```

## Installation

### Linux / macOS

```bash
# Rendre le binaire exécutable
chmod +x francisca-*

# Déplacer dans un dossier du PATH (optionnel)
sudo mv francisca-* /usr/local/bin/francisca

# Vérifier l'installation
francisca --version
```

### Windows

1. Créez un dossier `C:\Program Files\Francisca\`
2. Déplacez `francisca-windows-x86_64.exe` dans ce dossier
3. Renommez-le en `francisca.exe`
4. Ajoutez le dossier au PATH (optionnel)

## Configuration

### Créer le fichier de configuration

Créez un fichier `config.json` dans le même dossier que le binaire :

```json
{
  "bind_ip": "0.0.0.0",
  "bind_port": 8715,
  "log_level": "info",
  "tmdb_token": null,
  "use_tor": false,
  "tor_proxy": "127.0.0.1:9050"
}
```

:::info Aucune authentification requise
Le relais Nostr peut être public. Selon le relais, aucun identifiant n'est nécessaire.
:::

### Configuration via variables d'environnement

Vous pouvez aussi utiliser des variables d'environnement :

```bash
export BIND_PORT="8715"
export LOG_LEVEL="info"
# export TMDB_TOKEN="votre_token"  # Optionnel
```

## Lancement

### Lancement simple

```bash
# Linux/macOS
./francisca

# Windows (PowerShell)
.\francisca.exe
```

Le serveur démarre sur `http://localhost:8715`

### Lancement en arrière-plan (Linux/macOS)

```bash
# Avec nohup
nohup ./francisca > francisca.log 2>&1 &

# Avec screen
screen -S francisca
./francisca
# Ctrl+A puis D pour détacher
```

### Service systemd (Linux)

Créez `/etc/systemd/system/francisca.service` :

```ini
[Unit]
Description=Francisca - U2P System Indexer
After=network.target

[Service]
Type=simple
User=votreuser
WorkingDirectory=/opt/francisca
ExecStart=/usr/local/bin/francisca
Restart=on-failure
RestartSec=5s

Environment="LOG_LEVEL=info"

[Install]
WantedBy=multi-user.target
```

Activez et démarrez le service :

```bash
sudo systemctl daemon-reload
sudo systemctl enable francisca
sudo systemctl start francisca
sudo systemctl status francisca
```

### Tâche planifiée Windows

1. Ouvrez le Planificateur de tâches
2. Créez une nouvelle tâche de base
3. Configurez :
   - **Déclencheur** : Au démarrage
   - **Action** : Démarrer un programme → `C:\Program Files\Francisca\francisca.exe`
   - **Conditions** : Décochez "Démarrer uniquement sur secteur"

## Mise à jour

### Méthode manuelle

1. Téléchargez le nouveau binaire depuis les releases
2. Arrêtez Francisca (`systemctl stop francisca` ou `Ctrl+C`)
3. Remplacez l'ancien binaire
4. Redémarrez (`systemctl start francisca` ou relancez)

### Script de mise à jour (Linux)

```bash
#!/bin/bash
LATEST=$(curl -s https://api.github.com/repos/UwUDev/francisca/releases/latest | grep tag_name | cut -d '"' -f 4)
PLATFORM="linux-x86_64" # Changez selon votre plateforme

echo "Téléchargement de Francisca $LATEST..."
wget -O francisca.new "https://github.com/UwUDev/francisca/releases/download/$LATEST/francisca-$PLATFORM"

chmod +x francisca.new
sudo systemctl stop francisca
sudo mv francisca.new /usr/local/bin/francisca
sudo systemctl start francisca

echo "Mise à jour terminée vers $LATEST"
```

## Vérification

Testez que le service fonctionne :

```bash
curl http://localhost:8715/health
```

Réponse attendue :
```
OK
```

Pour un statut détaillé :
```bash
curl http://localhost:8715/status
```

Réponse :
```json
{
  "relay": "wss://relay.example.org",
  "search": "ok",
  "parsing": "ok",
  "tmdb_integration": "disabled"
}
```

## Dépannage

### "Permission denied" (Linux/macOS)

```bash
chmod +x francisca
```

### "Port déjà utilisé"

Changez le port dans `config.json` ou via la variable `BIND_PORT`.

### Logs en mode debug

```bash
export LOG_LEVEL="debug"
./francisca
```

### Le binaire ne démarre pas sur architectures anciennes

Utilisez la version `noupx` disponible dans les assets des releases (sans compression UPX).

## Compilation depuis les sources

Si aucun binaire précompilé ne correspond à votre plateforme, consultez le [guide de compilation](https://github.com/UwUDev/francisca#building-from-source).

## Prochaines étapes

Une fois Francisca installé et fonctionnel :

1. [Configurez les options avancées](../configuration)
2. [Intégrez avec Prowlarr](../integrations/prowlarr) ou [Jackett](../integrations/jackett)
3. [Explorez l'API](../api)
