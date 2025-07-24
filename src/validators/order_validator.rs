use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use serde::{Serialize};
use serde_json::json;
use crate::utils::Order;

#[derive(Debug, Serialize)]
pub struct ValidationError {
    pub error: String,
    pub field: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ServerError {
    pub error: String,
    pub message: String,
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": self.error,
            "field": self.field
        }));
        (StatusCode::BAD_REQUEST, body).into_response()
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": self.error,
            "message": self.message
        }));
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

#[derive(Debug)]
pub enum ApiError {
    Validation(ValidationError),
    Server(ServerError),
    NotFound(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Validation(err) => err.into_response(),
            ApiError::Server(err) => err.into_response(),
            ApiError::NotFound(message) => {
                let body = Json(json!({
                    "error": message
                }));
                (StatusCode::NOT_FOUND, body).into_response()
            }
        }
    }
}

impl From<ValidationError> for ApiError {
    fn from(err: ValidationError) -> Self {
        ApiError::Validation(err)
    }
}

impl From<ServerError> for ApiError {
    fn from(err: ServerError) -> Self {
        ApiError::Server(err)
    }
}

/// Validates an order to ensure all fields meet the required criteria
pub fn validate_order(order: &Order) -> Result<(), ValidationError> {
    // Validate ID
    if order.id == 0 {
        return Err(ValidationError {
            error: "Order ID must be greater than 0".to_string(),
            field: Some("id".to_string()),
        });
    }

    // Validate item
    if order.item.trim().is_empty() {
        return Err(ValidationError {
            error: "Item name cannot be empty".to_string(),
            field: Some("item".to_string()),
        });
    }

    // Check for item length
    if order.item.len() > 100 {
        return Err(ValidationError {
            error: "Item name cannot exceed 100 characters".to_string(),
            field: Some("item".to_string()),
        });
    }

    // Validate status
    let valid_statuses = ["pending", "processing", "shipped", "delivered", "cancelled"];
    if !valid_statuses.contains(&order.status.as_str()) {
        return Err(ValidationError {
            error: format!("Status must be one of: {}", valid_statuses.join(", ")),
            field: Some("status".to_string()),
        });
    }

    // Validate quantity
    if order.quantity == 0 {
        return Err(ValidationError {
            error: "Quantity must be greater than 0".to_string(),
            field: Some("quantity".to_string()),
        });
    }

    if order.quantity > 1000 {
        return Err(ValidationError {
            error: "Quantity cannot exceed 1000".to_string(),
            field: Some("quantity".to_string()),
        });
    }

    Ok(())
}

