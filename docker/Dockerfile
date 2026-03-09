FROM rust:1.94-slim-trixie AS builder

# Build information arguments
ARG BUILD_COMMIT=unknown
ARG BUILD_DATE=unknown
ARG BUILD_BRANCH=unknown
ARG SKIP_UPX=0

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

# Installation of UPX adapted to the target architecture (skip if SKIP_UPX=1)
RUN if [ "${SKIP_UPX}" = "0" ]; then \
        ARCH=$(uname -m) && \
        case "${ARCH}" in \
            "aarch64") \
                wget -q https://github.com/upx/upx/releases/download/v5.0.0/upx-5.0.0-arm64_linux.tar.xz \
                && tar -xf upx-5.0.0-arm64_linux.tar.xz \
                && cp upx-5.0.0-arm64_linux/upx /usr/local/bin/ \
                && rm -rf upx-5.0.0-* \
                ;; \
            "x86_64") \
                wget -q https://github.com/upx/upx/releases/download/v5.0.0/upx-5.0.0-amd64_linux.tar.xz \
                && tar -xf upx-5.0.0-amd64_linux.tar.xz \
                && cp upx-5.0.0-amd64_linux/upx /usr/local/bin/ \
                && rm -rf upx-5.0.0-* \
                ;; \
        esac; \
    fi

WORKDIR /usr/src/app

# Copy project files
COPY . .

# Build the project
RUN BUILD_COMMIT=${BUILD_COMMIT} BUILD_DATE=${BUILD_DATE} BUILD_BRANCH=${BUILD_BRANCH} \
    cargo build --release

# Compress the binary with UPX (skip if SKIP_UPX=1)
RUN if [ "${SKIP_UPX}" = "0" ]; then \
        upx --best --lzma /usr/src/app/target/release/ygege; \
    fi

# Runtime stage (multi-arch)
FROM debian:trixie-slim

# Arguments for runtime stage
ARG TARGETARCH
ARG BUILD_COMMIT=unknown
ARG BUILD_DATE=unknown
ARG BUILD_BRANCH=unknown

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    adduser \
    ca-certificates \
    curl \
    && apt-get clean autoclean --yes \
    && apt-get autoremove --yes \
    && rm -rf /var/cache/apt/archives* /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary from the build stage
COPY --from=builder /usr/src/app/target/release/ygege /app/ygege

RUN chmod +x /app/ygege

# Create necessary directories
RUN mkdir -p /app/sessions

# Create non-root user with specific UID/GID for better compatibility
# Using UID/GID 10001 to avoid conflicts with host systems
# --no-log-init: Prevents large UID values from causing issues
# --system: Creates a system user without password
# --shell /sbin/nologin: Prevents login shell access for security
RUN addgroup --gid 10001 --system ygege \
    && adduser --uid 10001 --system --ingroup ygege --home /home/ygege --shell /sbin/nologin ygege \
    && chown -R ygege:ygege /app /home/ygege

VOLUME ["/app/sessions"]

# Set HOME and switch to non-root user
ENV HOME=/home/ygege
USER ygege

ENTRYPOINT ["/app/ygege"]
