#[cfg(test)]
mod tests {
    use crate::utils::{init_db, Order};
    use crate::routes::create_router;
    use axum_test::TestServer;
    use axum::http::StatusCode;
    use serde_json::{json, Value};
    use tokio;

    async fn setup_test_server() -> TestServer {
        let db_pool = init_db().await.expect("Failed to initialize test database");
        let app = create_router(db_pool);
        TestServer::new(app).unwrap()
    }

    async fn add_test_order(server: &TestServer, id: u32, item: &str, status: &str, quantity: u32) -> Order {
        let new_order = json!({
            "id": id,
            "item": item,
            "status": status,
            "quantity": quantity
        });

        let response = server.post("/orders").json(&new_order).await;
        response.assert_status_ok();
        response.json()
    }

    #[tokio::test]
    async fn test_get_orders_empty() {
        let server = setup_test_server().await;
        
        let response = server.get("/orders").await;
        response.assert_status_ok();
        
        let orders: Vec<Order> = response.json();
        assert_eq!(orders.len(), 0);
    }

    #[tokio::test]
    async fn test_get_orders_with_data() {
        let server = setup_test_server().await;
        
        // Add some test orders
        add_test_order(&server, 1, "First Item", "pending", 5).await;
        add_test_order(&server, 2, "Second Item", "shipped", 10).await;
        add_test_order(&server, 3, "Third Item", "delivered", 3).await;
        
        let response = server.get("/orders").await;
        response.assert_status_ok();
        
        let orders: Vec<Order> = response.json();
        assert_eq!(orders.len(), 3);
        
        // Verify the orders are present (order might vary)
        let items: Vec<&str> = orders.iter().map(|o| o.item.as_str()).collect();
        assert!(items.contains(&"First Item"));
        assert!(items.contains(&"Second Item"));
        assert!(items.contains(&"Third Item"));
    }

    #[tokio::test]
    async fn test_add_order_valid() {
        let server = setup_test_server().await;
        
        let new_order = json!({
            "id": 1,
            "item": "Test Item",
            "status": "pending",
            "quantity": 5
        });

        let response = server.post("/orders").json(&new_order).await;
        response.assert_status_ok();
        
        let order: Order = response.json();
        assert_eq!(order.id, 1);
        assert_eq!(order.item, "Test Item");
        assert_eq!(order.status, "pending");
        assert_eq!(order.quantity, 5);
    }

    #[tokio::test]
    async fn test_add_order_all_valid_statuses() {
        let server = setup_test_server().await;
        let valid_statuses = ["pending", "processing", "shipped", "delivered", "cancelled"];
        
        for (i, status) in valid_statuses.iter().enumerate() {
            let new_order = json!({
                "id": i + 1,
                "item": format!("Test Item {}", i + 1),
                "status": status,
                "quantity": 5
            });

            let response = server.post("/orders").json(&new_order).await;
            response.assert_status_ok();
            
            let order: Order = response.json();
            assert_eq!(order.status, *status);
        }
    }

    #[tokio::test]
    async fn test_add_order_invalid_empty_item() {
        let server = setup_test_server().await;
        
        let invalid_order = json!({
            "id": 1,
            "item": "",
            "status": "pending",
            "quantity": 5
        });

        let response = server.post("/orders").json(&invalid_order).await;
        response.assert_status(StatusCode::BAD_REQUEST);
        
        let error_body: Value = response.json();
        assert!(error_body["error"].as_str().unwrap().contains("Item name cannot be empty"));
    }

    #[tokio::test]
    async fn test_add_order_invalid_status() {
        let server = setup_test_server().await;
        
        let invalid_order = json!({
            "id": 1,
            "item": "Test Item",
            "status": "invalid_status",
            "quantity": 5
        });

        let response = server.post("/orders").json(&invalid_order).await;
        response.assert_status(StatusCode::BAD_REQUEST);
        
        let error_body: Value = response.json();
        assert!(error_body["error"].as_str().unwrap().contains("Status must be one of:"));
    }

    #[tokio::test]
    async fn test_add_order_zero_quantity() {
        let server = setup_test_server().await;
        
        let invalid_order = json!({
            "id": 1,
            "item": "Test Item",
            "status": "pending",
            "quantity": 0
        });

        let response = server.post("/orders").json(&invalid_order).await;
        response.assert_status(StatusCode::BAD_REQUEST);
        
        let error_body: Value = response.json();
        assert!(error_body["error"].as_str().unwrap().contains("Quantity must be greater than 0"));
    }

    #[tokio::test]
    async fn test_add_order_excessive_quantity() {
        let server = setup_test_server().await;
        
        let invalid_order = json!({
            "id": 1,
            "item": "Test Item",
            "status": "pending",
            "quantity": 1001
        });

        let response = server.post("/orders").json(&invalid_order).await;
        response.assert_status(StatusCode::BAD_REQUEST);
        
        let error_body: Value = response.json();
        assert!(error_body["error"].as_str().unwrap().contains("Quantity cannot exceed 1000"));
    }

