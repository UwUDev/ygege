---
sidebar_position: 3
---

# PR Preview Workflow Documentation

## Overview

The preview workflow allows contributors and reviewers to test pull requests without manually building Docker images. When a PR is labeled with `preview`, a Docker image is automatically built and published for easy testing.

## How It Works

### 🏷️ Label-Based Activation

The preview workflow is **opt-in** using GitHub labels:

1. PR is created → No preview image (by default)
2. Add `preview` label → Workflow triggers and builds Docker image
3. Push new commits → Preview image auto-updates
4. Remove `preview` label → Workflow stops building (old images remain accessible)

### 🎯 Why Label-Based?

- **Save resources**: Only build previews when actually needed
- **Contributor choice**: Author decides if preview is necessary
- **Review convenience**: Reviewers can request previews for complex changes
- **Cost control**: Avoid building Docker images for every PR

## Usage Guide

### For PR Authors

#### Enable Preview for Your PR

1. Open your pull request on GitHub
2. On the right sidebar, click "Labels"
3. Add the `preview` label
4. Wait ~10-15 minutes for the build to complete
5. Check the PR comments for Docker pull commands

#### Update Your Preview

Just push new commits! The preview image updates automatically when you push to the PR branch.

#### Disable Preview

Simply remove the `preview` label from your PR.

### For Reviewers

#### Request a Preview

If you want to test a PR before approving:

1. Comment on the PR: "Could you add the `preview` label so I can test this?"
2. Or add the label yourself (if you have permissions)
3. Wait for the build to complete
4. Use the Docker commands from the bot's comment

## Preview Image Tags

For **PR #123** from branch **feature/awesome**:

### Tags Created
- `uwucode/ygege:pr-123` - Simple PR number tag (updates on every push)
- `uwucode/ygege:pr-123-feature-awesome` - Includes branch name
- `ghcr.io/uwudev/ygege:pr-123` - GHCR version
- `ghcr.io/uwudev/ygege:pr-123-feature-awesome` - GHCR with branch

### Which Tag to Use?

**Quick testing**: Use `pr-123` (shorter)
```bash
docker pull uwucode/ygege:pr-123
```

**Multiple PRs**: Use `pr-123-feature-awesome` (more descriptive)
```bash
docker pull uwucode/ygege:pr-123-feature-awesome
```

## Testing a Preview

### Quick Test

```bash
# Pull the preview image
docker pull uwucode/ygege:pr-123

# Check version info
docker run --rm uwucode/ygege:pr-123 --version

# Run the preview
docker run -p 8080:8080 uwucode/ygege:pr-123
```

### Using Docker Compose

Create `docker-compose.preview.yml`:

```yaml
services:
  ygege-preview:
    image: uwucode/ygege:pr-123
    ports:
      - "8080:8080"
    environment:
      - LOG_LEVEL=debug
    volumes:
      - ./config:/config
```

Run it:
```bash
docker compose -f docker-compose.preview.yml up
```

### Compare with Production

```bash
# Pull both images
docker pull uwucode/ygege:latest
docker pull uwucode/ygege:pr-123

# Compare sizes
docker images | grep ygege

# Run side-by-side
docker run -p 8080:8080 uwucode/ygege:latest  # Production
docker run -p 8081:8080 uwucode/ygege:pr-123  # Preview
```

## Automatic PR Comments

The workflow posts an automated comment on your PR with:

### First Build
```markdown
## 🐳 Preview Docker Image Ready!

Your PR preview has been built and published...

### Docker Hub
docker pull uwucode/ygege:pr-123
...
```

### Subsequent Updates
The bot **updates the same comment** instead of spamming new ones.

### Label Removed
```markdown
## ℹ️ Preview Docker Image Disabled

The `preview` label has been removed...
```

## Image Details

### What's Included
- **Architectures**: linux/amd64, linux/arm64
- **Build Info**: Commit SHA, branch name, build date
- **Labels**: PR number, branch name, PR URL
- **Registries**: Docker Hub and GitHub Container Registry

