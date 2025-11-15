# Release Workflow Documentation

## Overview

The release workflow automatically builds, tests, and publishes Ygégé binaries and Docker images when you create a new version tag (e.g., `v1.0.0`).

## What Gets Built

### 📦 Binary Artifacts (16 total)

For each of the 8 target platforms, we create:
- **Regular binary**: Optimized but uncompressed
- **UPX binary**: Compressed with UPX for smaller size

#### Platforms

**Linux GNU** (4 targets):
- `x86_64-unknown-linux-gnu` - Intel/AMD 64-bit
- `aarch64-unknown-linux-gnu` - ARM 64-bit
- `armv7-unknown-linux-gnueabihf` - ARM 32-bit (Raspberry Pi, etc.)
- `i686-unknown-linux-gnu` - Intel/AMD 32-bit

**Windows** (2 targets):
- `x86_64-pc-windows-msvc` - Windows 64-bit
- `i686-pc-windows-msvc` - Windows 32-bit

**macOS** (2 targets):
- `x86_64-apple-darwin` - macOS Intel
- `aarch64-apple-darwin` - macOS Apple Silicon (M1/M2/M3)

### 🐳 Docker Images

Multi-architecture Docker images published to:
- **Docker Hub**: `uwucode/ygege`
- **GitHub Container Registry**: `ghcr.io/uwudev/ygege`

Supported architectures:
- `linux/amd64` - Intel/AMD 64-bit
- `linux/arm64` - ARM 64-bit

### 🔐 Security Features

- **Image signing** with Cosign (keyless signing)
- **SBOM generation** using Trivy (CycloneDX format)
- **SBOM attestation** attached to images
- **Signature verification** before release publication

## How to Create a Release

### 1. Prepare Your Code

Ensure all changes are merged to `master` branch:
```bash
git checkout master
git pull origin master
```

### 2. Create and Push a Tag

```bash
# Create a tag (follow semantic versioning)
git tag v1.0.0

# Push the tag to trigger the release workflow
git push origin v1.0.0
```

### 3. Monitor the Workflow

Go to: `https://github.com/UwUDev/ygege/actions`

The workflow will:
1. ✅ Generate changelog from commits
2. ✅ Create a draft release
3. ✅ Build all 16 binary artifacts in parallel
4. ✅ Upload binaries to the draft release
5. ✅ Build and publish Docker images
6. ✅ Sign images and create SBOMs
7. ✅ Upload SBOMs to the draft release
8. ✅ Verify signatures and attestations
9. ✅ Publish the release (make it public)
10. ✅ Send Discord notification (if webhook configured)

### 4. Review and Customize

After the workflow completes, you can:
- Edit the release notes on GitHub
- Add additional assets manually
- Update the description

## Tag Naming Conventions

### Stable Releases
```bash
v1.0.0        # Major.Minor.Patch
v2.1.3
```

### Pre-releases
```bash
v1.0.0-alpha.1   # Alpha release
v1.0.0-beta.2    # Beta release
v1.0.0-rc.1      # Release candidate
```

**Note**: Pre-releases won't get the `:latest` Docker tag, only version-specific tags.

## Docker Tags

For version `v1.2.3`:
- `uwucode/ygege:1.2.3` - Full version
- `uwucode/ygege:1.2` - Minor version
- `uwucode/ygege:1` - Major version
- `uwucode/ygege:stable` - Stable tag
- `uwucode/ygege:latest` - Latest stable (only if not a pre-release)

## Required Secrets

Configure these in your GitHub repository settings:

### Mandatory
- `DOCKERHUB_USERNAME` - Your Docker Hub username
- `DOCKERHUB_TOKEN` - Docker Hub access token (create at hub.docker.com)

### Optional
- `DISCORD_WEBHOOK` - Discord webhook URL for notifications

## Troubleshooting

### Build Fails on Specific Architecture

Check the job logs for that specific platform:
1. Go to Actions tab
2. Click on the failed workflow run
3. Click on the failed job (e.g., "Build Linux GNU (aarch64-unknown-linux-gnu)")
4. Review the logs

Common issues:
- Missing cross-compilation dependencies
- Architecture-specific code errors
- Linker issues

### Docker Push Fails

**Problem**: `unauthorized: authentication required`

**Solution**: Verify `DOCKERHUB_USERNAME` and `DOCKERHUB_TOKEN` secrets are set correctly.

### UPX Compression Fails

Some binaries may fail UPX compression. The workflow will fail if UPX returns an error.

**Workaround**: You can modify the workflow to use `continue-on-error: true` for UPX step.

### Signature Verification Fails

This means the images weren't signed correctly. Check:
1. Cosign installation succeeded
2. OIDC token was obtained
3. Images were pushed successfully

## Release Assets

Each release includes:

### Binary Files
```
ygege-x86_64-unknown-linux-gnu          # 8 regular binaries
ygege-x86_64-unknown-linux-gnu-upx      # 8 UPX compressed binaries
ygege-x86_64-pc-windows-msvc.exe
ygege-x86_64-pc-windows-msvc-upx.exe
... (and so on for all platforms)
```

### SBOM Files
```
ygege-ghcr-image-v1.0.0.sbom           # SBOM for GHCR image
ygege-dockerhub-image-v1.0.0.sbom      # SBOM for Docker Hub image
```

## Version Information

All binaries include embedded version information:
- Build commit SHA
- Build date
- Build branch/tag

View version info:
```bash
./ygege --version
```

## Workflow Duration

Approximate times:
- **Changelog generation**: ~10 seconds
- **Binary builds**: 5-15 minutes (parallel)
- **Docker build**: 10-20 minutes
- **Signing & verification**: 2-5 minutes
- **Total**: ~20-40 minutes

## Canceling a Release

If you need to stop a release in progress:

1. Cancel the workflow in GitHub Actions
2. Delete the draft release if created
3. Delete the tag locally and remotely:
```bash
git tag -d v1.0.0
git push origin :refs/tags/v1.0.0
```

## Best Practices

1. **Test before tagging**: Run the CI workflow on `develop` first
2. **Use semantic versioning**: Follow MAJOR.MINOR.PATCH format
3. **Write good commit messages**: They'll appear in the changelog
4. **Review the draft**: Always check the draft release before it goes public
5. **Don't force-push tags**: Tags should be immutable

## Integration with CI Workflow

The release workflow is separate from the CI workflow:

| Workflow | Trigger | Purpose | Artifacts |
|----------|---------|---------|-----------|
| **CI** | Push to branches | Test & preview | 7-day temporary artifacts |
| **Release** | Push tags | Production release | Permanent GitHub release |

## Questions?

- **Why 16 binaries?** To support every major platform (8 platforms × 2 versions)
- **Why UPX versions?** Smaller download size for bandwidth-constrained users
- **Why sign Docker images?** Security and supply chain integrity
- **Why two registries?** Docker Hub for public access, GHCR as backup
- **Can I skip some platforms?** Yes, just remove them from the matrix in the workflow

## Example: Full Release Process

```bash
# 1. Finish your feature
git checkout develop
git commit -am "feat: add awesome feature"
git push origin develop

# 2. Verify CI passes
# Check: https://github.com/UwUDev/ygege/actions

# 3. Merge to master
git checkout master
git merge develop
git push origin master

# 4. Create release tag
git tag v1.2.0
git push origin v1.2.0

# 5. Monitor release workflow
# Check: https://github.com/UwUDev/ygege/actions

# 6. Celebrate! 🎉
```

---

**Last Updated**: November 11, 2025  
**Workflow Version**: 2.0 (Multi-arch with auto-upload)
