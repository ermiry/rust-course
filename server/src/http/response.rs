use std::fmt::{ Display, Formatter, Result as FmtResult };
use std::io::{ Write, Result as IOResult };

use super::status::StatusCode;

#[derive(Debug)]
pub struct Response {
	status: StatusCode,
	body: Option <String>
}

impl Response {
	pub fn new(status: StatusCode, body: Option <String>) -> Self {
		Response {status, body}
	}

	// with this we are writting directly to the stream
	// so n additional allocations are needed
	pub fn send(&self, stream: &mut impl Write) -> IOResult <()> {
		let body = match &self.body {
			Some (b) => b,
			None => ""
		};

		write! (
			stream,
			"HTTP/1.1 {} {}\r\n\r\n{}",
			self.status, self.status.reason_phrase(),
			body
		)
	}
}

impl Display for Response {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		let body = match &self.body {
			Some (b) => b,
			None => ""
		};

		write! (
			f,
			"HTTP/1.1 {} {}\r\n\r\n{}",
			self.status, self.status.reason_phrase(),
			body
		)
	}
}