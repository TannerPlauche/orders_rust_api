use axum::{
    http::StatusCode,
    response::Json,
    routing::{get, patch},
    Router,
};
use serde_json::json;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers::{ // bring in all handler functions
    get_orders,
    add_order,
    get_order_by_id,
    update_order_by_id,
    update_order_status,
    delete_order_by_id,
};
use crate::utils::DbPool;
use crate::openapi::ApiDoc;

// Fallback handler for unmatched routes
async fn path_not_found() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "error": "Path not found",
            "message": "The requested endpoint does not exist"
        }))
    )
}

pub fn create_router(db_pool: DbPool) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/orders", get(get_orders).post(add_order))
        .route(
            "/orders/:id",
            get(get_order_by_id).put(update_order_by_id).delete(delete_order_by_id)
        )
        .route("/orders/:id/status", patch(update_order_status))
        .fallback(path_not_found)
        .with_state(db_pool)
}
