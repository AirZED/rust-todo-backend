use std::net::{Ipv4Addr, SocketAddrV4};

use axum::{routing::get, Router};

mod config;
mod controllers;
mod db;
mod entities;
mod utils;

use config::Config;
use controllers::auth::{login, signup};
use controllers::todo::{create_todo, delete_todo, get_single_todo, get_todos, update_todo};
use db::init_db;
use tracing::info;

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get("Server is working properly"))
        .route("/todo", get(get_todos).post(create_todo))
        .route(
            "/todo/{id}",
            get(get_single_todo).patch(update_todo).delete(delete_todo),
        )
        .route("/auth/login", get(login))
        .route("/auth/signup", get(signup));

    let port = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, config.port);
    let listener = tokio::net::TcpListener::bind(port).await.unwrap();

    init_db().await.expect("Failed to connect to the database");
    info!("🚀 Starting server at {}", port);
    axum::serve(listener, app).await.unwrap();
}
