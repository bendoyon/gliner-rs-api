use rocket::{get, post, launch, routes, serde::json::Json, Build, Rocket, State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use orp::params::RuntimeParameters;
use gliner::{
    model::GLiNER,
    model::params::Parameters,
    model::input::text::TextInput,
    model::pipeline::token::TokenMode,
};

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PiiRequest {
    pub text: String,
}

// Global model state
pub type ModelState = Arc<Mutex<Option<GLiNER<TokenMode>>>>;

#[get("/health")]
pub fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        message: "API is running".to_string(),
    })
}

#[get("/")]
pub fn index() -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: true,
        data: Some("Welcome to Gliner RS API".to_string()),
        message: None,
    })
}

#[get("/api/version")]
pub fn version() -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: true,
        data: Some("0.1.0".to_string()),
        message: None,
    })
}

#[post("/api/pii/detect", data = "<request>")]
pub async fn detect_pii(
    request: Json<PiiRequest>,
    model_state: &State<ModelState>,
) -> Result<Json<ApiResponse<serde_json::Value>>, Json<ApiResponse<String>>> {
    // Check if model is loaded
    let model_guard = model_state.lock().await;
    let model = match model_guard.as_ref() {
        Some(model) => model,
        None => {
            return Err(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("PII detection model not loaded. Please ensure model files are available.".to_string()),
            }));
        }
    };

    // Create text input for GLiNER - using the exact API from the docs
    let text_input = match TextInput::from_str(&[&request.text], &["person", "email", "phone", "address", "organization"]) {
        Ok(input) => input,
        Err(e) => {
            return Err(Json(ApiResponse {
                success: false,
                data: None,
                message: Some(format!("Failed to process input text: {}", e)),
            }));
        }
    };

    // Run inference using the exact API from the docs
    let output = match model.inference(text_input) {
        Ok(output) => output,
        Err(e) => {
            return Err(Json(ApiResponse {
                success: false,
                data: None,
                message: Some(format!("Inference failed: {}", e)),
            }));
        }
    };

    // Extract entities from the output and return them in a structured format
    let mut entities = Vec::new();
    for spans in &output.spans {
        for span in spans {
            entities.push(serde_json::json!({
                "text": span.text(),
                "label": span.class(),
                "sequence": span.sequence(),
                "probability": span.probability()
            }));
        }
    }

    let result = serde_json::json!({
        "text": request.text,
        "entities": entities,
        "total_entities": entities.len(),
        "message": "PII detection completed successfully"
    });

    Ok(Json(ApiResponse {
        success: true,
        data: Some(result),
        message: None,
    }))
}

// Initialize the model from environment variables
pub async fn init_model() -> Result<GLiNER<TokenMode>, Box<dyn std::error::Error + Send + Sync>> {
    // Get model path from environment variable, default to onnx-community/gliner-multitask-large-v0.5
    let model_name = std::env::var("GLINER_MODEL").unwrap_or_else(|_| "onnx-community/gliner-multitask-large-v0.5".to_string());
    
    // For now, we'll use local paths - in production you'd download from HuggingFace
    let model_path = format!("models/{}", model_name);
    let tokenizer_path = format!("{}/tokenizer.json", model_path);
    let onnx_path = format!("{}/model.onnx", model_path);

    println!("Loading GLiNER model: {}", model_name);
    println!("Tokenizer path: {}", tokenizer_path);
    println!("ONNX path: {}", onnx_path);

    // Use the exact API from the documentation
    let model = GLiNER::<TokenMode>::new(
        Parameters::default(),
        RuntimeParameters::default(),
        &tokenizer_path,
        &onnx_path,
    ).map_err(|e| -> Box<dyn std::error::Error + Send + Sync> {
        e
    })?;

    println!("Model loaded successfully!");
    Ok(model)
}

