use axum::{http::Method, routing::post, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::controllers::auth::{login, signup};

pub fn auth() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/signup", post(signup))
        .layer(cors)
}