### Build Args
```dockerfile
BUILD_COMMIT=abc123def456...
BUILD_DATE=2025-11-11T12:34:56Z
BUILD_BRANCH=feature/awesome-feature
```

### Caching
- Uses GitHub Actions cache for faster rebuilds
- Also caches from previous PR image if available
- Typical build time: 10-15 minutes (first build), 5-10 minutes (updates)

## Workflow Triggers

The preview workflow runs when:

| Event | Action | Builds? |
|-------|--------|---------|
| PR opened | `opened` | ✅ If has `preview` label |
| PR updated | `synchronize` | ✅ If has `preview` label |
| PR reopened | `reopened` | ✅ If has `preview` label |
| Label added | `labeled` | ✅ If label is `preview` |
| Label removed | `unlabeled` | ❌ Posts cleanup comment |

**Important**: The workflow only builds if the `preview` label is present.

## Best Practices

### When to Use Preview

✅ **Do use preview for**:
- Complex features that change behavior
- UI/UX changes that need visual testing
- Performance improvements that need benchmarking
- Bug fixes that are hard to test locally
- Changes affecting Docker configuration

❌ **Don't use preview for**:
- Simple documentation fixes
- Minor typos or code formatting
- Changes with comprehensive unit tests
- One-line bug fixes

### For PR Authors

1. **Add early**: If you know reviewers will want to test, add the label immediately
2. **Document changes**: Explain what to test in the PR description
3. **Clean up**: Remove the label once PR is merged or closed
4. **Test locally first**: Verify the Docker build works before requesting preview

### For Reviewers

1. **Be specific**: Tell the author what you want to test
2. **Share results**: Comment on the PR with your testing findings
3. **Remove when done**: Remove the label after testing is complete

## Resource Management

### Build Costs
- Each preview build uses ~10-15 minutes of GitHub Actions time
- Multi-arch builds (amd64 + arm64) use more resources
- Updates reuse cache, making subsequent builds faster

### Storage Costs
Preview images remain on Docker Hub/GHCR until manually deleted.

**Cleanup suggestions**:
- Remove preview images after PR is merged
- Set up automated cleanup for old PR images (optional)

## Troubleshooting

### Preview Build Failed

**Problem**: Workflow shows red ❌

**Check**:
1. View workflow logs in the "Actions" tab
2. Look for build errors in the "Build and push preview image" step
3. Common issues:
   - Dockerfile syntax errors
   - Missing dependencies
   - Out of memory

**Fix**: Fix the issue in your branch and push a new commit

### Can't Pull Preview Image

**Problem**: `docker pull uwucode/ygege:pr-123` fails

**Possible causes**:
1. Build hasn't completed yet - check workflow status
2. Wrong tag name - check the bot's comment for exact tag
3. Registry authentication issue - try pulling from GHCR instead

### Preview Image Is Outdated

**Problem**: Pulled image doesn't reflect latest commits

**Solution**:
```bash
# Force pull latest version
docker pull uwucode/ygege:pr-123 --no-cache

# Or remove old image first
docker rmi uwucode/ygege:pr-123
docker pull uwucode/ygege:pr-123
```

### Bot Comment Not Appearing

**Problem**: No preview comment on PR

**Check**:
1. Workflow completed successfully?
2. Check workflow logs for "Comment on PR" step
3. Bot might have permission issues

**Workaround**: Use tags manually: `uwucode/ygege:pr-{number}`

## Security Considerations

### Image Signing

Preview images are **NOT signed** with Cosign (unlike release images).

**Why?**
- Previews are for testing, not production
- Signing adds complexity and time
- Release workflow handles production signing

### Secrets

Preview workflow uses:
- `DOCKERHUB_USERNAME` - Docker Hub login
- `DOCKERHUB_TOKEN` - Docker Hub token
- `GITHUB_TOKEN` - Auto-provided by GitHub

### Trust

