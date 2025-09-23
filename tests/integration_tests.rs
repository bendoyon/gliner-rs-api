use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use serde_json::Value;

// Import the rocket function from the main crate
use gliner_rs_api::rocket;

fn create_test_client() -> Client {
    Client::tracked(rocket()).expect("valid rocket instance")
}

#[test]
fn test_api_endpoints_integration() {
    let client = create_test_client();
    
    // Test all endpoints in sequence
    let endpoints = vec![
        ("/", "Welcome to Gliner RS API"),
        ("/health", "API is running"),
        ("/api/version", "0.1.0"),
    ];
    
    for (endpoint, expected_data) in endpoints {
        let response = client.get(endpoint).dispatch();
        assert_eq!(response.status(), Status::Ok);
        
        let json_response: Value = response.into_json().expect("valid JSON");
        
        if endpoint == "/health" {
            assert_eq!(json_response["status"], "ok");
            assert_eq!(json_response["message"], expected_data);
        } else {
            assert_eq!(json_response["success"], true);
            assert_eq!(json_response["data"], expected_data);
        }
    }
}

#[test]
fn test_content_type_headers() {
    let client = create_test_client();
    let response = client.get("/health").dispatch();
    
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}

#[test]
fn test_error_handling() {
    let client = create_test_client();
    
    // Test 404 for non-existent routes
    let response = client.get("/non-existent").dispatch();
    assert_eq!(response.status(), Status::NotFound);
    
    // Test unsupported methods
    let response = client.post("/health").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_json_structure_consistency() {
    let client = create_test_client();
    
    // Test that all API endpoints return consistent JSON structure
    let api_endpoints = vec!["/", "/api/version"];
    
    for endpoint in api_endpoints {
        let response = client.get(endpoint).dispatch();
        let json_response: Value = response.into_json().expect("valid JSON");
        
        // Check required fields
        assert!(json_response.get("success").is_some());
        assert!(json_response.get("data").is_some());
        
        // Check data types
        assert!(json_response["success"].is_boolean());
        assert!(json_response["data"].is_string());
    }
}

#[test]
fn test_health_endpoint_structure() {
    let client = create_test_client();
    let response = client.get("/health").dispatch();
    let json_response: Value = response.into_json().expect("valid JSON");
    
    // Health endpoint has different structure
    assert!(json_response.get("status").is_some());
    assert!(json_response.get("message").is_some());
    assert!(json_response["status"].is_string());
    assert!(json_response["message"].is_string());
}

#[test]
fn test_multiple_requests() {
    let client = create_test_client();
    
    // Make multiple sequential requests
    for _ in 0..10 {
        let response = client.get("/health").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
