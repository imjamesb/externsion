use std::{error::Error, fmt::Display};

use crate::{ExtensionData, ExtensionIdentifier, ExtensionSource};

#[derive(Debug)]
pub struct QueueError<'a> {
	queued: (&'a ExtensionIdentifier, Option<ExtensionSource<'a>>),
	attempted: (&'a ExtensionIdentifier, Option<ExtensionSource<'a>>),
	pub data: Option<ExtensionData>,
}

impl<'a> QueueError<'a> {
	pub fn new(
		queued: (&'a ExtensionIdentifier, Option<ExtensionSource<'a>>),
		attempted: (&'a ExtensionIdentifier, Option<ExtensionSource<'a>>),
		data: Option<ExtensionData>,
	) -> QueueError<'a> {
		QueueError {
			queued,
			attempted,
			data,
		}
	}
}

impl Display for QueueError<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "An extension with the same identifier ({}) has already been queued or registered{}.{}", self.queued.0, match self.queued.1 {
			Some(source) => format!(" (from {})", source),
			None => "".to_string()
		}, match self.attempted.1 {
			Some(source) => format!(" Loading from {}", source),
			None => "".to_string()
		})
	}
}

impl Error for QueueError<'_> {}
