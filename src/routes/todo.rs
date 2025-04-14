use axum::{http::Method, routing::delete, routing::get, routing::patch, routing::post, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::controllers::todo::{create_todo, delete_todo, get_single_todo, get_todos, update_todo};

pub fn auth_routes() -> Router {
    // defined allowed methods
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    Router::new()
        .route("/todo", get(get_todos).post(create_todo))
        .route(
            "/todo/{id}",
            get(get_single_todo).patch(update_todo).delete(delete_todo),
        )
        .layer(cors)
}
