use std::sync::{Arc, Mutex};

use poem::{listener::TcpListener, EndpointExt, Route, Server};
use store::store::Store;

use crate::{config::Config, routes::{auth, website}};

pub mod request_inputs;
pub mod request_outputs;
pub mod routes;
pub mod config;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let port = 3001;
    let addr = format!("0.0.0.0:{port}");

    let s = Arc::new(
        Mutex::new(
            Store::new().unwrap()
        )
    );

    let config = Config::from_env();

    let app = Route::new()
        .nest("/auth", auth::routes())
        .nest("/website", website::routes())
        .data(s)
        .data(config.clone());
    
    println!("Server is running on http://{}", addr);

    Server::new(TcpListener::bind(addr))
        .run(app)
        .await
}