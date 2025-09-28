use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::de::Read;
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

impl CreateUserModel {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if self.email.trim().is_empty() || !self.email.contains('@') {
            return Err("Invalid email address".to_string());
        }
        if self.password.len() < 6 {
            return Err("Password must be at least 6 characters long".to_string());
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct ReadUserModel {
    pub email: String,
    pub password: String,
}

impl ReadUserModel {
    pub fn validate(&self) -> Result<(), String> {
        if self.email.trim().is_empty() || !self.email.contains('@') {
            return Err("Invalid email address".to_string());
        }
        if self.password.len() < 6 {
            return Err("Password must be at least 6 characters long".to_string());
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserModel {
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserQueryModel {
    pub uuid: String,
}
