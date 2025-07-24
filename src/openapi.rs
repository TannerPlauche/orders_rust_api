use utoipa::OpenApi;
use crate::utils::Order;
use crate::handlers::StatusUpdate;
use crate::validators::{ValidationError, ServerError};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::handlers::get_orders,
        crate::handlers::handlers::add_order,
        crate::handlers::handlers::get_order_by_id,
        crate::handlers::handlers::update_order_by_id,
        crate::handlers::handlers::update_order_status,
        crate::handlers::handlers::delete_order_by_id,
    ),
    components(
        schemas(Order, StatusUpdate, ValidationError, ServerError)
    ),
    tags(
        (name = "orders", description = "Order management endpoints")
    ),
    info(
        title = "Rust Order Management API",
        description = "A comprehensive REST API for managing orders with SQLite persistence",
        version = "1.0.0",
        contact(
            name = "API Support",
            email = "support@example.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    servers(
        (url = "http://localhost:3000", description = "Local development server")
    )
)]
pub struct ApiDoc;
