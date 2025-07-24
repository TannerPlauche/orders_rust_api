pub mod routes;
pub use routes::create_router;

#[cfg(test)]
#[path = "routes.tests.rs"]
mod tests;
