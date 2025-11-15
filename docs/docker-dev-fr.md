### Build from Source

Pour créer et exécuter votre propre image Docker :

1. Cloné le dépôt :

```bash
git clone https://github.com/UwUDev/ygege.git
cd ygege
```

2. Créer l'image Docker :

```bash
docker build -t ygege-custom .
```

**Pour Synology NAS ou systèmes anciens** (si vous rencontrez des segfaults avec les binaires compressés UPX) :

```bash
docker build -t ygege-custom --build-arg SKIP_UPX=1 .
```

3. Créer un dossier et un fichier de configuration :

```

mkdir -p docker-config
nano docker-config/config.json

```

4. Exécuter le conteneur avec une configuration personnalisée :

```

docker run -d \
  -p 8715:8715 \
  -v "${PWD}/docker-config/config.json:/app/config.json" \
  --name ygege-instance \
  ygege-custom

```

#### Exemple de configuration de fichier Dockerfile

```


# Build stage

FROM rust:1.86-slim-bookworm AS builder

# Install build dependencies

RUN apt-get update \&\& apt-get install -y --no-install-recommends \
build-essential cmake perl pkg-config libclang-dev git wget \
\&\& rm -rf /var/lib/apt/lists/*

# Install UPX

RUN wget https://github.com/upx/upx/releases/download/v5.0.0/upx-5.0.0-amd64_linux.tar.xz \
\&\& tar -xf upx-5.0.0-amd64_linux.tar.xz \
\&\& cp upx-5.0.0-amd64_linux/upx /usr/local/bin/ \
\&\& rm -rf upx-5.0.0-amd64_linux*

# Build and compress binary

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release \&\& \
upx --best --lzma target/release/ygege

# Runtime stage

FROM debian:bookworm-slim
RUN apt-get update \&\& apt-get install -y ca-certificates \
\&\& apt-get clean autoclean --yes \
\&\& apt-get autoremove --yes \
\&\& rm -rf /var/cache/apt/archives* /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/ygege /app/
RUN mkdir -p /app/sessions

ENTRYPOINT ["/app/ygege"]

```

**Important**: Assurez-vous que votre `config.json` contient des identifiants YGG valides pour éviter la limitation du
débit. Le fichier doit inclure:

```

{
"username": "your_actual_username",
"password": "your_actual_password",
"bind_ip": "0.0.0.0",
"bind_port": 8715,
"log_level": "debug"
}

```

Pour les déploiements de production, envisagez d'utiliser Docker Compose :

```

version: '3.8'

services:
ygege:
image: ygege-custom
restart: unless-stopped
ports:
- "8715:8715"
volumes:
- ./config.json:/app/config.json
- ygege-sessions:/app/sessions

volumes:
ygege-sessions:

```

**Avis de sécurité** : Ne jamais valider vos identifiants réels dans config.json. Utiliser des variables
d'environnement pour les déploiements de production.