/// Validates only the status field of an order
pub fn validate_status(status: &str) -> Result<(), ValidationError> {
    let valid_statuses = ["pending", "processing", "shipped", "delivered", "cancelled"];
    if !valid_statuses.contains(&status) {
        return Err(ValidationError {
            error: format!("Status must be one of: {}", valid_statuses.join(", ")),
            field: Some("status".to_string()),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::Order;

    fn create_valid_order() -> Order {
        Order {
            id: 1,
            item: "Test Item".to_string(),
            status: "pending".to_string(),
            quantity: 5,
        }
    }

    #[test]
    fn test_validate_order_success() {
        let order = create_valid_order();
        assert!(validate_order(&order).is_ok());
    }

    #[test]
    fn test_validate_order_zero_id() {
        let mut order = create_valid_order();
        order.id = 0;
        
        let result = validate_order(&order);
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert_eq!(error.error, "Order ID must be greater than 0");
        assert_eq!(error.field, Some("id".to_string()));
    }

    #[test]
    fn test_validate_order_empty_item() {
        let mut order = create_valid_order();
        order.item = "".to_string();
        
        let result = validate_order(&order);
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert_eq!(error.error, "Item name cannot be empty");
        assert_eq!(error.field, Some("item".to_string()));
    }

    #[test]
    fn test_validate_order_whitespace_only_item() {
        let mut order = create_valid_order();
        order.item = "   ".to_string();
        
        let result = validate_order(&order);
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert_eq!(error.error, "Item name cannot be empty");
        assert_eq!(error.field, Some("item".to_string()));
    }

    #[test]
    fn test_validate_order_long_item_name() {
        let mut order = create_valid_order();
        order.item = "a".repeat(101); // 101 characters
        
        let result = validate_order(&order);
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert_eq!(error.error, "Item name cannot exceed 100 characters");
        assert_eq!(error.field, Some("item".to_string()));
    }

    #[test]
    fn test_validate_order_max_length_item_name() {
        let mut order = create_valid_order();
        order.item = "a".repeat(100); // Exactly 100 characters - should be valid
        
        let result = validate_order(&order);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_order_invalid_status() {
        let mut order = create_valid_order();
        order.status = "invalid_status".to_string();
        
        let result = validate_order(&order);
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert!(error.error.contains("Status must be one of:"));
        assert_eq!(error.field, Some("status".to_string()));
    }

    #[test]
    fn test_validate_order_all_valid_statuses() {
        let valid_statuses = ["pending", "processing", "shipped", "delivered", "cancelled"];
        
        for status in valid_statuses.iter() {
            let mut order = create_valid_order();
            order.status = status.to_string();
            
            let result = validate_order(&order);
            assert!(result.is_ok(), "Status '{}' should be valid", status);
        }
    }

    #[test]
    fn test_validate_order_zero_quantity() {
        let mut order = create_valid_order();
        order.quantity = 0;
        
        let result = validate_order(&order);
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert_eq!(error.error, "Quantity must be greater than 0");
        assert_eq!(error.field, Some("quantity".to_string()));
    }

    #[test]
    fn test_validate_order_excessive_quantity() {
        let mut order = create_valid_order();
        order.quantity = 1001;
        
        let result = validate_order(&order);
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert_eq!(error.error, "Quantity cannot exceed 1000");
        assert_eq!(error.field, Some("quantity".to_string()));
    }

    #[test]
    fn test_validate_order_max_quantity() {
        let mut order = create_valid_order();
        order.quantity = 1000; // Exactly 1000 - should be valid
        
        let result = validate_order(&order);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validation_error_serialization() {
        let error = ValidationError {
            error: "Test error".to_string(),
            field: Some("test_field".to_string()),
        };
        
        // Test that the error can be serialized (this would fail if Serialize wasn't implemented)
        let _serialized = serde_json::to_string(&error).unwrap();
    }

    #[test]
    fn test_validation_error_without_field() {
        let error = ValidationError {
            error: "Test error".to_string(),
            field: None,
        };
        
        let serialized = serde_json::to_string(&error).unwrap();
        assert!(serialized.contains("\"field\":null"));
    }

    #[test]
    fn test_validate_status_success() {
        let valid_statuses = ["pending", "processing", "shipped", "delivered", "cancelled"];
        
        for status in valid_statuses.iter() {
            let result = validate_status(status);
            assert!(result.is_ok(), "Status '{}' should be valid", status);
        }
    }

    #[test]
    fn test_validate_status_invalid() {
        let invalid_statuses = ["invalid", "PENDING", "complete", "done", ""];
        
        for status in invalid_statuses.iter() {
            let result = validate_status(status);
            assert!(result.is_err(), "Status '{}' should be invalid", status);
            
            let error = result.unwrap_err();
            assert!(error.error.contains("Status must be one of:"));
            assert_eq!(error.field, Some("status".to_string()));
        }
    }

    #[test]
    fn test_validate_status_case_sensitive() {
        // Test that validation is case-sensitive
        let result = validate_status("PENDING");
        assert!(result.is_err());
        
        let result = validate_status("Pending");
        assert!(result.is_err());
        
        let result = validate_status("pending");
        assert!(result.is_ok());
    }

    #[test]
    fn test_server_error_creation() {
        let server_error = ServerError {
            error: "Internal error".to_string(),
            message: "Something went wrong".to_string(),
        };
        
        // Test serialization
        let serialized = serde_json::to_string(&server_error).unwrap();
        assert!(serialized.contains("Internal error"));
        assert!(serialized.contains("Something went wrong"));
    }

    #[test]
    fn test_api_error_from_validation_error() {
        let validation_error = ValidationError {
            error: "Test error".to_string(),
            field: Some("test_field".to_string()),
        };
        
        let api_error: ApiError = validation_error.into();
        match api_error {
            ApiError::Validation(_) => {}, // Expected
            _ => panic!("Expected Validation variant"),
        }
    }

    #[test]
    fn test_api_error_from_server_error() {
        let server_error = ServerError {
            error: "Server error".to_string(),
            message: "Internal issue".to_string(),
        };
        
        let api_error: ApiError = server_error.into();
        match api_error {
            ApiError::Server(_) => {}, // Expected
            _ => panic!("Expected Server variant"),
        }
    }

    #[test]
    fn test_api_error_not_found() {
        let api_error = ApiError::NotFound("Resource not found".to_string());
        match api_error {
            ApiError::NotFound(msg) => assert_eq!(msg, "Resource not found"),
            _ => panic!("Expected NotFound variant"),
        }
    }
}
