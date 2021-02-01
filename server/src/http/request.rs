use std::str;
use std::str::{ Utf8Error };
use std::fmt::{ Display, Debug, Formatter, Result as FmtResult };
use std::error::Error;
use std::convert::TryFrom;

use super::method::{ Method, MethodError };
use super::{ QueryString };

#[derive(Debug)]
pub struct Request <'buf> {
	path: &'buf str,
	query: Option <QueryString <'buf>>,
	method: Method
}

impl <'buf> Request <'buf> {
	pub fn path(&self) -> &str {
		&self.path
	}

	pub fn query(&self) -> Option <&QueryString> {
		self.query.as_ref()
	}

	pub fn method(&self) -> &Method {
		&self.method
	}
}

pub enum ParseError {
	InvalidRequest,
	InvalidEncoding,
	InvalidProtocol,
	InvalidMethod,
}

impl ParseError {
	fn message(&self) -> &str {
		match self {
			Self::InvalidRequest => "Invalid Request",
			Self::InvalidEncoding => "Invalid Encoding",
			Self::InvalidProtocol => "Invalid Protocol",
			Self::InvalidMethod => "Invalid Method",
		}
	}
}

impl Display for ParseError {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "{}", self.message())
	}
}

impl Debug for ParseError {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "{}", self.message())
	}
}

impl From <Utf8Error> for ParseError {
	fn from(_: Utf8Error) -> Self {
		Self::InvalidEncoding
	}
}

impl From <MethodError> for ParseError {
	fn from(_: MethodError) -> Self {
		Self::InvalidMethod
	}
}

impl Error for ParseError {}

impl <'buf> TryFrom <&'buf [u8]> for Request <'buf> {
	type Error = ParseError;

	fn try_from(buf: &'buf [u8]) -> Result<Request <'buf>, Self::Error> {
		// this works because we added an Utf8Error impl for ParseError
		let request = str::from_utf8(buf)?;

		// get method
		let (method, request) = get_next_word(request)
			.ok_or(ParseError::InvalidRequest)?;

		// get requested url
		let (mut path, request) = get_next_word(request)
			.ok_or(ParseError::InvalidRequest)?;

		// get protocol
		let (protocol, _) = get_next_word(request)
			.ok_or(ParseError::InvalidRequest)?;

		if protocol != "HTTP/1.1" {
			return Err(ParseError::InvalidProtocol);
		}

		let method: Method = method.parse()?;

		let mut query_string = None;
		if let Some (i) = path.find('?') {
			query_string = Some(QueryString::from(&path[i + 1..]));
			path = &path[..i];
		}

		Ok(Self {
			path: path,
			query: query_string,
			method
		})
	}
}

fn get_next_word(request: &str) -> Option <(&str, &str)> {
	for (idx, c) in request.chars().enumerate() {
		if (c == ' ') || (c == '\r') {
			return Some((&request[..idx], &request[idx + 1..]));
		}
	}

	return None
}
