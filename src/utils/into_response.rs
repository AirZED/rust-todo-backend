use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::error::ApiError;

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            ApiError::DatabaseError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            ApiError::UserNotFound => (StatusCode::NOT_FOUND, "User not found".to_owned()),
            ApiError::DatabaseConnectionError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
            ApiError::BadRequest(_) => (StatusCode::BAD_REQUEST, "Bad request".to_owned()),
            ApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_owned(),
            ),
        };

        (status_code, error_message).into_response()
    }
}
