# v0.7.1-fixed - Fix Connection Timeout to YGG Torrent

## 🔴 Critical Fix

This release fixes a **critical connection timeout issue** that prevented Ygégé from connecting to YGG Torrent.

### Problem
- ❌ Connection timeout: `wreq::Error { kind: Request, ... Connection timed out }`
- ❌ Failed authentication
- ❌ Unable to search torrents

### Root Cause
The authentication mechanism used a "leaked IP" (`89.42.231.91`) from Pastebin that is **no longer valid**. The forced DNS resolution to this obsolete IP caused TCP connection timeouts.

**Real YGG IPs**: `104.26.5.166`, `104.26.4.166`, `172.67.70.199`

### Solution
- ✅ Disabled forced DNS resolution to outdated leaked IP
- ✅ Enabled normal Cloudflare DNS resolution
- ✅ Successful authentication and search

---

## 📦 Installation

### Docker (Recommended)

**Using pre-built image from this release:**

```bash
# Pull the image from GitHub Container Registry (if published)
docker pull ghcr.io/ist3rik/ygege:v0.7.1-fixed

# Or build from source:
git clone https://github.com/IsT3RiK/ygege.git
cd ygege
git checkout v0.7.1-fixed
docker build -t ygege-fixed:latest -f docker/Dockerfile .
```

**Run the container:**

```bash
docker run -d \
  --name ygege \
  -p 8715:8715 \
  -v ./sessions:/app/sessions \
  -e YGG_USERNAME="your_username" \
  -e YGG_PASSWORD="your_password" \
  -e YGG_DOMAIN="yggtorrent.org" \
  ygege-fixed:latest
```

**Or use Docker Compose:**

```bash
git clone https://github.com/IsT3RiK/ygege.git
cd ygege
git checkout v0.7.1-fixed

# Edit docker-compose.test.yml with your credentials
nano docker-compose.test.yml

# Start
docker compose -f docker-compose.test.yml up -d
```

### From Source

```bash
# Clone and checkout
git clone https://github.com/IsT3RiK/ygege.git
cd ygege
git checkout v0.7.1-fixed

# Build
cargo build --release

# Run
./target/release/ygege
```

---

## 🔧 Changes

### Modified Files
- **`src/auth.rs`**: Commented out outdated leaked IP mechanism
  - Removed `get_leaked_ip()` call
  - Removed forced DNS resolve to `89.42.231.91`
  - Allow normal Cloudflare DNS resolution

### Added Files
- **`FIX_LEAKED_IP_ISSUE.md`**: Detailed technical analysis
- **`build-and-test.sh`**: Automated build and test script
- **`docker-compose.test.yml`**: Docker Compose configuration for testing

---

## 🧪 Testing

After deploying this version, you should see:

```
✅ INFO  ygege > Ygégé v0.7.1 (commit: 70c6e12...)
✅ INFO  ygege > Detected own IP address: xx.xx.xx...
✅ INFO  ygege > Using configured YGG domain: yggtorrent.org
✅ DEBUG ygege::auth > Logging in with username: your_username
✅ DEBUG ygege::auth > Logged in successfully in 1.2s
✅ INFO  ygege > Logged in to YGG with username: your_username
```

**Test endpoints:**
```bash
# Health check
curl http://localhost:8715/health

# Status
curl http://localhost:8715/status | jq

# Search
curl "http://localhost:8715/search?name=debian" | jq
```

---

## ⚠️ Important Notes

- This is a **fork-specific fix** for the connection timeout issue
- A PR will be submitted to the upstream repository: [UwUDev/ygege](https://github.com/UwUDev/ygege)
- This release is marked as **pre-release** until merged upstream

---

## 🔗 Links

- **Full Changelog**: [v0.7.1...v0.7.1-fixed](https://github.com/IsT3RiK/ygege/compare/v0.7.1...v0.7.1-fixed)
- **Issue Report**: See `FIX_LEAKED_IP_ISSUE.md` for detailed analysis
- **Upstream Project**: [UwUDev/ygege](https://github.com/UwUDev/ygege)

---

## 👤 Credits

- **Original Project**: [UwUDev/ygege](https://github.com/UwUDev/ygege)
- **Fix Author**: [@IsT3RiK](https://github.com/IsT3RiK)
- **Issue Reporter**: IsT3RiK

---

## 📊 Technical Details

**Base Version**: v0.7.1
**Fix Commits**:
- `70c6e12` - Fix: Disable outdated leaked IP forcing DNS resolution
- `5817446` - Add Docker build utilities for testing the fix

**Compilation tested**: ✅ `cargo check` passes
**Docker build tested**: ✅ Multi-arch build successful
**Runtime tested**: ✅ Authentication and search working
