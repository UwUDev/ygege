#!/bin/bash
set -e

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔧 Ygégé - Build & Test Script (avec fix)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Couleurs
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Variables de build
BUILD_COMMIT=$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")
BUILD_DATE=$(date -u +"%Y-%m-%dT%H:%M:%S%z")
BUILD_BRANCH=$(git branch --show-current 2>/dev/null || echo "unknown")

echo -e "${BLUE}ℹ️  Build Info:${NC}"
echo "   Commit: $BUILD_COMMIT"
echo "   Date: $BUILD_DATE"
echo "   Branch: $BUILD_BRANCH"
echo ""

# Vérifier si Docker est installé
if ! command -v docker &> /dev/null; then
    echo -e "${RED}❌ Docker n'est pas installé${NC}"
    exit 1
fi

echo -e "${YELLOW}📦 Étape 1/5 - Arrêt des anciens conteneurs...${NC}"
docker stop ygege-fixed 2>/dev/null || true
docker rm ygege-fixed 2>/dev/null || true

echo -e "${YELLOW}🏗️  Étape 2/5 - Build de l'image Docker...${NC}"
docker build \
    --build-arg BUILD_COMMIT="$BUILD_COMMIT" \
    --build-arg BUILD_DATE="$BUILD_DATE" \
    --build-arg BUILD_BRANCH="$BUILD_BRANCH" \
    -t ygege-fixed:latest \
    -f docker/Dockerfile \
    .

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Build réussi !${NC}"
else
    echo -e "${RED}❌ Échec du build${NC}"
    exit 1
fi

echo -e "${YELLOW}🔍 Étape 3/5 - Informations sur l'image...${NC}"
docker images ygege-fixed:latest
echo ""

echo -e "${YELLOW}🚀 Étape 4/5 - Démarrage du conteneur...${NC}"

# Vérifier si docker-compose.test.yml existe
if [ -f "docker-compose.test.yml" ]; then
    echo -e "${BLUE}ℹ️  Utilisation de docker-compose.test.yml${NC}"
    echo -e "${YELLOW}⚠️  IMPORTANT: Configurez vos identifiants YGG dans docker-compose.test.yml${NC}"
    echo ""
    read -p "Voulez-vous lancer avec Docker Compose ? (y/N) " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        docker compose -f docker-compose.test.yml up -d
        CONTAINER_NAME="ygege-fixed"
    else
        echo -e "${BLUE}ℹ️  Lancez manuellement avec:${NC}"
        echo "   docker compose -f docker-compose.test.yml up -d"
        exit 0
    fi
else
    echo -e "${YELLOW}⚠️  docker-compose.test.yml non trouvé${NC}"
    echo -e "${BLUE}ℹ️  Lancez manuellement avec:${NC}"
    echo '   docker run -d \'
    echo '     --name ygege-fixed \'
    echo '     -p 8715:8715 \'
    echo '     -e YGG_USERNAME="votre_username" \'
    echo '     -e YGG_PASSWORD="votre_password" \'
    echo '     -e YGG_DOMAIN="yggtorrent.org" \'
    echo '     ygege-fixed:latest'
    exit 0
fi

echo -e "${YELLOW}🧪 Étape 5/5 - Tests...${NC}"
sleep 3

echo -e "${BLUE}📋 Logs du conteneur:${NC}"
docker logs $CONTAINER_NAME 2>&1 | tail -20

echo ""
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✅ Build et démarrage terminés !${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "${BLUE}📊 Commandes utiles:${NC}"
echo ""
echo "   Voir les logs en temps réel:"
echo "   $ docker logs -f $CONTAINER_NAME"
echo ""
echo "   Tester le health check:"
echo "   $ curl http://localhost:8715/health"
echo ""
echo "   Tester le status complet:"
echo "   $ curl http://localhost:8715/status | jq"
echo ""
echo "   Tester une recherche:"
echo "   $ curl \"http://localhost:8715/search?name=debian\" | jq"
echo ""
echo "   Arrêter le conteneur:"
echo "   $ docker stop $CONTAINER_NAME"
echo ""
echo "   Redémarrer le conteneur:"
echo "   $ docker restart $CONTAINER_NAME"
echo ""
echo -e "${YELLOW}⚠️  N'oubliez pas de configurer vos identifiants YGG !${NC}"
