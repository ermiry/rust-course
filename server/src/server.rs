use std::io::{ Read };
use std::convert::TryFrom;
use std::net::TcpListener;

use crate::http::{ Request, ParseError, Response, StatusCode };

pub trait Handler {
	fn handle_request(&mut self, request: &Request) -> Response;

	fn handle_bad_request(&mut self, e: &ParseError) -> Response {
		println!("Failed to parse request: {:?}", e);
		Response::new(StatusCode::BadRequest, None)
	}
}

pub struct Server {
	address: String
}

impl Server {
	pub fn new(address: String) -> Self {
		Self {
			address: address
		}
	}

	pub fn run(self, mut handler: impl Handler) {
		let listener = TcpListener::bind(&self.address).unwrap();

		println!("Listening on {}", self.address);

		loop {
			match listener.accept() {
				Ok((mut stream, addr)) => {
					println!("new client: {:?}", addr);

					let mut buffer = [0; 4096];
					match stream.read(&mut buffer) {
						Ok(_size) => {
							// println!(
							// 	"Received a request ({}): {:?}",
							// 	size,
							// 	String::from_utf8_lossy(&buffer)
							// );

							let response = match Request::try_from(&buffer[..]) {
								Ok(request) => {
									dbg! (&request);

									handler.handle_request(&request)

									// this will use the Display impl and
									// allocate a new string before sending through the socket
									// write! (stream, "{}", res);
								}

								Err(e) => {
									handler.handle_bad_request(&e)
								}
							};

							// send directly through the stream
							if let Err(e) = response.send(&mut stream) {
								println! ("Failed to send response: {}", e);
							}
						},
						Err(e) => {
							println! ("Failed to read from connection: {:?}", e)
						}
					}
				},

				Err(e) => println! ("Failed to establish connection: {:?}", e)
			}
		}
	}
}