use axum::{extract::{Json, Path}, http::StatusCode, response::IntoResponse};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub completed: bool,
}

#[derive(Deserialize)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

use crate::{
    db::get_connection,
    entities::todo::{ActiveModel as TodoActiveModel, Entity as Todo, Model as TodoModel},
};

pub async fn get_todos() -> Result<impl IntoResponse, StatusCode> {
    let db = get_connection().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let todos = Todo::find().all(db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(todos))
}

pub async fn get_single_todo(Path(id): Path<i32>) -> Result<impl IntoResponse, StatusCode> {
    let db = get_connection().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let todo = Todo::find_by_id(id).one(db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(todo))
}

pub async fn create_todo(Json(payload): Json<CreateTodoRequest>) -> Result<impl IntoResponse, StatusCode> {
    let db = get_connection().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let new_todo = TodoActiveModel {
        title: Set(payload.title),
        completed: Set(payload.completed),
        ..Default::default()
    };
    let inserted = new_todo.insert(db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::CREATED, Json(inserted)))
}

pub async fn update_todo(
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let db = get_connection().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(todo) = Todo::find_by_id(id).one(db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
        let mut active: TodoActiveModel = todo.into();
        if let Some(title) = payload.title {
            active.title = Set(title);
        }
        if let Some(completed) = payload.completed {
            active.completed = Set(completed);
        }
        let updated = active.update(db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(updated))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn delete_todo(Path(id): Path<i32>) -> Result<impl IntoResponse, StatusCode> {
    let db = get_connection().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Todo::delete_by_id(id).exec(db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}
