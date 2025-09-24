# Multi-stage build for optimized production image
FROM rust:1.82-slim as builder

# Install system dependencies including ONNX Runtime
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    wget \
    curl \
    build-essential \
    g++ \
    && rm -rf /var/lib/apt/lists/*

# Download and install ONNX Runtime
RUN wget https://github.com/microsoft/onnxruntime/releases/download/v1.20.0/onnxruntime-linux-x64-1.20.0.tgz \
    && tar -xzf onnxruntime-linux-x64-1.20.0.tgz \
    && cp onnxruntime-linux-x64-1.20.0/lib/libonnxruntime.so* /usr/local/lib/ \
    && ln -sf /usr/local/lib/libonnxruntime.so.1.20.0 /usr/local/lib/libonnxruntime.so \
    && ldconfig \
    && rm -rf onnxruntime-linux-x64-1.20.0*

# Set working directory
WORKDIR /app

# Copy manifest files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release && rm -rf src

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    wget \
    && rm -rf /var/lib/apt/lists/*

# Download and install ONNX Runtime for runtime stage
RUN wget https://github.com/microsoft/onnxruntime/releases/download/v1.20.0/onnxruntime-linux-x64-1.20.0.tgz \
    && tar -xzf onnxruntime-linux-x64-1.20.0.tgz \
    && cp onnxruntime-linux-x64-1.20.0/lib/libonnxruntime.so* /usr/local/lib/ \
    && ln -sf /usr/local/lib/libonnxruntime.so.1.20.0 /usr/local/lib/libonnxruntime.so \
    && ldconfig \
    && rm -rf onnxruntime-linux-x64-1.20.0*

# Create non-root user
RUN useradd -r -s /bin/false appuser

# Set working directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/gliner-rs-api /usr/local/bin/gliner-rs-api

# Create models directory and download GLiNER model
RUN mkdir -p /app/models/onnx-community/gliner-multitask-large-v0.5 && \
    wget -O /app/models/onnx-community/gliner-multitask-large-v0.5/tokenizer.json \
        'https://huggingface.co/onnx-community/gliner-multitask-large-v0.5/raw/main/tokenizer.json' && \
    wget -O /app/models/onnx-community/gliner-multitask-large-v0.5/model.onnx \
        'https://huggingface.co/onnx-community/gliner-multitask-large-v0.5/resolve/main/onnx/model.onnx'

# Change ownership to non-root user
RUN chown -R appuser:appuser /usr/local/bin/gliner-rs-api /app

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 8000

# Set default environment variable for the model
ENV GLINER_MODEL=onnx-community/gliner-multitask-large-v0.5

# Configure Rocket to bind to all interfaces
ENV ROCKET_ADDRESS=0.0.0.0

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8000/health || exit 1

# Run the application
CMD ["gliner-rs-api"]
