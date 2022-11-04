use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct QueueError<'a> {
	description: &'a str,
}

impl<'a> QueueError<'a> {
	pub fn new(description: &'a str) -> QueueError<'a> {
		QueueError { description }
	}
}

impl Display for QueueError<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.description)
	}
}

impl Error for QueueError<'_> {
	fn description(&self) -> &str {
		self.description
	}
}
