use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateWebsiteInput {
    pub url: String
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserInput {
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct  SigninUserInput {
    pub username: String,
    pub password: String
}