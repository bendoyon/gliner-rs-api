#!/bin/bash

echo "🐳 Building Gliner RS API Docker Image"
echo "======================================"

# Build the Docker image
echo "📦 Building Docker image..."
docker build -t gliner-rs-api:latest .

if [ $? -eq 0 ]; then
    echo "✅ Docker image built successfully!"
    echo ""
    echo "🚀 To run the container:"
    echo "   docker run -p 8000:8000 gliner-rs-api:latest"
    echo ""
    echo "🔍 To test the API:"
    echo "   curl http://localhost:8000/health"
    echo ""
    echo "📊 To view running containers:"
    echo "   docker ps"
else
    echo "❌ Docker build failed!"
    exit 1
fi
