#!/bin/bash

echo "🤖 Setting up GLiNER Model"
echo "=========================="

# Create models directory for the GLiNER model
mkdir -p models/onnx-community/gliner-multitask-large-v0.5

echo "📁 Created model directory structure"

# Download model files for the GLiNER model
echo ""
echo "📥 Downloading GLiNER Model Files:"
echo "=================================="
echo ""

# Download tokenizer
echo "Downloading tokenizer..."
wget -O models/onnx-community/gliner-multitask-large-v0.5/tokenizer.json \
  'https://huggingface.co/onnx-community/gliner-multitask-large-v0.5/raw/main/tokenizer.json'

# Download ONNX model
echo "Downloading ONNX model..."
wget -O models/onnx-community/gliner-multitask-large-v0.5/model.onnx \
  'https://huggingface.co/onnx-community/gliner-multitask-large-v0.5/resolve/main/onnx/model.onnx'

echo ""
echo "✅ Model setup complete!"
echo ""
echo "🚀 To run the API:"
echo "   cargo run"
echo ""
echo "🧪 To test PII detection:"
echo "   curl -X POST http://localhost:8000/api/pii/detect \\"
echo "     -H 'Content-Type: application/json' \\"
echo "     -d '{\"text\": \"My name is John Doe and my email is john@example.com\"}'"
echo ""
echo "🔧 Environment Variables:"
echo "   GLINER_MODEL=onnx-community/gliner-multitask-large-v0.5  # Default model"
