use axum::{
    extract::{Path, State},
    Json
};
use serde::{Deserialize, Serialize};
use crate::validators::{validate_order, validate_status, ApiError};
use crate::utils::{DbPool, Order, get_all_orders, get_order_by_id as db_get_order_by_id, 
                   create_order, update_order, update_order_status as db_update_order_status, 
                   delete_order};

#[derive(Debug, Deserialize, Serialize)]
pub struct StatusUpdate {
    pub status: String,
}

#[axum::debug_handler]
pub async fn get_orders(State(db_pool): State<DbPool>) -> Result<Json<Vec<Order>>, ApiError> {
    let orders = get_all_orders(&db_pool).await?;
    Ok(Json(orders))
}

#[axum::debug_handler]
pub async fn add_order(State(db_pool): State<DbPool>, Json(new_order): Json<Order>) -> Result<Json<Order>, ApiError> {
    // Validate the order first
    validate_order(&new_order)?;
    
    // Create the order in the database (includes duplicate ID check)
    let created_order = create_order(&db_pool, &new_order).await?;
    Ok(Json(created_order))
}

#[axum::debug_handler]
pub async fn get_order_by_id(
    State(db_pool): State<DbPool>,
    Path(id): Path<u32>,
) -> Result<Json<Order>, ApiError> {
    let order = db_get_order_by_id(&db_pool, id).await?
        .ok_or_else(|| ApiError::NotFound("Order not found".to_string()))?;
    Ok(Json(order))
}

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

pub async fn delete_order_by_id(
    State(db_pool): State<DbPool>,
    Path(id): Path<u32>,
) -> Result<Json<Order>, ApiError> {
    let deleted_order = delete_order(&db_pool, id).await?;
    Ok(Json(deleted_order))
}
