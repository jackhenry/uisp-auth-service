#[macro_use]
extern crate log;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;

use listenfd::ListenFd;

mod api_error;
mod cache;
mod device;
mod login;
mod uisp;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    cache::init();
    cache::populate().await.expect("Failed to populate cache.");

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().configure(login::init_routes));
    server = if let Some(listener) = listenfd.take_tcp_listener(0)? {
        server.listen(listener)?
    } else {
        server.bind(("0.0.0.0", 8080))?
    };

    server.run().await
}
