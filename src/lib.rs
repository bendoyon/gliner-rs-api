use rocket::{get, launch, routes, serde::json::Json, Build, Rocket};
use serde::{Deserialize, Serialize};

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

#[launch]
pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, health_check, version])
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
}
