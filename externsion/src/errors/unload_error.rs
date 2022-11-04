use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct UnloadError<'a> {
	description: &'a str,
}

impl<'a> UnloadError<'a> {
	pub fn new(description: &'a str) -> UnloadError<'a> {
		UnloadError { description }
	}
}

impl Display for UnloadError<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.description)
	}
}

impl Error for UnloadError<'_> {
	fn description(&self) -> &str {
		self.description
	}
}
