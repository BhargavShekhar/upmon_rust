use poem::{handler, listener::TcpListener, post, web::{Json, Path}, Route, Server};

use crate::{request_inputs::CreateWebsiteInput, request_outputs::{CreateWebsiteOutput}};

pub mod request_inputs;
pub mod request_outputs;

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let url = data.url;

    let response = CreateWebsiteOutput {
        id: url
    };

    Json(response)
}

fn _get_website(Path(name): Path<String>) -> String {
    format!("hello: {name}")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // specify the buisness details of the app
    let app = Route::new()
        .at("/website", post(create_website));
    
    // creates and runs the http server
    Server::new(TcpListener::bind("0.0.0.0:3001"))
        .run(app)
        .await
}