use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Hash)]
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
