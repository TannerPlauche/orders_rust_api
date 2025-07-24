use sqlx::{Pool, Sqlite, SqlitePool};
use serde::{Deserialize, Serialize};
use crate::validators::{ApiError, ServerError};

// Database configuration  
const DATABASE_URL: &str = "sqlite::memory:";

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, utoipa::ToSchema)]
/// Order structure representing a customer order
pub struct Order {
    /// Unique identifier for the order
    pub id: u32,
    /// Name of the item being ordered
    pub item: String,
    /// Current status of the order (pending, processing, shipped, delivered, cancelled)
    pub status: String,
    /// Quantity of items ordered
    pub quantity: u32,
}

pub type DbPool = Pool<Sqlite>;

/// Initialize the database connection pool and create tables
pub async fn init_db() -> Result<DbPool, sqlx::Error> {
    // Create the database file if it doesn't exist
    let pool = SqlitePool::connect(DATABASE_URL).await?;
    
    // Create the orders table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS orders (
            id INTEGER PRIMARY KEY,
            item TEXT NOT NULL,
            status TEXT NOT NULL,
            quantity INTEGER NOT NULL
        )
        "#
    )
    .execute(&pool)
    .await?;
    
    println!("Database initialized successfully");
    Ok(pool)
}

/// Get all orders from the database
pub async fn get_all_orders(pool: &DbPool) -> Result<Vec<Order>, ApiError> {
    let orders = sqlx::query_as::<_, Order>("SELECT id, item, status, quantity FROM orders")
        .fetch_all(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error in get_all_orders: {}", e);
            ApiError::Server(ServerError {
                error: "Database error".to_string(),
                message: "Failed to retrieve orders".to_string(),
            })
        })?;
    
    Ok(orders)
}

/// Get a specific order by ID
pub async fn get_order_by_id(pool: &DbPool, order_id: u32) -> Result<Option<Order>, ApiError> {
    let order = sqlx::query_as::<_, Order>("SELECT id, item, status, quantity FROM orders WHERE id = ?")
        .bind(order_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error in get_order_by_id: {}", e);
            ApiError::Server(ServerError {
                error: "Database error".to_string(),
                message: "Failed to retrieve order".to_string(),
            })
        })?;
    
    Ok(order)
}

/// Create a new order in the database
pub async fn create_order(pool: &DbPool, order: &Order) -> Result<Order, ApiError> {
    // Check if order with this ID already exists
    if let Some(_) = get_order_by_id(pool, order.id).await? {
        return Err(ApiError::Validation(crate::validators::ValidationError {
            error: format!("Order with ID {} already exists", order.id),
            field: Some("id".to_string()),
        }));
    }
    
    sqlx::query("INSERT INTO orders (id, item, status, quantity) VALUES (?, ?, ?, ?)")
        .bind(order.id)
        .bind(&order.item)
        .bind(&order.status)
        .bind(order.quantity)
        .execute(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error in create_order: {}", e);
            ApiError::Server(ServerError {
                error: "Database error".to_string(),
                message: "Failed to create order".to_string(),
            })
        })?;
    
    Ok(order.clone())
}

/// Update an existing order in the database
pub async fn update_order(pool: &DbPool, order_id: u32, order: &Order) -> Result<Order, ApiError> {
    let result = sqlx::query("UPDATE orders SET item = ?, status = ?, quantity = ? WHERE id = ?")
        .bind(&order.item)
        .bind(&order.status)
        .bind(order.quantity)
        .bind(order_id)
        .execute(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error in update_order: {}", e);
            ApiError::Server(ServerError {
                error: "Database error".to_string(),
                message: "Failed to update order".to_string(),
            })
        })?;
    
    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Order not found".to_string()));
    }
    
    // Return the updated order
    let mut updated_order = order.clone();
    updated_order.id = order_id;
    Ok(updated_order)
}

/// Update only the status of an order
pub async fn update_order_status(pool: &DbPool, order_id: u32, status: &str) -> Result<Order, ApiError> {
    let result = sqlx::query("UPDATE orders SET status = ? WHERE id = ?")
        .bind(status)
        .bind(order_id)
        .execute(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error in update_order_status: {}", e);
            ApiError::Server(ServerError {
                error: "Database error".to_string(),
                message: "Failed to update order status".to_string(),
            })
        })?;
    
    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Order not found".to_string()));
    }
    
    // Get and return the updated order
    get_order_by_id(pool, order_id).await?
        .ok_or_else(|| ApiError::NotFound("Order not found".to_string()))
}

