### Build from Source

To build and run your own Docker image:

1. Clone repository:

```bash
git clone https://github.com/UwUDev/ygege.git
cd ygege
```

2. Build Docker image:

```bash
docker build -t ygege-custom .
```

**For Synology NAS or older systems** (if you experience segfaults with UPX-compressed binaries):

You can either build without UPX:

```bash
docker build -t ygege-custom --build-arg SKIP_UPX=1 .
```

Or use the official `-noupx` Docker image:

```bash
docker pull uwucode/ygege:noupx
# or
docker pull uwucode/ygege:latest-noupx
```

3. Create config folder and file:

```

mkdir -p docker-config
nano docker-config/config.json

```

4. Run container with custom configuration:

```

docker run -d \
-p 8715:8715 \
-v \$(pwd)/docker-config/config.json:/app/config.json \
-v ygege-sessions:/app/sessions \
--name ygege-instance \
ygege-custom

```

#### Example Dockerfile Configuration

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

**Important**: Ensure your `config.json` contains valid YGG credentials to avoid rate-limiting. File must include:

```

{
"username": "your_actual_username",
"password": "your_actual_password",
"bind_ip": "0.0.0.0",
"bind_port": 8715,
"log_level": "debug"
}

```

For production deployments, consider using Docker Compose:

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

**Security Note**: Never commit your real credentials in config.json. Use environment variables for production
deployments.