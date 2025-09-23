# Gliner RS API

A modern Rust API built with Rocket framework, featuring comprehensive testing and clean architecture.

## 🚀 Features

- **Rocket Framework**: Fast, type-safe web framework for Rust
- **JSON API**: RESTful endpoints with structured JSON responses
- **Comprehensive Testing**: Unit tests and integration tests
- **Health Monitoring**: Built-in health check endpoint
- **Type Safety**: Strong typing with Serde serialization

## 📁 Project Structure

```
gliner-rs-api/
├── src/
│   ├── lib.rs          # Library with API logic and unit tests
│   └── main.rs         # Binary entry point
├── tests/
│   └── integration_tests.rs  # Integration tests
├── Dockerfile          # Multi-stage Docker build
├── docker-compose.yml  # Docker Compose configuration
├── nginx.conf          # Nginx reverse proxy config
├── .dockerignore       # Docker ignore file
├── docker-build.sh     # Docker build script
├── docker-run.sh       # Docker run script
├── Cargo.toml          # Dependencies
├── run_tests.sh        # Test runner script
└── README.md           # This file
```

## 🛠️ Installation & Setup

### Prerequisites
- Rust 1.70+ (2021 edition)
- Cargo package manager

### Quick Start

1. **Clone and navigate to the project:**
   ```bash
   cd gliner-rs-api
   ```

2. **Install dependencies:**
   ```bash
   cargo build
   ```

3. **Run the API server:**
   ```bash
   cargo run
   ```

   The server will start on `http://127.0.0.1:8000`

## 🌐 API Endpoints

### Base URL
```
http://127.0.0.1:8000
```

### Available Endpoints

| Method | Endpoint | Description | Response |
|--------|----------|-------------|----------|
| `GET` | `/` | Welcome message | `{"success": true, "data": "Welcome to Gliner RS API", "message": null}` |
| `GET` | `/health` | Health check | `{"status": "ok", "message": "API is running"}` |
| `GET` | `/api/version` | API version | `{"success": true, "data": "0.1.0", "message": null}` |

### Example Requests

```bash
# Health check
curl http://127.0.0.1:8000/health

# Welcome message
curl http://127.0.0.1:8000/

# API version
curl http://127.0.0.1:8000/api/version
```

### Example Responses

**Health Check:**
```json
{
  "status": "ok",
  "message": "API is running"
}
```

**API Response:**
```json
{
  "success": true,
  "data": "Welcome to Gliner RS API",
  "message": null
}
```

## 🧪 Testing

The project includes comprehensive testing with both unit tests and integration tests.

### Test Types

#### **Unit Tests** (in `src/lib.rs`)
- Response structure validation
- JSON serialization/deserialization
- Individual endpoint testing
- Error handling validation

#### **Integration Tests** (in `tests/integration_tests.rs`)
- End-to-end API testing
- Content type validation
- Error scenario testing
- Performance testing

### Running Tests

#### **Run All Tests**
```bash
cargo test
```

#### **Run Specific Test Types**
```bash
# Unit tests only
cargo test --lib

# Integration tests only  
cargo test --test integration_tests

# Run with verbose output
cargo test -- --nocapture
```

#### **Run Specific Tests**
```bash
# Run a specific test
cargo test test_health_check_response

# Run tests matching a pattern
cargo test health
```

#### **Use the Test Runner Script**
```bash
./run_tests.sh
```

### Test Coverage

**Unit Tests (6 tests):**
- ✅ `test_health_check_response` - Health endpoint validation
- ✅ `test_index_response` - Root endpoint validation  
- ✅ `test_version_response` - Version endpoint validation
- ✅ `test_404_for_unknown_route` - Error handling
- ✅ `test_health_response_serialization` - JSON serialization
- ✅ `test_api_response_serialization` - Response structure validation

**Integration Tests (6 tests):**
- ✅ `test_api_endpoints_integration` - Full endpoint testing
- ✅ `test_content_type_headers` - Header validation
- ✅ `test_error_handling` - Error scenario testing
- ✅ `test_json_structure_consistency` - Response format validation
- ✅ `test_health_endpoint_structure` - Health endpoint structure
- ✅ `test_multiple_requests` - Basic performance testing

## 🔧 Development

### Adding New Endpoints

