use std::io::Read;
use std::convert::TryFrom;
use std::net::TcpListener;

use crate::http::Request;

pub struct Server {
	address: String
}

impl Server {
	pub fn new(address: String) -> Self {
		Self {
			address: address
		}
	}

	pub fn run(self) {
		let listener = TcpListener::bind(&self.address).unwrap();

		println!("Listening on {}", self.address);

		loop {
			match listener.accept() {
				Ok((mut stream, addr)) => {
					println!("new client: {:?}", addr);

					let mut buffer = [0; 4096];
					match stream.read(&mut buffer) {
						Ok(size) => {
							println!(
								"Received a request ({}): {:?}",
								size,
								String::from_utf8_lossy(&buffer)
							);

							Request::try_from(&buffer[..]);
						},
						Err(e) => {
							println!("failed to read from connection: {:?}", e)
						}
					}
				},

				Err(e) => println!("couldn't get client: {:?}", e)
			}
		}
	}
}