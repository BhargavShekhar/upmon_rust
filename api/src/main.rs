use std::sync::{Arc, Mutex};

use poem::{listener::TcpListener, EndpointExt, Route, Server};
use store::store::Store;

use crate::routes::{auth, website};

pub mod request_inputs;
pub mod request_outputs;
pub mod routes;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let port = 3001;
    let addr = format!("0.0.0.0:{port}");

    let s = Arc::new(
        Mutex::new(
            Store::new().unwrap()
        )
    );

    let app = Route::new()
        .nest("/auth", auth::routes())
        .nest("/website", website::routes())
        .data(s);
    
    println!("Server is running on http://{}", addr);

    Server::new(TcpListener::bind(addr))
        .run(app)
        .await
}