1. **Define the endpoint function in `src/lib.rs`:**
   ```rust
   #[get("/api/new-endpoint")]
   pub fn new_endpoint() -> Json<ApiResponse<String>> {
       Json(ApiResponse {
           success: true,
           data: Some("New endpoint data".to_string()),
           message: None,
       })
   }
   ```

2. **Add the route to the rocket function:**
   ```rust
   #[launch]
   pub fn rocket() -> Rocket<Build> {
       rocket::build()
           .mount("/", routes![index, health_check, version, new_endpoint])
   }
   ```

3. **Add tests for the new endpoint:**
   ```rust
   #[test]
   fn test_new_endpoint() {
       let client = create_test_client();
       let response = client.get("/api/new-endpoint").dispatch();
       
       assert_eq!(response.status(), Status::Ok);
       let api_response: ApiResponse<String> = response.into_json().expect("valid JSON");
       assert!(api_response.success);
   }
   ```

### Project Dependencies

```toml
[dependencies]
rocket = { version = "0.5", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

## 🚀 Deployment

### Development
```bash
cargo run
```

### Production Build
```bash
cargo build --release
./target/release/gliner-rs-api
```

## 🐳 Docker Deployment

The project includes comprehensive Docker support for easy deployment and scaling.

### Quick Start with Docker

#### **Option 1: Using Docker Scripts (Recommended)**
```bash
# Build and run with one command
./docker-run.sh
```

#### **Option 2: Manual Docker Commands**
```bash
# Build the image
docker build -t gliner-rs-api:latest .

# Run the container
docker run -d --name gliner-rs-api -p 8000:8000 gliner-rs-api:latest
```

#### **Option 3: Docker Compose (Full Stack)**
```bash
# Start with nginx reverse proxy
docker-compose up -d

# View logs
docker-compose logs -f
```

### Docker Features

- **Multi-stage Build**: Optimized production image
- **Security**: Non-root user execution
- **Health Checks**: Built-in health monitoring
- **Small Image Size**: Minimal runtime dependencies
- **Reverse Proxy**: Optional nginx configuration

### Docker Commands Reference

```bash
# Build image
./docker-build.sh
# or
docker build -t gliner-rs-api:latest .

# Run container
./docker-run.sh
# or
docker run -d --name gliner-rs-api -p 8000:8000 gliner-rs-api:latest

# View logs
docker logs gliner-rs-api

# Stop container
docker stop gliner-rs-api

# Remove container
docker rm gliner-rs-api

# Docker Compose
docker-compose up -d          # Start services
docker-compose down           # Stop services
docker-compose logs -f        # View logs
docker-compose ps             # Check status
```

### Docker Environment Variables

```bash
# Custom port
docker run -p 8080:8000 -e ROCKET_PORT=8000 gliner-rs-api:latest

# Custom address
docker run -p 8000:8000 -e ROCKET_ADDRESS=0.0.0.0 gliner-rs-api:latest
```

### Production Docker Setup

For production deployment, use the docker-compose setup with nginx:

```yaml
# docker-compose.prod.yml
version: '3.8'
services:
  gliner-api:
    build: .
    restart: always
    environment:
      - ROCKET_ADDRESS=0.0.0.0
      - ROCKET_PORT=8000
    networks:
      - api-network

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
    depends_on:
      - gliner-api
    restart: always
    networks:
      - api-network
```

## 📊 API Response Structure

### Standard API Response
```json
{
  "success": boolean,
  "data": any | null,
  "message": string | null
}
```

### Health Response
```json
{
  "status": string,
  "message": string
}
```

## 🛡️ Security Features

- **Shield Protection**: Built-in security headers
- **Content Type Validation**: Proper JSON content types
- **Input Validation**: Type-safe request/response handling

## 📝 Logging

The API includes comprehensive logging:
- Request/response logging
- Error tracking
- Performance monitoring

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License.

## 🆘 Troubleshooting

### Common Issues

**Port already in use:**
```bash
# Kill existing processes
pkill -f gliner-rs-api
# Or use a different port
ROCKET_PORT=8001 cargo run
```

**Tests failing:**
```bash
# Clean and rebuild
cargo clean
cargo test
```

**Dependencies issues:**
```bash
# Update dependencies
cargo update
cargo build
```

---

**Happy coding! 🦀**
