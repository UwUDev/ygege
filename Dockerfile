FROM --platform=$BUILDPLATFORM rust:1.89-slim-trixie AS builder

# Arguments for cross-compilation
ARG TARGETPLATFORM
ARG TARGETARCH
ARG TARGETVARIANT

# Install build dependencies for cross-compilation
RUN dpkg --add-architecture arm64 && \
    apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    cmake \
    perl \
    pkg-config \
    libclang-dev \
    git \
    wget \
    gcc-aarch64-linux-gnu \
    g++-aarch64-linux-gnu \
    libssl-dev:arm64 \
    && rm -rf /var/lib/apt/lists/*

# Rust toolchain configuration
RUN case "${TARGETARCH}" in \
    "arm64") \
        rustup target add aarch64-unknown-linux-gnu \
        ;; \
    "amd64") \
        rustup target add x86_64-unknown-linux-gnu \
        ;; \
    esac

# Installation of UPX adapted to the target architecture
RUN case "${TARGETARCH}" in \
    "arm64") \
        wget https://github.com/upx/upx/releases/download/v5.0.0/upx-5.0.0-arm64_linux.tar.xz \
        && tar -xf upx-5.0.0-arm64_linux.tar.xz \
        && cp upx-5.0.0-arm64_linux/upx /usr/local/bin/ \
        ;; \
    "amd64") \
        wget https://github.com/upx/upx/releases/download/v5.0.0/upx-5.0.0-amd64_linux.tar.xz \
        && tar -xf upx-5.0.0-amd64_linux.tar.xz \
        && cp upx-5.0.0-amd64_linux/upx /usr/local/bin/ \
        ;; \
    esac \
    && rm -rf upx-5.0.0-*

WORKDIR /usr/src/app

# Copy project files
COPY . .

# Build the project with cross-compilation
RUN case "${TARGETARCH}" in \
    "arm64") \
        CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
        cargo build --release --target=aarch64-unknown-linux-gnu \
        ;; \
    "amd64") \
        cargo build --release --target=x86_64-unknown-linux-gnu \
        ;; \
    esac

# Compress the binary with UPX
RUN case "${TARGETARCH}" in \
    "arm64") \
        upx --best --lzma /usr/src/app/target/aarch64-unknown-linux-gnu/release/ygege \
        ;; \
    "amd64") \
        upx --best --lzma /usr/src/app/target/x86_64-unknown-linux-gnu/release/ygege \
        ;; \
    esac

# Runtime stage (multi-arch)
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && apt-get clean autoclean --yes \
    && apt-get autoremove --yes \
    && rm -rf /var/cache/apt/archives* /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary from the build stage
COPY --from=builder /usr/src/app/target/*-unknown-linux-gnu/release/ygege /app/

# Create necessary directories
RUN mkdir -p /app/sessions
VOLUME ["/app/sessions"]

# Metadata
LABEL "org.opencontainers.image.source"="https://github.com/uwudev/ygege"
LABEL "org.opencontainers.image.title"="Ygégé"
LABEL "org.opencontainers.image.description"="High-performance indexer for YGG Torrent written in Rust"
LABEL "org.opencontainers.image.documentation"="https://github.com/uwudev/ygege/wiki"

ENTRYPOINT ["/app/ygege"]
