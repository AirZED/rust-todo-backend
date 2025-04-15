use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserModel {
    pub name: String,
    pub email: String,
    pub password: String,
    pub uuid: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserModel {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReadUserModel {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserModel {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserQueryModel {
    pub uuid: String,
}
