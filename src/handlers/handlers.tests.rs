#[cfg(test)]
mod tests {
    use crate::utils::{init_db, Order, DbPool};
    use crate::handlers::handlers::*;
    use crate::validators::ApiError;
    use axum::{
        extract::{Path, State},
        Json
    };
    use tokio;

    async fn setup_test_db() -> DbPool {
        // Use in-memory database for tests to ensure isolation
        let db_pool = init_db().await.expect("Failed to initialize test database");
        db_pool
    }

    async fn create_test_order(db_pool: &DbPool) -> Order {
        let order = Order {
            id: 1,
            item: "Test Item".to_string(),
            status: "pending".to_string(),
            quantity: 5,
        };
        let _result = add_order(State(db_pool.clone()), Json(order.clone())).await.unwrap();
        order
    }

    #[tokio::test]
    async fn test_get_orders_empty() {
        let db_pool = setup_test_db().await;
        
        let result = get_orders(State(db_pool)).await;
        assert!(result.is_ok());
        let orders = result.unwrap().0;
        assert_eq!(orders.len(), 0);
    }

    #[tokio::test]
    async fn test_get_orders_with_data() {
        let db_pool = setup_test_db().await;
        
        // Add some test orders
        let _order1 = create_test_order(&db_pool).await;
        
        let order2 = Order {
            id: 2,
            item: "Another Item".to_string(),
            status: "shipped".to_string(),
            quantity: 10,
        };
        let _result2 = add_order(State(db_pool.clone()), Json(order2)).await.unwrap();
        
        let result = get_orders(State(db_pool)).await;
        assert!(result.is_ok());
        let orders = result.unwrap().0;
        assert_eq!(orders.len(), 2);
    }

