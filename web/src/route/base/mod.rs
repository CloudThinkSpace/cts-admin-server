use axum::Router;
use axum::routing::{get, post};
use crate::handler::base;
pub fn login() -> Router {
    Router::new()
        .route("/login", post(base::login))
        .route("/logout", get(base::logout))
        .route("/register", post(base::register))
}