use poem::{handler, post, web::Json, Route};
use store::store::Store;

use crate::{request_inputs::{CreateUserInput, SigninUserInput}, request_outputs::{CreateUserOutput, SigninUserOutput}};

#[handler]
fn sign_up(Json(data): Json<CreateUserInput>) -> Json<CreateUserOutput> {
    let mut s = Store::default().unwrap();

    let id = s.sign_up(data.username, data.password).unwrap();

    let response = CreateUserOutput {
        id
    };

    Json(response)
}

#[handler]
async fn sign_in(Json(data): Json<SigninUserInput>) -> Json<SigninUserOutput> {
    let mut s = Store::default().unwrap();

    let _ = s.sign_in(data.username, data.password).unwrap();

    let responec = SigninUserOutput {
        jwt: String::new()
    };

    Json(responec)
}

pub fn routes() -> Route {
    Route::new()
        .at("/sign-in", post(sign_in))
        .at("/sign-up", post(sign_up))
}