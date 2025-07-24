pub mod handlers;
pub use handlers::{
    get_orders, 
    add_order, 
    get_order_by_id, 
    update_order_by_id,
    update_order_status,
    delete_order_by_id,
    StatusUpdate
};

#[cfg(test)]
#[path = "handlers.tests.rs"]
mod tests;
