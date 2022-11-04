use std::{error::Error, fmt::Display};

use crate::ExtensionIdentifier;

#[derive(Debug)]
pub struct InjectionError<'a> {
	extension_identifier: &'a ExtensionIdentifier,
	description: String,
	caused_by: Option<Box<dyn Error>>,
}

impl InjectionError<'_> {
	pub fn new<'a>(
		extension_identifier: &'a ExtensionIdentifier,
		description: String,
	) -> InjectionError<'a> {
		InjectionError {
			extension_identifier,
			description,
			caused_by: None,
		}
	}
	pub fn caused_by<'a>(
		extension_identifier: &'a ExtensionIdentifier,
		description: String,
		error: Box<dyn Error>,
	) -> InjectionError<'a> {
		InjectionError {
			extension_identifier,
			description,
			caused_by: Some(error),
		}
	}

	pub fn extension_identifier(&self) -> &ExtensionIdentifier {
		self.extension_identifier
	}
}

impl Display for InjectionError<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Couldn't inject extension {}: {}",
			self.extension_identifier, self.description
		)
	}
}

impl Error for InjectionError<'_> {
	fn description(&self) -> &str {
		self.description.as_str()
	}
	fn cause(&self) -> Option<&dyn Error> {
		match &self.caused_by {
			Some(error) => Some(error.as_ref()),
			None => None,
		}
	}
}
