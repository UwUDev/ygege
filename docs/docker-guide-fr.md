# Ygégé – Déploiement Docker

Ygégé est un indexeur haute performance pour YGG Torrent écrit en Rust. Ce guide explique comment déployer l’image Docker officielle, configurer le service et éviter le rate-limit de YGG en fournissant les bons identifiants.

---

## Prérequis

- [Docker](https://docs.docker.com/get-docker/) installé
- [Docker Compose](https://docs.docker.com/compose/install/) installé
- Un compte YGG Torrent valide

---

## 1. Préparer le dossier de configuration

Créez un dossier `ygege` à la racine de votre projet (ou dans le dossier de votre choix) :

```bash
mkdir -p ygege
```

---

## 2. Créer et remplir le fichier `config.json`

Dans le dossier `ygege`, créez un fichier `config.json` avec vos informations YGG :

```json
{
    "username": "your_ygg_username",
    "password": "your_ygg_password",
    "bind_ip": "0.0.0.0",
    "bind_port": 8080,
    "log_level": "debug"
}
```

> **Important :**
> - **Remplissez correctement** les champs `username` et `password` avec vos identifiants YGG.
> - Si le fichier n’est pas présent ou mal rempli, vous risquez d’être **rate-limit** ou bloqué par YGG.

---

## 3. Exemple de fichier `compose.yml`

Placez ce fichier à la racine de votre projet :

```yaml
services:
  ygege:
    image: uwucode/ygege:latest
    container_name: ygege
    restart: unless-stopped 
    volumes:
      - ./ygege/sessions:/app/sessions
      - ./ygege/config.json:/app/config.json
    ports:
      - 8080:8080
```

---

## 4. Lancer le service

Dans le dossier où se trouve votre `compose.yml` :

```bash
docker compose up -d
```

Le service sera alors accessible sur le port `8080` de votre machine.

---

## 5. Vérifier le fonctionnement

- Consultez les logs du container :

```bash
docker logs -f ygege
```

- L’API ou l’interface devrait être accessible à [http://localhost:8080](http://localhost:8080)

---

## 6. Notes importantes

- **Ne partagez jamais** votre fichier `config.json` avec vos identifiants YGG.
- Le dossier `sessions` doit rester persistant pour éviter de devoir se reconnecter à chaque redémarrage.
- **En cas de mauvais identifiants ou de mauvais paramétrage, YGG peut vous bloquer ou vous limiter.**

---

## Liens utiles

- [Documentation officielle](https://github.com/uwudev/ygege/wiki)
- [Signaler un bug](https://github.com/uwudev/ygege/issues)

---

**Bon indexage avec Ygégé !** 🚀