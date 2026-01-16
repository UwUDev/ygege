# Installation d'Ygégé sur seedbox sans droits sudo (compilation depuis votre ordinateur)

Guide complet pour installer Ygégé sur une seedbox où vous n'avez pas les droits sudo et sans passer par Docker (idéal pour swizzin), en compilant depuis votre Mac ou PC Windows. 


## Prérequis

- Un Mac ou PC Windows
- Docker Desktop installé
  - https://www.docker.com/products/docker-desktop/
- Accès SSH à votre seedbox
- Identifiants YGG Torrent valides

## Étape 1 : Compilation avec Docker

### Option A : Sur Mac (Intel ou Apple Silicon)

#### 1.1 Télécharger les sources


# Ouvrir Terminal sur votre Mac

```
mkdir ~/ygege-build
cd ~/ygege-build
```


# Télécharger les sources d'Ygégé
```
curl -L https://github.com/UwUDev/ygege/archive/refs/tags/v0.8.0.tar.gz -o v0.8.0.tar.gz
tar -xzf v0.8.0.tar.gz
cd ygege-0.8.0
```

#### 1.2 Compiler pour Linux x86_64

 **Important** : Votre Mac compile naturellement en ARM64, mais votre seedbox est en x86_64. Il faut forcer la plateforme.

```bash
# Nettoyer tout build précédent
rm -rf target/

# Compiler pour x86_64 Linux (forcer la plateforme)
docker run --platform linux/amd64 --rm -v $(pwd):/workspace -w /workspace rust:1.92-bullseye bash -c "
  apt-get update && 
  apt-get install -y cmake build-essential libclang-dev && 
  cargo build --release
"
```




#### 1.3 Vérifier le binaire

```bash
# Vérifier que c'est bien un binaire Linux x86_64
file target/release/ygege
```

Résultat attendu :
```
target/release/ygege: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked...
```

 Si vous voyez **"x86-64"**, c'est bon !
 Si vous voyez **"ARM aarch64"**, recommencez avec `--platform linux/amd64`

---

### Option B : Sur Windows (PowerShell) 

#### 1.1 Télécharger les sources

```powershell
# Ouvrir PowerShell
New-Item -Path "$env:USERPROFILE\ygege-build" -ItemType Directory -Force
cd "$env:USERPROFILE\ygege-build"

# Télécharger les sources d'Ygégé
Invoke-WebRequest -Uri "https://github.com/UwUDev/ygege/archive/refs/tags/v0.8.0.tar.gz" -OutFile "v0.8.0.tar.gz"

# Extraire (nécessite 7-Zip ou tar sous Windows 10+)
tar -xzf v0.8.0.tar.gz
cd ygege-0.8.0
```

**Si `tar` n'est pas disponible**, téléchargez manuellement :
1. Allez sur https://github.com/UwUDev/ygege/archive/refs/tags/v0.8.0.tar.gz
2. Extrayez avec 7-Zip ou WinRAR dans `C:\Users\VotreNom\ygege-build\`

#### 1.2 Compiler pour Linux x86_64

```powershell
# Nettoyer tout build précédent
Remove-Item -Path "target" -Recurse -Force -ErrorAction SilentlyContinue

# Compiler pour x86_64 Linux
docker run --platform linux/amd64 --rm -v ${PWD}:/workspace -w /workspace rust:1.92-bullseye bash -c "apt-get update && apt-get install -y cmake build-essential libclang-dev && cargo build --release"
```



#### 1.3 Vérifier le binaire

```powershell
# Le binaire est dans target\release\ygege
ls target\release\ygege
```

Le fichier doit faire environ **8-9 MB**.

---

## Étape 2 : Transfert sur la seedbox
 
### Préparer le dossier sur la seedbox

```bash
# D'abord, connectez-vous à votre seedbox et créez le dossier
mkdir -p ~/Ygege
```

### Option A : Depuis Mac

```bash
# Depuis votre Mac (remplacez par votre vraie adresse SSH)
scp target/release/ygege user@votre-seedbox.com:/home/user/Ygege/ygege
```

### Option B : Depuis Windows (PowerShell)

```powershell
# Avec scp (disponible sur Windows 10+)
scp target\release\ygege user@votre-seedbox.com:/home/user/Ygege/ygege
```

**Alternative si scp ne fonctionne pas** : Utilisez WinSCP, FileZilla ou votre client FTP/SFTP préféré pour transférer :
- **Fichier source** : `C:\Users\User\ygege-build\ygege-0.8.0\target\release\ygege`
- **Destination** : `/home/user/Ygege/ygege`

---

### 2.1 Vérifier sur la seedbox

> Connection en SSH

```bash
# SSH sur votre seedbox
ssh user@votre-seedbox.com

