# Ygégé - Docker deployment

Ygégé is a high performance indexer for YGG Torrent written in Rust. This guide explains how to deploy the official
Docker image, configure the service and avoid YGG rate-limit by providing the correct credentials.

---

## Prérequis

- [Docker](https://docs.docker.com/get-docker/) installed
- [Docker Compose](https://docs.docker.com/compose/install/) installed
- A valid YGG Torrent account

---

## 1. Préparer le dossier de configuration

Create a folder `ygege` at the root of your project (or in the folder of your choice):

```bash
mkdir -p ygege
```

---

## 2. Create and fill the file `config.json`

In the `ygege` folder, create a `config.json` file with your YGG information:

```json
{
    "username": "your_ygg_username",
    "password": "your_ygg_password",
    "bind_ip": "0.0.0.0",
    "bind_port": 8715,
    "log_level": "debug"
}
```

> **Important:**
> - **Fill in the fields `username` and `password` with your YGG credentials correctly.
> - If the file is not present or incorrectly filled out, you may be **rate-limit** or blocked by YGG.

---

## 3. Sample `compose.yml` file

Place this file at the root of your project :

```yaml
services:
  ygege:
    image: uwucode/ygege:master
    container_name: ygege
    restart: unless-stopped
    volumes:
      - ./ygege/sessions:/app/sessions
      - ./ygege/config.json:/app/config.json
    ports:
      - 8715:8715
```

---

## Alternative: Environment variables

Instead of `config.json`, you can use environment variables:

```yaml
services:
  ygege:
    image: uwucode/ygege:master
    container_name: ygege
    restart: unless-stopped
    environment:
      - YGG_USERNAME=your_ygg_username
      - YGG_PASSWORD=your_ygg_password
      - BIND_IP=0.0.0.0
      - BIND_PORT=8715
      - LOG_LEVEL=debug
    volumes:
      - ./ygege/sessions:/app/sessions
    ports:
      - 8715:8715
```

> **Important:**
> - **Fill in the fields** `username` and `password` with your YGG credentials correctly.
> - If the environment variables are missing or misconfigured, you may be **rate-limited** or blocked by YGG.

---

## 4. Launch the service

In the folder where you have your `compose.yml`:

```bash
docker compose up -d
```

The service will then be available on your machine's `8715` port.

---

## 5. Verify the functioning

- View the container logs:

```bash
docker logs -f ygege
```

- The API or interface should be accessible to [http://localhost:8715](http://localhost:8715)

---

## 6. Important notes

- **Never share your `config.json` file with your YGG credentials.
- The `sessions` folder must remain persistent to avoid having to reconnect each time you restart.
- **In case of wrong identifiers or wrong settings, YGG may block or limit you.**

---

## Useful links

- [Official documentation](https://github.com/uwudev/ygege/wiki)
- [Report a bug](https://github.com/uwudev/ygege/issues)

---
