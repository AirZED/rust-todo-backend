use std::net::{Ipv4Addr, SocketAddrV4};

use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{patch, post};
use axum::Json;
use axum::{routing::get, Router};
use chrono::Utc;
use entity::user;
use migration::Mode;
use models::user::{CreateUserModel, ReadUserModel, UpdateUserModel, UserModel};
use uuid::Uuid;

mod config;
mod controllers;
mod db;
mod models;
mod routes;
mod utils;

use config::Config;
use controllers::auth::{login, signup};
use controllers::todo::{create_todo, delete_todo, get_single_todo, get_todos, update_todo};
use db::{get_connection, init_db};
use routes::*;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::{Condition, Database, DatabaseConnection};
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
        .route("/auth/login", post(login_user))
        .route("/auth/signup", post(create_user))
        .route("/user/update_user", patch(update_user));

    let port = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, config.port);
    let listener = tokio::net::TcpListener::bind(port).await.unwrap();

    init_db().await.expect("Failed to connect to the database");
    info!("🚀 Starting server at {}", port);
    axum::serve(listener, app).await.unwrap();
}

async fn create_user(Json(user_data): Json<CreateUserModel>) -> impl IntoResponse {
    let db: DatabaseConnection = Database::connect("mysql://root:%23%23%23@localhost:3306/todo_db")
        .await
        .unwrap();

    let user_model = user::ActiveModel {
        name: Set(user_data.name.to_owned()),
        email: Set(user_data.email.to_owned()),
        password: Set(user_data.password.to_owned()),
        uuid: Set(Uuid::new_v4().into()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    let user = user_model.insert(&db).await.unwrap();

    db.close().await.unwrap();
    (
        StatusCode::ACCEPTED,
        format!("User created successfully: {}", user.id),
    )
}
async fn login_user(Json(user_data): Json<ReadUserModel>) -> impl IntoResponse {
    let db: DatabaseConnection = Database::connect("mysql://root:%23%23%23@localhost:3306/todo_db")
        .await
        .unwrap();

    let user = user::Entity::find()
        .filter(
            Condition::all()
                .add(user::Column::Email.eq(user_data.email.to_owned()))
                .add(user::Column::Password.eq(user_data.password.to_owned())),
        )
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    let data = UserModel {
        name: user.name,
        email: user.email,
        password: user.password,
        uuid: Uuid::from_slice(&user.uuid).unwrap(),
        created_at: user.created_at,
    };

    db.close().await.unwrap();
    (StatusCode::ACCEPTED, Json(data))
}

async fn update_user(
    Path(uuid): Path<Uuid>,
    Json(user_data): Json<UpdateUserModel>,
) -> impl IntoResponse {
    let db: DatabaseConnection = Database::connect("mysql://root:%23%23%23@localhost:3306/todo_db")
        .await
        .unwrap();

    let mut user: user::ActiveModel = user::Entity::find()
        .filter(user::Column::Uuid.eq(uuid))
        .one(&db)
        .await
        .unwrap()
        .unwrap()
        .into();
    // the into here converts it to the type that is written above

    user.name = Set(user_data.name.to_owned());

    user.update(&db).await.unwrap();

    (StatusCode::ACCEPTED, "Updated user")
}
