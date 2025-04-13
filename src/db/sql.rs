use crate::config::Config;
use once_cell::sync::OnceCell;
use sea_orm::{Database, DatabaseConnection};

static DB_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::new();

// Initialize the DB connection once during startup
pub async fn init_db() -> Result<(), sea_orm::DbErr> {
    let db = Database::connect(Config::from_env().db_url).await?;
    match DB_CONNECTION.set(db) {
        Ok(_) => Ok(()),
        Err(_) => Ok(()),
    }
}

// Get the connection for use in handlers
pub async fn get_connection() -> Result<&'static DatabaseConnection, sea_orm::DbErr> {
    DB_CONNECTION
        .get()
        .ok_or_else(|| sea_orm::DbErr::Custom("Database not initialized".to_string()))
}
