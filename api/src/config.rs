use std::env;

#[derive(Clone)]
pub struct Config {
    pub jwt_secret: String,
    pub database_url: String
}

impl Config {
    pub fn from_env() -> Self {
        let jwt_secret = env::var("JWT_SECRET")
            .expect("JWT_Secret must be set");

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        Self {
            jwt_secret,
            database_url
        }
    }
}