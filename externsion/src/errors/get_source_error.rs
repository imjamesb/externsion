use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct GetSourceError<'a> {
	description: &'a str,
}

impl<'a> GetSourceError<'a> {
	pub fn new(description: &'a str) -> GetSourceError<'a> {
		GetSourceError { description }
	}
}

impl Display for GetSourceError<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.description)
	}
}

impl Error for GetSourceError<'_> {
	fn description(&self) -> &str {
		self.description
	}
}
