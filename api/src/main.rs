use poem::{listener::TcpListener, Route, Server};

use crate::routes::{auth, website};

pub mod request_inputs;
pub mod request_outputs;
pub mod routes;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let port = 3001;
    let addr = format!("0.0.0.0:{port}");

    let app = Route::new()
        .nest("/auth", auth::routes())
        .nest("/website", website::routes());
    
    println!("Server is running on http://{}", addr);

    Server::new(TcpListener::bind(addr))
        .run(app)
        .await
}   