# Build stage
FROM rust:1.86-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    cmake \
    perl \
    pkg-config \
    libclang-dev \
    git \
    wget \
    && rm -rf /var/lib/apt/lists/*

# Télécharger et installer UPX
RUN wget https://github.com/upx/upx/releases/download/v5.0.0/upx-5.0.0-amd64_linux.tar.xz \
    && tar -xf upx-5.0.0-amd64_linux.tar.xz \
    && cp upx-5.0.0-amd64_linux/upx /usr/local/bin/ \
    && rm -rf upx-5.0.0-amd64_linux*

WORKDIR /usr/src/app

# Copy project files
COPY . .

# Build the project
RUN cargo build --release

# Compress the binary with UPX
RUN upx --best --lzma /usr/src/app/target/release/ygege

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && apt-get clean autoclean --yes \
    && apt-get autoremove --yes \
    && rm -rf /var/cache/apt/archives* /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary from the build stage
COPY --from=builder /usr/src/app/target/release/ygege /app/

# Creation of necessary directories
RUN mkdir -p /app/sessions
VOLUME ["/app/sessions"]

LABEL "org.opencontainers.image.source"="https://github.com/uwudev/ygege"
LABEL "org.opencontainers.image.title"="Ygégé"
LABEL "org.opencontainers.image.description"="High-performance indexer for YGG Torrent written in Rust"
LABEL "org.opencontainers.image.documentation"="https://github.com/uwudev/ygege/wiki"

ENTRYPOINT ["/app/ygege"]
