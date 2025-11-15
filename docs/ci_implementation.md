# Ygégé CI/CD Pipeline Guide

> **Quick Start:** This document explains how Ygégé is automatically built, tested, and distributed. Use this as a reference for troubleshooting build issues or understanding the release process.

## 📋 Table of Contents
- [What Gets Built Automatically](#what-gets-built-automatically)
- [How to Get Ygégé](#how-to-get-ygégé)
- [Branch Strategy](#branch-strategy)
- [Version Information](#version-information)
- [Troubleshooting](#troubleshooting)
- [For Contributors](#for-contributors)

---

## What Gets Built Automatically

Every time code is pushed to GitHub, our automated system builds Ygégé for multiple platforms:

### 📦 Binary Downloads (16 variants)

| Platform | Architectures | Types |
|----------|---------------|-------|
| **Linux (glibc)** | x86_64, i686, aarch64, armv7 | Normal + UPX compressed |
| **Windows** | x86_64, i686 | Normal + UPX compressed |
| **macOS** | Intel (x86_64), Apple Silicon (aarch64) | Normal + UPX compressed |

**What's UPX?** Compressed versions are 50-70% smaller but take slightly longer to start. Use normal version if unsure.

### 🐳 Docker Images (2 platforms)

| Architecture | Devices |
|--------------|---------|
| **linux/amd64** | Most PCs, servers, cloud instances |
| **linux/arm64** | Raspberry Pi 4+, Apple M1/M2/M3, AWS Graviton |

---

## How to Get Ygégé

### Option 1: Docker (Recommended for Servers) 🐳

```bash
# Latest stable release
docker pull uwucode/ygege:latest

# Development version (latest features, may be unstable)
docker pull uwucode/ygege:develop

# Beta testing version
docker pull uwucode/ygege:beta

# Specific version
docker pull uwucode/ygege:0.4.2
```

**Alternative registry (GitHub):**
```bash
docker pull ghcr.io/uwudev/ygege:latest
```

**Run with Docker:**
```bash
docker run -d \
  -p 8080:8080 \
  -v ./sessions:/app/sessions \
  uwucode/ygege:latest
```

### Option 2: Download Binaries 💾

1. Go to [GitHub Actions](https://github.com/UwUDev/ygege/actions)
2. Click on the latest successful workflow run
3. Scroll to **"Artifacts"** section
4. Download the file matching your system:
   - **Linux (most systems):** `ygege-linux-gnu-x86_64.zip`
   - **Linux (ARM/Raspberry Pi):** `ygege-linux-gnu-aarch64.zip` or `ygege-linux-gnu-armv7.zip`
   - **Windows:** `ygege-windows-x86_64.zip`
   - **macOS (Intel):** `ygege-macos-x86_64.zip`
   - **macOS (Apple Silicon):** `ygege-macos-aarch64.zip`

**Want smaller files?** Look for `-upx` versions (e.g., `ygege-linux-gnu-x86_64-upx.zip`)

**Note:** Artifacts are kept for 7 days. For permanent releases, use Docker images or wait for a tagged release.

---

## Branch Strategy

Understanding which version you're using:

| Branch | Purpose | When to Use | Docker Tag |
|--------|---------|-------------|------------|
| **master** | Stable releases | Production use | `latest`, `stable`, `0.4.2` |
| **beta** | Release candidates | Pre-release testing | `beta` |
| **develop** | Latest development | Testing new features | `develop` |

### Which Should I Use?

- 🟢 **master**: For production servers (most stable)
- 🟡 **beta**: To test upcoming releases before they go live
- 🔴 **develop**: For developers and early testers (may have bugs)

---

## Version Information

### Check Your Version

Every Ygégé binary includes build information:

```bash
# Show version details
./ygege --version
```

**Output:**
```
Ygégé v0.4.2
Commit: a1b2c3d4e5f6
Build Date: 2025-11-11T10:30:00Z
Branch: develop
```

**What this tells you:**
- **Version**: The release number
- **Commit**: Exact code snapshot (useful for bug reports)
- **Build Date**: When this binary was compiled
- **Branch**: Which version branch you're using

### Version in Logs

When you start Ygégé, it automatically logs version info:

```
INFO Ygégé v0.4.2 (commit: a1b2c3d, branch: develop, built: 2025-11-11T10:30:00Z)
INFO Logged in to YGG with username: youruser
```

---

## Troubleshooting

### ❓ "I can't find the binary for my system"

**Solution:** Check the [Artifacts section](#option-2-download-binaries-💾) in GitHub Actions. We build for:
- Linux: x86_64 (most PCs), i686 (32-bit), aarch64 (64-bit ARM), armv7 (32-bit ARM)
- Windows: x86_64 (64-bit), i686 (32-bit)
- macOS: x86_64 (Intel), aarch64 (Apple Silicon)

Not listed? Open an [issue](https://github.com/UwUDev/ygege/issues) requesting your platform.

### ❓ "Docker pull fails or image not found"

**Check:**
1. Spelling: `uwucode/ygege` (not `uwudev`)
2. Tag exists: `develop`, `beta`, `latest`, `stable`, or version number
3. Try alternative registry: `ghcr.io/uwudev/ygege:latest`

**Example error:**
```
Error: manifest for uwucode/ygege:wrong-tag not found
```
**Fix:** Use valid tag like `develop` or `latest`

### ❓ "Binary won't run / Permission denied"

**Linux/macOS:**
```bash
chmod +x ygege
./ygege
```

**Windows:** Right-click → Properties → Unblock → Apply

### ❓ "UPX version crashes on startup"

**Solution:** Use the normal (non-UPX) version. Some antivirus software flags compressed executables.

### ❓ "How do I report a build issue?"

When reporting issues, include:
1. **Your system:** OS, architecture (run `uname -m` on Linux/macOS)
2. **Version info:** Output of `./ygege --version`
3. **How you got it:** Docker tag or artifact name
4. **Error message:** Full error output

**Template:**
```
System: Ubuntu 22.04 x86_64
Version: Ygégé v0.4.2 (commit: a1b2c3d, branch: develop)
Source: Docker uwucode/ygege:develop
Error: [paste error here]
```

### ❓ "Artifacts expired (404 error)"

Artifacts are kept for 7 days. Options:
- Use Docker images (permanent)
- Build from source
- Wait for next commit to trigger new builds

---

## For Contributors

### Setting Up CI/CD

**Required GitHub Secrets:**

Go to: `Settings` → `Secrets and variables` → `Actions` → `New repository secret`

| Secret Name | Description | How to Get |
|-------------|-------------|------------|
| `DOCKERHUB_USERNAME` | Your Docker Hub username | Your Docker Hub account name |
| `DOCKERHUB_TOKEN` | Docker Hub access token | [Create token](https://hub.docker.com/settings/security) |

**Note:** `GITHUB_TOKEN` is automatic, no setup needed.

### When Does CI Run?

| Event | What Happens |
|-------|--------------|
| **Push to develop/beta/master** | Full build + Docker publish + artifacts |
| **Pull Request** | Tests + build verification only (no publish) |
| **Manual trigger** | Via Actions tab → "Run workflow" |

### Build Times

Approximate durations:
- **Tests only:** ~5 minutes
- **All binaries (16):** ~30-45 minutes
- **Docker images:** ~15-20 minutes
- **Total (on develop/beta/master):** ~60-80 minutes

### Modifying the CI

**Files to know:**
- `.github/workflows/ci.yml` - Main CI configuration
- `docker/Dockerfile` - Docker image build instructions
- `src/main.rs` - Version display logic

**Before changing:**
1. Test locally if possible
2. Use a feature branch
3. Check CI passes on your PR before merging

### Common CI Tasks

**Add a new architecture:**
Edit `.github/workflows/ci.yml` → Add to matrix under relevant job

**Change Docker tags:**
Edit `.github/workflows/ci.yml` → `docker` job → `Determine Docker tags` step

**Update Rust version:**
Edit `docker/Dockerfile` → Change `FROM rust:1.91-slim-trixie`

---

## Technical Details

### Build Optimizations

All binaries use these Cargo profile settings:
```toml
[profile.release]
opt-level = "z"      # Optimize for size
lto = true           # Link-time optimization
codegen-units = 1    # Better optimization
strip = true         # Remove debug symbols
```

### Docker Image Labels

Images include OpenContainer metadata:
- `org.opencontainers.image.source` - GitHub repository URL
- `org.opencontainers.image.revision` - Git commit SHA
- `org.opencontainers.image.created` - Build timestamp

View with: `docker inspect uwucode/ygege:latest`

### Build Environment Variables

These are embedded during compilation:
- `BUILD_COMMIT` - Git commit SHA
- `BUILD_DATE` - ISO 8601 timestamp
- `BUILD_BRANCH` - Branch name (develop/beta/master)

---

## Support

**Found a bug?** [Open an issue](https://github.com/UwUDev/ygege/issues)

**Need help?** Check existing issues or start a discussion

**Want to contribute?** Read [contribution guidelines](../contribution.md)
