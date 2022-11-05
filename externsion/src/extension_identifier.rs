use std::fmt::Display;

#[derive(Debug, Eq, Hash)]
pub struct ExtensionIdentifier {
	pub name: &'static str,
	pub version: &'static str,
}

impl ExtensionIdentifier {
	pub fn new(name: &'static str, version: &'static str) -> Self {
		ExtensionIdentifier { name, version }
	}
}

impl Clone for ExtensionIdentifier {
	fn clone(&self) -> Self {
		ExtensionIdentifier {
			name: self.name,
			version: self.version,
		}
	}
}

impl Display for ExtensionIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "'{}@{}'", self.name, self.version)
	}
}

impl PartialEq for ExtensionIdentifier {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name && self.version == other.version
	}

	fn ne(&self, other: &Self) -> bool {
		!self.eq(other)
	}
}
