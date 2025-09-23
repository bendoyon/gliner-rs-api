#!/bin/bash

echo "🧪 Running Rocket API Tests"
echo "=========================="

echo "📦 Building the project..."
cargo build

echo ""
echo "🔬 Running unit tests..."
cargo test --lib

echo ""
echo "🌐 Running integration tests..."
cargo test --test integration_tests

echo ""
echo "📊 Running API test suite..."
cargo test --test api_test_suite

echo ""
echo "✅ All tests completed!"
