use std::env;

use server::Server;
use handler::WebsiteHandler;

mod server;
mod handler;
mod http;

fn main() {
	let default_path = format! ("{}/", env! ("CARGO_MANIFEST_DIR"));
	let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

	let address = String::from("127.0.0.1:8080");
	let server = Server::new(address);

	server.run(WebsiteHandler::new(public_path));
}