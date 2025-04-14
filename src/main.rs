use std::net::{Ipv4Addr, SocketAddrV4};

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{routing::get, Router};
use chrono::Utc;
use entity::user;
use uuid::Uuid;

mod config;
mod controllers;
mod db;
mod entities;
mod routes;
mod utils;

use config::Config;
use controllers::auth::{login, signup};
use controllers::todo::{create_todo, delete_todo, get_single_todo, get_todos, update_todo};
use db::{get_connection, init_db};
use routes::*;
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use sea_orm::{Database, DatabaseConnection};
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
        .route("/auth/signup", post(create_user));

    let port = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, config.port);
    let listener = tokio::net::TcpListener::bind(port).await.unwrap();

    init_db().await.expect("Failed to connect to the database");
    info!("🚀 Starting server at {}", port);
    axum::serve(listener, app).await.unwrap();
}

async fn create_user() -> impl IntoResponse {
    let db: DatabaseConnection = Database::connect("mysql://root:%23%23%23@localhost:3306/todo_db")
        .await
        .unwrap();

    let user_model = user::ActiveModel {
        name: Set("Mfoniso".to_owned()),
        email: Set("mfoniso@gmail.com".to_owned()),
        password: Set("password".to_owned()),
        uuid: Set(Uuid::new_v4().into()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    let user = user_model.insert(&db).await.unwrap();

    (
        StatusCode::ACCEPTED,
        format!("User created successfully: {}", user.id),
    )
}
