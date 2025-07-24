mod handlers;
mod routes;
mod validators;
mod utils;

use axum::serve;
use routes::create_router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use utils::init_db;

#[tokio::main]
async fn main() {
    // Initialize the database
    let db_pool = init_db().await.expect("Failed to initialize database");
    
    let app = create_router(db_pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app.into_make_service()).await.unwrap();
}
