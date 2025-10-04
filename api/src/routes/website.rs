use poem::{handler, web::{Json, Path}, Route};
use store::store::Store;

use crate::{request_inputs::CreateWebsiteInput, request_outputs::{CreateWebsiteOutput, GetWebsiteOutput}};

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let mut s = Store::new().unwrap();

    let website = s.create_website(
        String::from("636a7ef1-67c3-4359-af04-19edef3faadf"),
        data.url
    ).unwrap();

    let responce = CreateWebsiteOutput {
        id: website.id
    };

    Json(responce)
}

#[handler]
fn get_website(Path(id): Path<String>) -> Json<GetWebsiteOutput> {
    let mut s = Store::new().unwrap();

    let websites = s.get_website(id).unwrap();

    let responce = GetWebsiteOutput {
        url: websites.url
    };

    Json(responce)
}

pub fn routes() -> Route {
    Route::new()
        .at("/", create_website)
}