/// Delete an order from the database
pub async fn delete_order(pool: &DbPool, order_id: u32) -> Result<Order, ApiError> {
    // First, get the order to return it
    let order = get_order_by_id(pool, order_id).await?
        .ok_or_else(|| ApiError::NotFound("Order not found".to_string()))?;
    
    let result = sqlx::query("DELETE FROM orders WHERE id = ?")
        .bind(order_id)
        .execute(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error in delete_order: {}", e);
            ApiError::Server(ServerError {
                error: "Database error".to_string(),
                message: "Failed to delete order".to_string(),
            })
        })?;
    
    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Order not found".to_string()));
    }
    
    Ok(order)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;
    
    async fn setup_test_db() -> DbPool {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        
        sqlx::query(
            r#"
            CREATE TABLE orders (
                id INTEGER PRIMARY KEY,
                item TEXT NOT NULL,
                status TEXT NOT NULL,
                quantity INTEGER NOT NULL
            )
            "#
        )
        .execute(&pool)
        .await
        .unwrap();
        
        pool
    }
    
    #[tokio::test]
    async fn test_create_and_get_order() {
        let pool = setup_test_db().await;
        
        let order = Order {
            id: 1,
            item: "Test Item".to_string(),
            status: "pending".to_string(),
            quantity: 5,
        };
        
        // Create order
        let created = create_order(&pool, &order).await.unwrap();
        assert_eq!(created.id, 1);
        assert_eq!(created.item, "Test Item");
        
        // Get order by ID
        let retrieved = get_order_by_id(&pool, 1).await.unwrap().unwrap();
        assert_eq!(retrieved.id, 1);
        assert_eq!(retrieved.item, "Test Item");
        assert_eq!(retrieved.status, "pending");
        assert_eq!(retrieved.quantity, 5);
    }
    
    #[tokio::test]
    async fn test_get_all_orders() {
        let pool = setup_test_db().await;
        
        let orders = vec![
            Order { id: 1, item: "Item 1".to_string(), status: "pending".to_string(), quantity: 1 },
            Order { id: 2, item: "Item 2".to_string(), status: "processing".to_string(), quantity: 2 },
        ];
        
        for order in &orders {
            create_order(&pool, order).await.unwrap();
        }
        
        let all_orders = get_all_orders(&pool).await.unwrap();
        assert_eq!(all_orders.len(), 2);
    }
    
    #[tokio::test]
    async fn test_update_order() {
        let pool = setup_test_db().await;
        
        let order = Order {
            id: 1,
            item: "Original Item".to_string(),
            status: "pending".to_string(),
            quantity: 1,
        };
        
        create_order(&pool, &order).await.unwrap();
        
        let updated_order = Order {
            id: 1, // This will be ignored in update
            item: "Updated Item".to_string(),
            status: "processing".to_string(),
            quantity: 2,
        };
        
        let result = update_order(&pool, 1, &updated_order).await.unwrap();
        assert_eq!(result.item, "Updated Item");
        assert_eq!(result.status, "processing");
        assert_eq!(result.quantity, 2);
    }
    
    #[tokio::test]
    async fn test_update_order_status() {
        let pool = setup_test_db().await;
        
        let order = Order {
            id: 1,
            item: "Test Item".to_string(),
            status: "pending".to_string(),
            quantity: 1,
        };
        
        create_order(&pool, &order).await.unwrap();
        
        let updated = update_order_status(&pool, 1, "shipped").await.unwrap();
        assert_eq!(updated.status, "shipped");
        assert_eq!(updated.item, "Test Item"); // Other fields unchanged
    }
    
    #[tokio::test]
    async fn test_delete_order() {
        let pool = setup_test_db().await;
        
        let order = Order {
            id: 1,
            item: "Test Item".to_string(),
            status: "pending".to_string(),
            quantity: 1,
        };
        
        create_order(&pool, &order).await.unwrap();
        
        let deleted = delete_order(&pool, 1).await.unwrap();
        assert_eq!(deleted.id, 1);
        
        // Verify it's deleted
        let result = get_order_by_id(&pool, 1).await.unwrap();
        assert!(result.is_none());
    }
    
    #[tokio::test]
    async fn test_duplicate_id_error() {
        let pool = setup_test_db().await;
        
        let order = Order {
            id: 1,
            item: "Test Item".to_string(),
            status: "pending".to_string(),
            quantity: 1,
        };
        
        create_order(&pool, &order).await.unwrap();
        
        // Try to create another order with the same ID
        let result = create_order(&pool, &order).await;
        assert!(result.is_err());
        
        match result.unwrap_err() {
            ApiError::Validation(err) => {
                assert!(err.error.contains("already exists"));
                assert_eq!(err.field, Some("id".to_string()));
            },
            _ => panic!("Expected validation error"),
        }
    }
}
