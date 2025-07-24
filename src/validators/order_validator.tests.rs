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
}
