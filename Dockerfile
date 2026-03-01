# =============================================================================
# Stage 1 — Builder
# =============================================================================
FROM rust:1.86-bookworm AS builder

# Dépendances système nécessaires pour compiler boring2 (BoringSSL) et wreq
RUN apt-get update && apt-get install -y --no-install-recommends \
    cmake \
    clang \
    libclang-dev \
    ninja-build \
    pkg-config \
    perl \
    nasm \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copier Cargo.toml / Cargo.lock en premier pour profiter du cache Docker
# Si vous n'avez que le dossier src/, adaptez selon votre structure réelle
COPY Cargo.toml Cargo.lock ./

# Pré-compiler les dépendances (cache layer)
RUN mkdir src && echo 'fn main() {}' > src/main.rs \
    && cargo build --release 2>/dev/null || true \
    && rm -rf src

# Copier le vrai code source
COPY src ./src

# Forcer la recompilation du binaire principal
RUN touch src/main.rs && cargo build --release

# =============================================================================
# Stage 2 — Image finale (minimale)
# =============================================================================
FROM debian:bookworm-slim

# Certificats TLS pour les requêtes HTTPS sortantes
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copier uniquement le binaire compilé
COPY --from=builder /app/target/release/ygege /app/ygege

# Copier les fichiers statiques (HTML)
COPY --from=builder /app/src/static /app/src/static

# Dossier pour les sessions persistantes
RUN mkdir -p /app/sessions

# Port par défaut de l'application
EXPOSE 8715

# Variables d'environnement configurables (voir config.json pour les autres)
ENV BIND_IP=0.0.0.0
ENV BIND_PORT=8715

# Lancement
CMD ["/app/ygege"]
