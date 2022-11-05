use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct InstallError<'a> {
	description: &'a str,
}

impl<'a> InstallError<'a> {
	pub fn new(description: &'a str) -> InstallError<'a> {
		InstallError { description }
	}
}

impl Display for InstallError<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.description)
	}
}

impl Error for InstallError<'_> {
	fn description(&self) -> &str {
		self.description
	}
}
