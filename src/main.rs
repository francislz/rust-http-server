#![allow(dead_code)]

use server::Server;
use website_handler::WebSiteHandler;
use std::env;

mod server;
mod website_handler;
mod http;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run(WebSiteHandler::new(public_path));
}
