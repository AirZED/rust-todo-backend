use crate::entities::todo::{ActiveModel as TodoActiveModel, Entity as Todo, Model as TodoModel};
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response, Json};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};

use crate::db::get_connection;

pub async fn get_todos(_req: Request, _next: Next) -> Result<Response, StatusCode> {
    let db = get_connection()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let todos = Todo::find()
        .all(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let todo_json = serde_json::to_string(&todos).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(todo_json.into())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(response)
}
pub async fn get_single_todo() {

}

pub async fn create_todo() {}

pub async fn update_todo() {}

pub async fn delete_todo() {}
