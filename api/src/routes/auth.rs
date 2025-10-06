use std::{sync::{Arc, Mutex}};

use jsonwebtoken::{encode, EncodingKey, Header};
use poem::{
    handler, post, web::{Data, Json}, Route
};
use store::{store::Store};

use crate::{
    config::Config, request_inputs::{CreateUserInput, SigninUserInput}, request_outputs::{CreateUserOutput, SigninUserOutput}
};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}


#[handler]
fn sign_up(
    Json(data): Json<CreateUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> poem::Result<Json<CreateUserOutput>> {
    let mut locked_s = s.lock().map_err(|_| {
        poem::Error::from_string(
            "failed to acquire lock",
            poem::http::StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    let id = locked_s
        .sign_up(data.username, data.password)
        .map_err(|e| {
            poem::Error::from_string(
                format!("Failed to create user :: {e}"),
                poem::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    let response = CreateUserOutput { id };

    Ok(Json(response))
}

#[handler]
async fn sign_in(
    Json(data): Json<SigninUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
    Data(config): Data<&Config>
) -> poem::Result<Json<SigninUserOutput>> {
    let mut locked_s = s.lock().map_err(|_| {
        poem::Error::from_string(
            "failed to acquire lock",
            poem::http::StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    let user_id = locked_s
        .sign_in(data.username, data.password)
        .map_err(|e| {
            poem::Error::from_string(
                format!("Failed to create user :: {e}"),
                poem::http::StatusCode::UNAUTHORIZED,
            )
        })?;

    let my_claims = Claims {
        sub: user_id,
        exp: 11111111111
    };

    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(config.jwt_secret.as_ref()))
        .map_err(|e| {
            poem::Error::from_string(
                format!("failed to encode password :: {e}"), 
                poem::http::StatusCode::INTERNAL_SERVER_ERROR
            )
        })?;

    Ok(Json(SigninUserOutput { jwt: token }))
}

pub fn routes() -> Route {
    Route::new()
        .at("/sign-in", post(sign_in))
        .at("/sign-up", post(sign_up))
}
