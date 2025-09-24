use axum::{Router, routing::get};
mod hello;

pub fn routes() -> Router {
    Router::new().route("/", get(hello::get))
}
