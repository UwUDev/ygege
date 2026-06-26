FROM mcr.microsoft.com/devcontainers/rust:2-1

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    cmake \
    perl \
    pkg-config \
    libclang-dev \
    git \
    wget \
    && rm -rf /var/lib/apt/lists/*
