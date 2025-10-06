use std::sync::{Arc, Mutex};

use poem::{
    Route, handler, post,
    web::{Data, Json},
};
use store::store::Store;

use crate::{
    request_inputs::{CreateUserInput, SigninUserInput},
    request_outputs::{CreateUserOutput, SigninUserOutput},
};

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
) -> poem::Result<Json<SigninUserOutput>> {
    let mut locked_s = s.lock().map_err(|_| {
        poem::Error::from_string(
            "failed to acquire lock",
            poem::http::StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    let jwt = locked_s
        .sign_in(data.username, data.password)
        .map_err(|e| {
            poem::Error::from_string(
                format!("Failed to create user :: {e}"),
                poem::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    Ok(Json(SigninUserOutput { jwt }))
}

pub fn routes() -> Route {
    Route::new()
        .at("/sign-in", post(sign_in))
        .at("/sign-up", post(sign_up))
}