    #[tokio::test]
    async fn test_add_order_success() {
        let db_pool = setup_test_db().await;
        
        let new_order = Order {
            id: 1,
            item: "Test Item".to_string(),
            status: "pending".to_string(),
            quantity: 5,
        };

        let result = add_order(State(db_pool.clone()), Json(new_order.clone())).await;
        assert!(result.is_ok());
        let created_order = result.unwrap().0;
        assert_eq!(created_order.id, new_order.id);
        assert_eq!(created_order.item, new_order.item);
        assert_eq!(created_order.status, new_order.status);
        assert_eq!(created_order.quantity, new_order.quantity);
        
        // Verify it was actually added to the database
        let orders_result = get_orders(State(db_pool)).await;
        assert!(orders_result.is_ok());
        let orders = orders_result.unwrap().0;
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].id, new_order.id);
    }

    #[tokio::test]
    async fn test_add_order_duplicate_id() {
        let db_pool = setup_test_db().await;
        
        let order1 = Order {
            id: 1,
            item: "First Item".to_string(),
            status: "pending".to_string(),
            quantity: 5,
        };

        let order2 = Order {
            id: 1, // Same ID
            item: "Second Item".to_string(),
            status: "processing".to_string(),
            quantity: 3,
        };

        // Add first order - should succeed
        let result1 = add_order(State(db_pool.clone()), Json(order1)).await;
        assert!(result1.is_ok());

        // Add second order with same ID - should fail
        let result2 = add_order(State(db_pool), Json(order2)).await;
        assert!(result2.is_err());
    }

    #[tokio::test]
    async fn test_add_order_validation_empty_item() {
        let db_pool = setup_test_db().await;
        
        let invalid_order = Order {
            id: 1,
            item: "".to_string(),
            status: "pending".to_string(),
            quantity: 5,
        };

        let result = add_order(State(db_pool), Json(invalid_order)).await;
        assert!(result.is_err());
        
        if let Err(ApiError::Validation(error)) = result {
            assert_eq!(error.error, "Item name cannot be empty");
            assert_eq!(error.field, Some("item".to_string()));
        } else {
            panic!("Expected validation error");
        }
    }

    #[tokio::test]
    async fn test_add_order_validation_invalid_status() {
        let db_pool = setup_test_db().await;
        
        let invalid_order = Order {
            id: 1,
            item: "Test Item".to_string(),
            status: "invalid_status".to_string(),
            quantity: 5,
        };

        let result = add_order(State(db_pool), Json(invalid_order)).await;
        assert!(result.is_err());
        
        if let Err(ApiError::Validation(error)) = result {
            assert!(error.error.contains("Status must be one of:"));
            assert_eq!(error.field, Some("status".to_string()));
        } else {
            panic!("Expected validation error");
        }
    }

    #[tokio::test]
    async fn test_add_order_validation_zero_quantity() {
        let db_pool = setup_test_db().await;
        
        let invalid_order = Order {
            id: 1,
            item: "Test Item".to_string(),
            status: "pending".to_string(),
            quantity: 0,
        };

        let result = add_order(State(db_pool), Json(invalid_order)).await;
        assert!(result.is_err());
        
        if let Err(ApiError::Validation(error)) = result {
            assert_eq!(error.error, "Quantity must be greater than 0");
            assert_eq!(error.field, Some("quantity".to_string()));
        } else {
            panic!("Expected validation error");
        }
    }

    #[tokio::test]
    async fn test_get_order_by_id_success() {
        let db_pool = setup_test_db().await;
        
        let created_order = create_test_order(&db_pool).await;
        
        let result = get_order_by_id(State(db_pool), Path(1)).await;
        assert!(result.is_ok());
        let order = result.unwrap().0;
        assert_eq!(order.id, created_order.id);
        assert_eq!(order.item, created_order.item);
        assert_eq!(order.status, created_order.status);
        assert_eq!(order.quantity, created_order.quantity);
    }

    #[tokio::test]
    async fn test_get_order_by_id_not_found() {
        let db_pool = setup_test_db().await;
        
        let result = get_order_by_id(State(db_pool), Path(999)).await;
        assert!(result.is_err());
        
        if let Err(ApiError::NotFound(message)) = result {
            assert_eq!(message, "Order not found");
        } else {
            panic!("Expected NotFound error");
        }
    }

    #[tokio::test]
    async fn test_update_order_by_id_success() {
        let db_pool = setup_test_db().await;
        
        let _created_order = create_test_order(&db_pool).await;
        
        let updated_order = Order {
            id: 1,
            item: "Updated Item".to_string(),
            status: "shipped".to_string(),
            quantity: 10,
        };

        let result = update_order_by_id(State(db_pool), Path(1), Json(updated_order.clone())).await;
        assert!(result.is_ok());
        let order = result.unwrap().0;
        assert_eq!(order.item, updated_order.item);
        assert_eq!(order.status, updated_order.status);
        assert_eq!(order.quantity, updated_order.quantity);
    }

    #[tokio::test]
    async fn test_update_order_by_id_not_found() {
        let db_pool = setup_test_db().await;
        
        let updated_order = Order {
            id: 999,
            item: "Updated Item".to_string(),
            status: "shipped".to_string(),
            quantity: 10,
        };

        let result = update_order_by_id(State(db_pool), Path(999), Json(updated_order)).await;
        assert!(result.is_err());
        
        if let Err(ApiError::NotFound(message)) = result {
            assert_eq!(message, "Order not found");
        } else {
            panic!("Expected NotFound error");
        }
    }

    #[tokio::test]
    async fn test_update_order_by_id_validation_error() {
        let db_pool = setup_test_db().await;
        
        let _created_order = create_test_order(&db_pool).await;
        
        let invalid_updated_order = Order {
            id: 1,
            item: "".to_string(), // Invalid empty item
            status: "shipped".to_string(),
            quantity: 10,
        };

        let result = update_order_by_id(State(db_pool), Path(1), Json(invalid_updated_order)).await;
        assert!(result.is_err());
        
        if let Err(ApiError::Validation(error)) = result {
            assert_eq!(error.error, "Item name cannot be empty");
        } else {
            panic!("Expected validation error");
        }
    }

    #[tokio::test]
    async fn test_update_order_status_success() {
        let db_pool = setup_test_db().await;
        
        let _created_order = create_test_order(&db_pool).await;
        
        let status_update = StatusUpdate {
            status: "shipped".to_string(),
        };

        let result = update_order_status(State(db_pool), Path(1), Json(status_update)).await;
        assert!(result.is_ok());
        let order = result.unwrap().0;
        assert_eq!(order.status, "shipped");
        assert_eq!(order.id, 1);
        assert_eq!(order.item, "Test Item"); // Other fields unchanged
        assert_eq!(order.quantity, 5);
    }

    #[tokio::test]
    async fn test_update_order_status_not_found() {
        let db_pool = setup_test_db().await;
        
        let status_update = StatusUpdate {
            status: "shipped".to_string(),
        };

        let result = update_order_status(State(db_pool), Path(999), Json(status_update)).await;
        assert!(result.is_err());
        
        if let Err(ApiError::NotFound(message)) = result {
            assert_eq!(message, "Order not found");
        } else {
            panic!("Expected NotFound error");
        }
    }

    #[tokio::test]
    async fn test_update_order_status_validation_error() {
        let db_pool = setup_test_db().await;
        
        let _created_order = create_test_order(&db_pool).await;
        
        let invalid_status_update = StatusUpdate {
            status: "invalid_status".to_string(),
        };

        let result = update_order_status(State(db_pool), Path(1), Json(invalid_status_update)).await;
        assert!(result.is_err());
        
        if let Err(ApiError::Validation(error)) = result {
            assert!(error.error.contains("Status must be one of:"));
        } else {
            panic!("Expected validation error");
        }
    }

    #[tokio::test]
    async fn test_delete_order_by_id_success() {
        let db_pool = setup_test_db().await;
        
        let created_order = create_test_order(&db_pool).await;
        
        let result = delete_order_by_id(State(db_pool.clone()), Path(1)).await;
        assert!(result.is_ok());
        let deleted_order = result.unwrap().0;
        assert_eq!(deleted_order.id, created_order.id);
        assert_eq!(deleted_order.item, created_order.item);
        
        // Verify it was deleted from the database
        let orders_result = get_orders(State(db_pool)).await;
        assert!(orders_result.is_ok());
        let orders = orders_result.unwrap().0;
        assert_eq!(orders.len(), 0);
    }

    #[tokio::test]
    async fn test_delete_order_by_id_not_found() {
        let db_pool = setup_test_db().await;
        
        let result = delete_order_by_id(State(db_pool), Path(999)).await;
        assert!(result.is_err());
        
        if let Err(ApiError::NotFound(message)) = result {
            assert_eq!(message, "Order not found");
        } else {
            panic!("Expected NotFound error");
        }
    }

    #[tokio::test]
    async fn test_status_update_struct() {
        let status_update = StatusUpdate {
            status: "processing".to_string(),
        };
        
        // Test serialization
        let serialized = serde_json::to_string(&status_update).unwrap();
        assert!(serialized.contains("\"status\":\"processing\""));
        
        // Test deserialization
        let json_str = r#"{"status":"delivered"}"#;
        let deserialized: StatusUpdate = serde_json::from_str(json_str).unwrap();
        assert_eq!(deserialized.status, "delivered");
    }

    #[tokio::test]
    async fn test_multiple_operations_sequence() {
        let db_pool = setup_test_db().await;
        
        // 1. Add an order
        let new_order = Order {
            id: 1,
            item: "Sequential Test Item".to_string(),
            status: "pending".to_string(),
            quantity: 5,
        };
        let add_result = add_order(State(db_pool.clone()), Json(new_order.clone())).await;
        assert!(add_result.is_ok());
        
        // 2. Get the order
        let get_result = get_order_by_id(State(db_pool.clone()), Path(1)).await;
        assert!(get_result.is_ok());
        let retrieved_order = get_result.unwrap().0;
        assert_eq!(retrieved_order.item, new_order.item);
        
        // 3. Update the order status
        let status_update = StatusUpdate {
            status: "processing".to_string(),
        };
        let status_result = update_order_status(State(db_pool.clone()), Path(1), Json(status_update)).await;
        assert!(status_result.is_ok());
        let updated_order = status_result.unwrap().0;
        assert_eq!(updated_order.status, "processing");
        
        // 4. Update the entire order
        let full_update = Order {
            id: 1,
            item: "Fully Updated Item".to_string(),
            status: "shipped".to_string(),
            quantity: 15,
        };
        let full_update_result = update_order_by_id(State(db_pool.clone()), Path(1), Json(full_update.clone())).await;
        assert!(full_update_result.is_ok());
        let final_order = full_update_result.unwrap().0;
        assert_eq!(final_order.item, full_update.item);
        assert_eq!(final_order.status, full_update.status);
        assert_eq!(final_order.quantity, full_update.quantity);
        
        // 5. Delete the order
        let delete_result = delete_order_by_id(State(db_pool.clone()), Path(1)).await;
        assert!(delete_result.is_ok());
        
        // 6. Verify it's gone
        let final_get_result = get_order_by_id(State(db_pool), Path(1)).await;
        assert!(final_get_result.is_err());
    }
}
