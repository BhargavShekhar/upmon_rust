use std::sync::{Arc, Mutex};

use poem::{handler, web::{Data, Json, Path}, Route};
use store::store::Store;

use crate::{request_inputs::CreateWebsiteInput, request_outputs::{CreateWebsiteOutput, GetWebsiteOutput}};

#[handler]
fn create_website(
    Json(
        data): Json<CreateWebsiteInput>,
        Data(s): Data<&Arc<Mutex<Store>>>
    ) -> Json<CreateWebsiteOutput> {
    let mut locked_s = match s.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            eprintln!("Website:: create_website :: Mutex poisoned: {}", poisoned);
            return  Json(CreateWebsiteOutput {
                id: String::new()
            });
        }
    };

    let website = locked_s.create_website(
        String::from("636a7ef1-67c3-4359-af04-19edef3faadf"), // TODO get user_id 
        data.url
    ).unwrap();

    let responce = CreateWebsiteOutput {
        id: website.id
    };

    Json(responce)
}

#[handler]
fn get_website(
    Path(id): Path<String>,
    Data(s): Data<&Arc<Mutex<Store>>>
) -> Json<GetWebsiteOutput> {
    let mut locked_s = match s.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            eprintln!("Website:: get_website :: Mutex poisoned {}", poisoned);
            return  Json(GetWebsiteOutput {
                url: String::new()
            });
        }
    };

    let websites = locked_s.get_website(id).unwrap();

    let responce = GetWebsiteOutput {
        url: websites.url
    };

    Json(responce)
}

pub fn routes() -> Route {
    Route::new()
        .at("/", create_website)
}