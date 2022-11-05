use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct UnqueueError<'a> {
	description: &'a str,
}

impl<'a> UnqueueError<'a> {
	pub fn new(description: &'a str) -> UnqueueError<'a> {
		UnqueueError { description }
	}
}

impl Display for UnqueueError<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.description)
	}
}

impl Error for UnqueueError<'_> {
	fn description(&self) -> &str {
		self.description
	}
}