# Vérifier le fichier
ls -lh /home/user/Ygege/ygege
file /home/user/Ygege/ygege
```

Résultat attendu :
```
/home/user/Ygege/ygege: ELF 64-bit LSB pie executable, x86-64...
```

## Étape 3 : Configuration sur la seedbox

### 3.1 Créer le fichier de configuration

```bash
# Sur votre seedbox
nano /home/user/Ygege/config.json
```

Contenu (remplacez par vos vrais identifiants YGG et l'url de votre seedbox) :

```json
{
  "username": "votre_username_ygg",
  "password": "votre_password_ygg",
  "bind_ip": "url_de_seedbox",
  "bind_port": 8715,
  "log_level": "debug",
  "tmdb_token": null,
  "ygg_domain": null,
  "turbo_enabled": null
}
```


Sauvegardez : `Ctrl+O`, `Entrée`, `Ctrl+X`

### 3.2 Rendre le binaire exécutable

```bash
chmod +x /home/user/Ygege/ygege
```

### 3.3 Test manuel

```bash
cd /home/user/Ygege
./ygege
```

Vous devriez voir :
```
INFO  ygege > Ygégé v0.8.0
INFO  ygege > Detected own IP address: ...
INFO  ygege > Logged in to YGG with username: ...
INFO  ygege > Categories cache initialized: 9 categories, 47 sub-categories
```

### 3.4 Vérifier que ca fonctionne "Ygégé Ready"

Dans votre navigateur, ouvrez l'onglet : http://votre_url_seedbox:8715

Résultat attendu: 
> Ygégé is ready !
> 
> Wiki
> 
> Readme
> 
> 
> Infos
> 
> Domaine : www.yggtorrent.org
> Statut d'authentification : Authentificated
> DNS du domaine : Resolves
> Accessibilité du domaine : Reachable
> Analyse : OK
> Recherche : OK
> Intégration TMDB/IMDB : Disabled
> Informations utilisateur : OK



## Étape 4 : Service systemd (démarrage automatique)

### 4.1 Créer le service utilisateur

```bash
mkdir -p ~/.config/systemd/user
nano ~/.config/systemd/user/ygege.service
```

Contenu :

```ini
[Unit]
Description=Ygege YGG Indexer
After=network.target

[Service]
Type=simple
WorkingDirectory=/home/user/Ygege
ExecStart=/home/user/Ygege/ygege
Restart=always
RestartSec=10

[Install]
WantedBy=default.target
```



Sauvegardez : `Ctrl+O`, `Entrée`, `Ctrl+X`

### 4.2 Activer et démarrer le service

```bash
# Recharger systemd
systemctl --user daemon-reload

# Activer le service (démarrage automatique au boot)
systemctl --user enable ygege

# Démarrer le service
systemctl --user start ygege

# Vérifier le statut
systemctl --user status ygege
```

Résultat attendu :
```
● ygege.service - Ygege YGG Indexer
   Active: active (running)
   Memory: 11.9M
   CPU: 803ms
```

### 4.3 Vérifier les logs

```bash
# Logs en temps réel
journalctl --user -u ygege -f

# Dernières 50 lignes
journalctl --user -u ygege -n 50
```

### 4.4 Tester l'API

```bash
# Health check
curl http://votre_url_seedbox:8715/health

# Status détaillé
curl http://votre_url_seedbox:8715/status

# Test de recherche
curl "http://votre_url_seedbox:8715/search?q=test"
```

## Étape 5 : Intégration avec Prowlarr

### 5.1 Télécharger le fichier de définition

```bash
# Sur votre seedbox
cd ~
wget https://raw.githubusercontent.com/UwUDev/ygege/master/ygege.yml
```

### 5.2 Copier dans Prowlarr

Trouvez le dossier AppData de Prowlarr (affiché dans **Prowlarr → System → Status**).

```bash
# Créer le dossier Custom
  mkdir -p /home/user/.config/Prowlarr/Definitions/Custom/

# Copier le fichier
cp ~/ygege.yml /home/user/.config/Prowlarr/Definitions/Custom/
```

###  5.3 : Éditer le fichier ygege.yml



```
# éditer le fichier
nano /home/user/.config/Prowlarr/Definitions/Custom/ygege.yml
```

Ajoutez votre url dans la partie link au début du fichier:
  - http://votre_ip_seedbox:8715
  - http://votre_url_seedbox:8715

```
---
id: ygege
name: Ygégé
description: "YggTorrent (YGG) est un tracker torrent FRANÇAIS Privé pour FILMS / TV / GÉNÉRAL."
language: fr-FR
type: private
encoding: UTF-8
links:
  - http://localhost:8715/
  - http://ygege:8715/
  - http://votre_ip_seedbox:8715    # ← Remplacez par votre IP
  - http://votre_url_seedbox:8715   # ← Remplacez par votre URL
```

Sauvegardez : `Ctrl+O`, `Entrée`, `Ctrl+X`


### 5.4 Redémarrer Prowlarr

```bash
# Si Docker
docker restart prowlarr

# Si systemd
systemctl restart prowlarr
```

### 5.5 Ajouter l'indexeur dans Prowlarr

1. Allez dans **Settings → Indexers**
2. Cliquez sur **+ Add Indexer**
3. Recherchez "Ygégé" dans la liste
4. Configurez :
   - **Nom** : Ygégé
   - **Activer** : ✅
   - **Url de base** : 
     -  `http://url_votre_seedbox:8715/`

5. Cliquez sur **Test** (doit afficher un ✅)
6. Cliquez sur **Save**




## Commandes utiles

### Gestion du service

```bash
# Démarrer
systemctl --user start ygege

# Arrêter
systemctl --user stop ygege

# Redémarrer
systemctl --user restart ygege

# Statut
systemctl --user status ygege

# Logs en temps réel
journalctl --user -u ygege -f

# Désactiver le démarrage automatique
systemctl --user disable ygege
```
