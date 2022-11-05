use std::fmt::Display;

#[derive(Debug, Eq, Hash)]
pub struct ExtensionDependency {
	pub name: &'static str,
	pub expected_version: &'static str,
}

impl Clone for ExtensionDependency {
	fn clone(&self) -> Self {
		ExtensionDependency {
			name: self.name,
			expected_version: self.expected_version,
		}
	}
}

impl PartialEq for ExtensionDependency {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
			&& self.expected_version == other.expected_version
	}

	fn ne(&self, other: &Self) -> bool {
		!self.eq(other)
	}
}

impl Display for ExtensionDependency {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "'{} ({})'", self.name, self.expected_version)
	}
}
