use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct UnqueueError {
	description: String,
}

impl UnqueueError {
	pub fn new(description: String) -> UnqueueError {
		UnqueueError { description }
	}
}

impl Display for UnqueueError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.description)
	}
}

impl Error for UnqueueError {
	fn description(&self) -> &str {
		self.description.as_str()
	}
}
