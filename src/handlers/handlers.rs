use axum::{
    extract::{Path, State},
    Json
};
use serde::{Deserialize, Serialize};
use utoipa;
use crate::validators::{validate_order, validate_status, ApiError};
use crate::utils::{DbPool, Order, get_all_orders, get_order_by_id as db_get_order_by_id, 
                   create_order, update_order, update_order_status as db_update_order_status, 
                   delete_order};

#[derive(Debug, Deserialize, Serialize, utoipa::ToSchema)]
/// Status update request body
pub struct StatusUpdate {
    /// New status for the order (pending, processing, shipped, delivered, cancelled)
    pub status: String,
}

#[utoipa::path(
    get,
    path = "/orders",
    responses(
        (status = 200, description = "List of all orders", body = [Order]),
        (status = 500, description = "Internal server error")
    ),
    tag = "orders"
)]
#[axum::debug_handler]
pub async fn get_orders(State(db_pool): State<DbPool>) -> Result<Json<Vec<Order>>, ApiError> {
    let orders = get_all_orders(&db_pool).await?;
    Ok(Json(orders))
}

#[utoipa::path(
    post,
    path = "/orders",
    request_body = Order,
    responses(
        (status = 201, description = "Order created successfully", body = Order),
        (status = 400, description = "Invalid input"),
        (status = 409, description = "Order with ID already exists"),
        (status = 500, description = "Internal server error")
    ),
    tag = "orders"
)]
#[axum::debug_handler]
pub async fn add_order(State(db_pool): State<DbPool>, Json(new_order): Json<Order>) -> Result<Json<Order>, ApiError> {
    // Validate the order first
    validate_order(&new_order)?;
    
    // Create the order in the database (includes duplicate ID check)
    let created_order = create_order(&db_pool, &new_order).await?;
    Ok(Json(created_order))
}

#[utoipa::path(
    get,
    path = "/orders/{id}",
    params(
        ("id" = u32, Path, description = "Order ID")
    ),
    responses(
        (status = 200, description = "Order found", body = Order),
        (status = 404, description = "Order not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "orders"
)]
#[axum::debug_handler]
pub async fn get_order_by_id(
    State(db_pool): State<DbPool>,
    Path(id): Path<u32>,
) -> Result<Json<Order>, ApiError> {
    let order = db_get_order_by_id(&db_pool, id).await?
        .ok_or_else(|| ApiError::NotFound("Order not found".to_string()))?;
    Ok(Json(order))
}

#[utoipa::path(
    put,
    path = "/orders/{id}",
    params(
        ("id" = u32, Path, description = "Order ID")
    ),
    request_body = Order,
    responses(
        (status = 200, description = "Order updated successfully", body = Order),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "Order not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "orders"
)]
#[axum::debug_handler]
pub async fn update_order_by_id(
    State(db_pool): State<DbPool>,
    Path(id): Path<u32>,
    Json(updated_order): Json<Order>,
) -> Result<Json<Order>, ApiError> {
    // Validate the updated order
    validate_order(&updated_order)?;
    
    let updated = update_order(&db_pool, id, &updated_order).await?;
    Ok(Json(updated))
}

#[utoipa::path(
    patch,
    path = "/orders/{id}/status",
    params(
        ("id" = u32, Path, description = "Order ID")
    ),
    request_body = StatusUpdate,
    responses(
        (status = 200, description = "Order status updated successfully", body = Order),
        (status = 400, description = "Invalid status"),
        (status = 404, description = "Order not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "orders"
)]
#[axum::debug_handler]
pub async fn update_order_status(
    State(db_pool): State<DbPool>,
    Path(id): Path<u32>,
    Json(status_update): Json<StatusUpdate>,
) -> Result<Json<Order>, ApiError> {
    // Validate the status
    validate_status(&status_update.status)?;
    
    let updated = db_update_order_status(&db_pool, id, &status_update.status).await?;
    Ok(Json(updated))
}

#[utoipa::path(
    delete,
    path = "/orders/{id}",
    params(
        ("id" = u32, Path, description = "Order ID")
    ),
    responses(
        (status = 200, description = "Order deleted successfully", body = Order),
        (status = 404, description = "Order not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "orders"
)]
pub async fn delete_order_by_id(
    State(db_pool): State<DbPool>,
    Path(id): Path<u32>,
) -> Result<Json<Order>, ApiError> {
    let deleted_order = delete_order(&db_pool, id).await?;
    Ok(Json(deleted_order))
}
