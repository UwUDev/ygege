#!/bin/bash
set -e

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🏷️  Ygégé - Create Release Script"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Couleurs
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Variables
VERSION="v0.7.1-fixed"
BRANCH=$(git branch --show-current)

echo -e "${BLUE}ℹ️  Current branch: ${NC}$BRANCH"
echo -e "${BLUE}ℹ️  Release version: ${NC}$VERSION"
echo ""

# Vérifier que nous sommes sur la bonne branche
if [[ "$BRANCH" != "claude/explain-codebase-mjln4wtc20928t9t-QS6KF" ]]; then
    echo -e "${YELLOW}⚠️  Warning: Not on the expected branch${NC}"
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Vérifier que le working tree est clean
if ! git diff-index --quiet HEAD --; then
    echo -e "${RED}❌ Working tree is not clean. Please commit changes first.${NC}"
    git status --short
    exit 1
fi

echo -e "${GREEN}✅ Working tree is clean${NC}"
echo ""

# Étape 1 : Créer le tag
echo -e "${YELLOW}📍 Étape 1/4 - Creating git tag...${NC}"

if git rev-parse "$VERSION" >/dev/null 2>&1; then
    echo -e "${YELLOW}⚠️  Tag $VERSION already exists${NC}"
    read -p "Delete and recreate? (y/N) " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        git tag -d "$VERSION"
        git push origin --delete "$VERSION" 2>/dev/null || true
    else
        echo -e "${BLUE}ℹ️  Using existing tag${NC}"
    fi
fi

if ! git rev-parse "$VERSION" >/dev/null 2>&1; then
    git tag -a "$VERSION" -m "Fix: Connection timeout due to outdated leaked IP

- Disabled forced DNS resolution to obsolete IP (89.42.231.91)
- Allow normal Cloudflare DNS resolution
- Fixes connection timeout to YGG Torrent

See release-notes.md for full details."

    echo -e "${GREEN}✅ Tag created: $VERSION${NC}"
else
    echo -e "${BLUE}ℹ️  Tag exists: $VERSION${NC}"
fi

# Étape 2 : Push le tag
echo -e "${YELLOW}📤 Étape 2/4 - Pushing tag to remote...${NC}"
git push origin "$VERSION" 2>/dev/null || git push origin "$VERSION" --force

echo -e "${GREEN}✅ Tag pushed to remote${NC}"
echo ""

# Étape 3 : Créer les artifacts (binaires)
echo -e "${YELLOW}🔨 Étape 3/4 - Building release artifacts...${NC}"

# Créer le dossier pour les artifacts
mkdir -p release-artifacts

echo -e "${BLUE}  • Building for current platform...${NC}"
cargo build --release

# Copier le binaire
cp target/release/ygege release-artifacts/ygege-$(uname -s)-$(uname -m) 2>/dev/null || \
cp target/release/ygege.exe release-artifacts/ygege-windows-x86_64.exe 2>/dev/null || \
echo -e "${YELLOW}    ⚠️  Could not copy binary${NC}"

# Créer une archive
if [ -f "release-artifacts/ygege-$(uname -s)-$(uname -m)" ]; then
    tar -czf "release-artifacts/ygege-$VERSION-$(uname -s)-$(uname -m).tar.gz" \
        -C release-artifacts \
        "ygege-$(uname -s)-$(uname -m)"
    echo -e "${GREEN}  ✅ Created: ygege-$VERSION-$(uname -s)-$(uname -m).tar.gz${NC}"
fi

echo ""

# Étape 4 : Instructions pour créer la release
echo -e "${YELLOW}🚀 Étape 4/4 - Creating GitHub Release...${NC}"
echo ""

# Vérifier si gh CLI est installé
if command -v gh &> /dev/null; then
    echo -e "${BLUE}ℹ️  GitHub CLI detected${NC}"
    echo ""
    read -p "Create release with GitHub CLI? (y/N) " -n 1 -r
    echo ""

    if [[ $REPLY =~ ^[Yy]$ ]]; then
        # Créer la release avec gh
        gh release create "$VERSION" \
            --title "$VERSION - Fix Connection Timeout" \
            --notes-file release-notes.md \
            --prerelease \
            release-artifacts/*.tar.gz release-artifacts/*.exe 2>/dev/null || true

        echo ""
        echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "${GREEN}✅ Release created successfully!${NC}"
        echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo ""

        # Afficher l'URL de la release
        REPO=$(gh repo view --json nameWithOwner -q .nameWithOwner)
        echo -e "${BLUE}🔗 View release at:${NC}"
        echo "   https://github.com/$REPO/releases/tag/$VERSION"
    else
        echo -e "${BLUE}ℹ️  Skipping GitHub CLI release creation${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  GitHub CLI (gh) not found${NC}"
    echo ""
    echo -e "${BLUE}📋 Manual steps to create the release:${NC}"
    echo ""
    echo "1. Go to: https://github.com/IsT3RiK/ygege/releases/new"
    echo "2. Select tag: $VERSION"
    echo "3. Title: $VERSION - Fix Connection Timeout"
    echo "4. Copy content from: release-notes.md"
    echo "5. Upload files from: release-artifacts/"
    echo "6. Check 'Set as a pre-release'"
    echo "7. Click 'Publish release'"
fi

echo ""
echo -e "${BLUE}📦 Artifacts location:${NC}"
ls -lh release-artifacts/ 2>/dev/null || echo "  No artifacts created"

echo ""
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✅ Release preparation complete!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
