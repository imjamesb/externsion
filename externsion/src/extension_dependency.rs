#[derive(Debug, PartialEq, Eq, Hash)]
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
