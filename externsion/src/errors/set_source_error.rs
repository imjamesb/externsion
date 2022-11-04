use crate::ExtensionIdentifier;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct SetSourceError<'a> {
	identifier: &'a ExtensionIdentifier,
	previous_value: Option<&'a str>,
	new_value: &'a str,
	reason: &'a str,
}

impl<'a> SetSourceError<'a> {
	pub fn replace(
		identifier: &'a ExtensionIdentifier,
		previous_value: Option<&'a str>,
		new_value: &'a str,
		reason: &'a str,
	) -> SetSourceError<'a> {
		SetSourceError {
			identifier,
			previous_value,
			new_value,
			reason,
		}
	}
	pub fn set(
		identifier: &'a ExtensionIdentifier,
		new_value: &'a str,
		reason: &'a str,
	) -> SetSourceError<'a> {
		SetSourceError {
			identifier,
			previous_value: None,
			new_value,
			reason,
		}
	}
}

impl Display for SetSourceError<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Some(previous_value) = self.previous_value {
			write!(
				f,
				"Could not replace source ({:?}) with {:?} for {}: {}",
				previous_value, self.new_value, self.identifier, self.reason
			)
		} else {
			write!(f, "Could not set source {:?} for {}: {}", self.new_value, self.identifier, self.reason)
		}
	}
}

impl Error for SetSourceError<'_> {
	fn description(&self) -> &str {
		self.reason
	}
}
