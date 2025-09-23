#!/bin/bash

echo "🚀 Starting Gliner RS API Container"
echo "==================================="

# Check if image exists
if ! docker image inspect gliner-rs-api:latest >/dev/null 2>&1; then
    echo "📦 Image not found. Building first..."
    ./docker-build.sh
fi

# Stop any existing container
echo "🛑 Stopping any existing containers..."
docker stop gliner-rs-api 2>/dev/null || true
docker rm gliner-rs-api 2>/dev/null || true

# Run the container
echo "🏃 Starting new container..."
docker run -d \
    --name gliner-rs-api \
    -p 8000:8000 \
    --restart unless-stopped \
    gliner-rs-api:latest

if [ $? -eq 0 ]; then
    echo "✅ Container started successfully!"
    echo ""
    echo "🌐 API is available at: http://localhost:8000"
    echo "🏥 Health check: http://localhost:8000/health"
    echo ""
    echo "📊 Container status:"
    docker ps --filter name=gliner-rs-api
    echo ""
    echo "📝 To view logs:"
    echo "   docker logs gliner-rs-api"
    echo ""
    echo "🛑 To stop:"
    echo "   docker stop gliner-rs-api"
else
    echo "❌ Failed to start container!"
    exit 1
fi