    #[tokio::test]
    async fn test_add_order_duplicate_id() {
        let server = setup_test_server().await;
        
        let order1 = json!({
            "id": 1,
            "item": "First Item",
            "status": "pending",
            "quantity": 5
        });

        let order2 = json!({
            "id": 1, // Same ID
            "item": "Second Item",
            "status": "processing",
            "quantity": 3
        });

        // Add first order - should succeed
        let response1 = server.post("/orders").json(&order1).await;
        response1.assert_status_ok();

        // Add second order with same ID - should fail
        let response2 = server.post("/orders").json(&order2).await;
        response2.assert_status(StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_get_order_by_id_success() {
        let server = setup_test_server().await;
        
        // Add an order first
        add_test_order(&server, 1, "Test Item", "pending", 5).await;

        // Get the order by ID
        let response = server.get("/orders/1").await;
        response.assert_status_ok();
        
        let order: Order = response.json();
        assert_eq!(order.id, 1);
        assert_eq!(order.item, "Test Item");
        assert_eq!(order.status, "pending");
        assert_eq!(order.quantity, 5);
    }

    #[tokio::test]
    async fn test_get_order_by_id_not_found() {
        let server = setup_test_server().await;
        
        let response = server.get("/orders/999").await;
        response.assert_status(StatusCode::NOT_FOUND);
        
        let error_body: Value = response.json();
        assert_eq!(error_body["error"], "Order not found");
    }

    #[tokio::test]
    async fn test_update_order_success() {
        let server = setup_test_server().await;
        
        // Add an order first
        add_test_order(&server, 1, "Original Item", "pending", 5).await;

        // Update the order
        let updated_order = json!({
            "id": 1,
            "item": "Updated Item",
            "status": "shipped",
            "quantity": 10
        });

        let response = server.put("/orders/1").json(&updated_order).await;
        response.assert_status_ok();
        
        let order: Order = response.json();
        assert_eq!(order.id, 1);
        assert_eq!(order.item, "Updated Item");
        assert_eq!(order.status, "shipped");
        assert_eq!(order.quantity, 10);
    }

    #[tokio::test]
    async fn test_update_order_not_found() {
        let server = setup_test_server().await;
        
        let updated_order = json!({
            "id": 999,
            "item": "Updated Item",
            "status": "shipped",
            "quantity": 10
        });

        let response = server.put("/orders/999").json(&updated_order).await;
        response.assert_status(StatusCode::NOT_FOUND);
        
        let error_body: Value = response.json();
        assert_eq!(error_body["error"], "Order not found");
    }

    #[tokio::test]
    async fn test_update_order_validation_error() {
        let server = setup_test_server().await;
        
        // Add an order first
        add_test_order(&server, 1, "Original Item", "pending", 5).await;

        // Try to update with invalid data
        let invalid_updated_order = json!({
            "id": 1,
            "item": "", // Invalid empty item
            "status": "shipped",
            "quantity": 10
        });

        let response = server.put("/orders/1").json(&invalid_updated_order).await;
        response.assert_status(StatusCode::BAD_REQUEST);
        
        let error_body: Value = response.json();
        assert!(error_body["error"].as_str().unwrap().contains("Item name cannot be empty"));
    }

    #[tokio::test]
    async fn test_update_order_status_success() {
        let server = setup_test_server().await;
        
        // Add an order first
        add_test_order(&server, 1, "Test Item", "pending", 5).await;

        // Update order status
        let status_update = json!({
            "status": "shipped"
        });

        let response = server.patch("/orders/1/status").json(&status_update).await;
        response.assert_status_ok();
        
        let order: Order = response.json();
        assert_eq!(order.id, 1);
        assert_eq!(order.status, "shipped");
        assert_eq!(order.item, "Test Item"); // Other fields unchanged
        assert_eq!(order.quantity, 5);
    }

    #[tokio::test]
    async fn test_update_order_status_not_found() {
        let server = setup_test_server().await;
        
        let status_update = json!({
            "status": "shipped"
        });

        let response = server.patch("/orders/999/status").json(&status_update).await;
        response.assert_status(StatusCode::NOT_FOUND);
        
        let error_body: Value = response.json();
        assert_eq!(error_body["error"], "Order not found");
    }

    #[tokio::test]
    async fn test_update_order_status_validation_error() {
        let server = setup_test_server().await;
        
        // Add an order first
        add_test_order(&server, 1, "Test Item", "pending", 5).await;

        // Try to update with invalid status
        let invalid_status_update = json!({
            "status": "invalid_status"
        });

        let response = server.patch("/orders/1/status").json(&invalid_status_update).await;
        response.assert_status(StatusCode::BAD_REQUEST);
        
        let error_body: Value = response.json();
        assert!(error_body["error"].as_str().unwrap().contains("Status must be one of:"));
    }

    #[tokio::test]
    async fn test_delete_order_success() {
        let server = setup_test_server().await;
        
        // Add an order first
        let created_order = add_test_order(&server, 1, "Test Item", "pending", 5).await;

        // Delete the order
        let response = server.delete("/orders/1").await;
        response.assert_status_ok();
        
        let deleted_order: Order = response.json();
        assert_eq!(deleted_order.id, created_order.id);
        assert_eq!(deleted_order.item, created_order.item);

        // Verify it's gone
        let get_response = server.get("/orders/1").await;
        get_response.assert_status(StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_delete_order_not_found() {
        let server = setup_test_server().await;
        
        let response = server.delete("/orders/999").await;
        response.assert_status(StatusCode::NOT_FOUND);
        
        let error_body: Value = response.json();
        assert_eq!(error_body["error"], "Order not found");
    }

    #[tokio::test]
    async fn test_catch_all_route() {
        let server = setup_test_server().await;
        
        let response = server.get("/nonexistent").await;
        response.assert_status(StatusCode::NOT_FOUND);
        
        let error_body: Value = response.json();
        assert_eq!(error_body["error"], "Path not found");
    }

    #[tokio::test]
    async fn test_catch_all_route_different_paths() {
        let server = setup_test_server().await;
        
        let invalid_paths = [
            "/invalid",
            "/random/path", 
            "/api/v1/orders",
            "/orders/1/invalid"
        ];
        
        for path in invalid_paths.iter() {
            let response = server.get(path).await;
            response.assert_status(StatusCode::NOT_FOUND);
            
            let error_body: Value = response.json();
            assert_eq!(error_body["error"], "Path not found");
        }
    }

    #[tokio::test]
    async fn test_invalid_id_format() {
        let server = setup_test_server().await;
        
        // Test invalid ID formats that return 400 instead of 404
        let invalid_id_paths = [
            "/orders/invalid",
            "/orders/abc", 
            "/orders/123abc",
            "/orders/-1"
        ];
        
        for path in invalid_id_paths.iter() {
            let response = server.get(path).await;
            response.assert_status(StatusCode::BAD_REQUEST);
        }
    }

    #[tokio::test]
    async fn test_malformed_json() {
        let server = setup_test_server().await;
        
        let response = server
            .post("/orders")
            .text("{ invalid json }")
            .await;
        
        response.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_missing_fields() {
        let server = setup_test_server().await;
        
        let incomplete_order = json!({
            "id": 1,
            "item": "Test Item"
            // Missing status and quantity
        });

        let response = server.post("/orders").json(&incomplete_order).await;
        response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_wrong_data_types() {
        let server = setup_test_server().await;
        
        let wrong_types_order = json!({
            "id": "not_a_number", // Should be u32
            "item": "Test Item",
            "status": "pending",
            "quantity": "also_not_a_number" // Should be u32
        });

        let response = server.post("/orders").json(&wrong_types_order).await;
        response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_content_type_headers() {
        let server = setup_test_server().await;
        
        let new_order = json!({
            "id": 1,
            "item": "Test Item",
            "status": "pending",
            "quantity": 5
        });

        let response = server.post("/orders").json(&new_order).await;
        response.assert_status_ok();
        
        // Check that response is JSON
        let content_type = response.headers().get("content-type").unwrap();
        assert!(content_type.to_str().unwrap().contains("application/json"));
    }

    #[tokio::test]
    async fn test_integration_workflow() {
        let server = setup_test_server().await;
        
        // 1. Start with empty orders list
        let response = server.get("/orders").await;
        response.assert_status_ok();
        let orders: Vec<Order> = response.json();
        assert_eq!(orders.len(), 0);
        
        // 2. Add multiple orders
        add_test_order(&server, 1, "First Order", "pending", 5).await;
        add_test_order(&server, 2, "Second Order", "processing", 10).await;
        add_test_order(&server, 3, "Third Order", "shipped", 3).await;
        
        // 3. Verify all orders are present
        let response = server.get("/orders").await;
        response.assert_status_ok();
        let orders: Vec<Order> = response.json();
        assert_eq!(orders.len(), 3);
        
        // 4. Update an order status
        let status_update = json!({"status": "delivered"});
        let response = server.patch("/orders/2/status").json(&status_update).await;
        response.assert_status_ok();
        let updated_order: Order = response.json();
        assert_eq!(updated_order.status, "delivered");
        
        // 5. Update a full order
        let full_update = json!({
            "id": 1,
            "item": "Updated First Order",
            "status": "delivered",
            "quantity": 15
        });
        let response = server.put("/orders/1").json(&full_update).await;
        response.assert_status_ok();
        let updated_order: Order = response.json();
        assert_eq!(updated_order.item, "Updated First Order");
        assert_eq!(updated_order.quantity, 15);
        
        // 6. Delete an order
        let response = server.delete("/orders/3").await;
        response.assert_status_ok();
        
        // 7. Verify final state
        let response = server.get("/orders").await;
        response.assert_status_ok();
        let final_orders: Vec<Order> = response.json();
        assert_eq!(final_orders.len(), 2);
        
        // 8. Verify deleted order is gone
        let response = server.get("/orders/3").await;
        response.assert_status(StatusCode::NOT_FOUND);
    }
}
