use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub db_url: String,
    pub port: u16,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok(); //this line loads the .env file

        Self {
            db_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string()),
        }
    }
}