#[launch]
pub async fn rocket() -> Rocket<Build> {
    // Initialize model at startup
    let model = match init_model().await {
        Ok(model) => Some(model),
        Err(e) => {
            eprintln!("Failed to initialize model: {}", e);
            eprintln!("Continuing without model - PII detection will not work");
            None
        }
    };
    
    let model_state: ModelState = Arc::new(Mutex::new(model));
    
    rocket::build()
        .manage(model_state)
        .mount("/", routes![
            index, 
            health_check, 
            version, 
            detect_pii
        ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    fn create_test_client() -> Client {
        Client::tracked(rocket()).expect("valid rocket instance")
    }

    #[test]
    fn test_health_check_response() {
        let client = create_test_client();
        let response = client.get("/health").dispatch();
        
        assert_eq!(response.status(), Status::Ok);
        
        let health_response: HealthResponse = response.into_json().expect("valid JSON");
        assert_eq!(health_response.status, "ok");
        assert_eq!(health_response.message, "API is running");
    }

    #[test]
    fn test_index_response() {
        let client = create_test_client();
        let response = client.get("/").dispatch();
        
        assert_eq!(response.status(), Status::Ok);
        
        let api_response: ApiResponse<String> = response.into_json().expect("valid JSON");
        assert!(api_response.success);
        assert_eq!(api_response.data, Some("Welcome to Gliner RS API".to_string()));
        assert!(api_response.message.is_none());
    }

    #[test]
    fn test_version_response() {
        let client = create_test_client();
        let response = client.get("/api/version").dispatch();
        
        assert_eq!(response.status(), Status::Ok);
        
        let api_response: ApiResponse<String> = response.into_json().expect("valid JSON");
        assert!(api_response.success);
        assert_eq!(api_response.data, Some("0.1.0".to_string()));
        assert!(api_response.message.is_none());
    }

    #[test]
    fn test_404_for_unknown_route() {
        let client = create_test_client();
        let response = client.get("/unknown-route").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_health_response_serialization() {
        let health = HealthResponse {
            status: "ok".to_string(),
            message: "test".to_string(),
        };
        
        let json = serde_json::to_string(&health).expect("serialization should work");
        let deserialized: HealthResponse = serde_json::from_str(&json).expect("deserialization should work");
        
        assert_eq!(health.status, deserialized.status);
        assert_eq!(health.message, deserialized.message);
    }

    #[test]
    fn test_api_response_serialization() {
        let api_response = ApiResponse {
            success: true,
            data: Some("test data".to_string()),
            message: Some("test message".to_string()),
        };
        
        let json = serde_json::to_string(&api_response).expect("serialization should work");
        let deserialized: ApiResponse<String> = serde_json::from_str(&json).expect("deserialization should work");
        
        assert_eq!(api_response.success, deserialized.success);
        assert_eq!(api_response.data, deserialized.data);
        assert_eq!(api_response.message, deserialized.message);
    }

    #[test]
    fn test_pii_entities_endpoint() {
        let client = create_test_client();
        let response = client.get("/api/pii/entities").dispatch();
        
        assert_eq!(response.status(), Status::Ok);
        
        let api_response: ApiResponse<Vec<String>> = response.into_json().expect("valid JSON");
        assert!(api_response.success);
        assert!(api_response.data.is_some());
        
        let entities = api_response.data.unwrap();
        assert!(entities.contains(&"person".to_string()));
        assert!(entities.contains(&"email".to_string()));
        assert!(entities.contains(&"phone".to_string()));
    }

    #[test]
    fn test_pii_detect_without_model() {
        let client = create_test_client();
        let request_body = serde_json::json!({
            "text": "My name is John Doe and my email is john@example.com",
            "threshold": 0.5
        });
        
        let response = client
            .post("/api/pii/detect")
            .header(rocket::http::ContentType::JSON)
            .body(request_body.to_string())
            .dispatch();
        
        // Should return an error since model is not loaded
        assert_eq!(response.status(), Status::Ok);
        
        let api_response: ApiResponse<String> = response.into_json().expect("valid JSON");
        assert!(!api_response.success);
        assert!(api_response.message.unwrap().contains("model not loaded"));
    }

    #[test]
    fn test_pii_request_serialization() {
        let pii_request = PiiRequest {
            text: "Test text".to_string(),
        };
        
        let json = serde_json::to_string(&pii_request).expect("serialization should work");
        let deserialized: PiiRequest = serde_json::from_str(&json).expect("deserialization should work");
        
        assert_eq!(pii_request.text, deserialized.text);
    }

    #[test]
    fn test_pii_entity_serialization() {
        let pii_entity = PiiEntity {
            text: "John Doe".to_string(),
            label: "person".to_string(),
            confidence: 0.95,
            start: 0,
            end: 8,
        };
        
        let json = serde_json::to_string(&pii_entity).expect("serialization should work");
        let deserialized: PiiEntity = serde_json::from_str(&json).expect("deserialization should work");
        
        assert_eq!(pii_entity.text, deserialized.text);
        assert_eq!(pii_entity.label, deserialized.label);
        assert_eq!(pii_entity.confidence, deserialized.confidence);
        assert_eq!(pii_entity.start, deserialized.start);
        assert_eq!(pii_entity.end, deserialized.end);
    }
}