⚠️ **Preview images should only be used for testing**
- Don't use in production
- Don't trust previews from unknown contributors
- Always review the PR code before running preview images

## Comparison: CI vs Preview vs Release

| Workflow | Trigger | Docker Tags | Signed? | Use Case |
|----------|---------|-------------|---------|----------|
| **CI** | Push to branches | `develop`, `beta`, `master` | ❌ | Development testing |
| **Preview** | PR with label | `pr-123` | ❌ | PR testing |
| **Release** | Version tags | `v1.2.3`, `latest`, `stable` | ✅ | Production |

## Examples

### Example 1: Testing a Feature PR

```bash
# Reviewer adds preview label to PR #456

# Wait for build (~10 min)

# Pull and test
docker pull uwucode/ygege:pr-456

# Run with custom config
docker run -p 8080:8080 \
  -v $(pwd)/test-config:/config \
  uwucode/ygege:pr-456

# Test the feature
curl http://localhost:8080/api/endpoint

# Leave feedback on PR
# Remove preview label when done
```

### Example 2: Author Providing Preview

```bash
# Author opens PR #789 with new Docker optimization

# Author adds preview label
# Adds comment: "Preview image available for testing performance improvements"

# Workflow builds and comments with tags

# Reviewers test and confirm performance gains

# PR merged, preview label removed
```

### Example 3: Comparing Previews

```bash
# Testing multiple PRs side-by-side
docker pull uwucode/ygege:pr-111  # Feature A
docker pull uwucode/ygege:pr-222  # Feature B

# Run Feature A on port 8081
docker run -d -p 8081:8080 --name preview-a uwucode/ygege:pr-111

# Run Feature B on port 8082
docker run -d -p 8082:8080 --name preview-b uwucode/ygege:pr-222

# Compare features
curl http://localhost:8081/api/test
curl http://localhost:8082/api/test

# Clean up
docker stop preview-a preview-b
docker rm preview-a preview-b
```

## FAQ

### Q: Does every PR get a preview?

**A**: No! Only PRs with the `preview` label. This saves resources.

### Q: How long does a preview build take?

**A**: 10-15 minutes for first build, 5-10 minutes for updates (thanks to caching).

### Q: Can I preview on multiple branches?

**A**: Yes! Each PR gets its own unique tag (pr-123, pr-456, etc.).

### Q: What happens to preview images after PR is merged?

**A**: They remain on Docker Hub/GHCR until manually deleted. Consider cleaning them up periodically.

### Q: Can I test ARM images?

**A**: Yes! Preview images are multi-arch (amd64 + arm64).

### Q: Why not just build locally?

**A**: You can! But preview images are convenient for reviewers who don't want to clone and build.

### Q: Do preview images include the latest code?

**A**: Yes! They're built from the exact commit SHA in the PR.

## Tips & Tricks

### Fast Preview Access

Create an alias:
```bash
alias ygege-pr='docker run --rm -p 8080:8080 uwucode/ygege:pr-'

# Usage
ygege-pr123  # Runs PR #123 preview
```

### Auto-update Testing Script

```bash
#!/bin/bash
PR_NUM=$1

while true; do
  echo "Pulling latest pr-$PR_NUM..."
  docker pull uwucode/ygege:pr-$PR_NUM
  
  echo "Restarting container..."
  docker stop ygege-preview 2>/dev/null
  docker rm ygege-preview 2>/dev/null
  docker run -d --name ygege-preview -p 8080:8080 uwucode/ygege:pr-$PR_NUM
  
  echo "Preview updated! Sleeping for 5 minutes..."
  sleep 300
done
```

### Multi-Reviewer Testing

Use Docker Compose with environment variable:
```bash
export PR_NUMBER=123
docker compose up
```

```yaml
# docker-compose.yml
services:
  ygege:
    image: uwucode/ygege:pr-${PR_NUMBER:-develop}
    ports:
      - "8080:8080"
```

---

**Last Updated**: November 11, 2025  
**Workflow Version**: 2.0 (Label-based PR previews)
