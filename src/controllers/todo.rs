use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

// pub async fn get_todos(req: Request, next: Next) -> Result<Response, StatusCode> {
//     Ok(StatusCode::OK)
// }

pub async fn get_single_todo() {}

pub async fn create_todo() {}

pub async fn update_todo() {}

pub async fn delete_todo() {}
