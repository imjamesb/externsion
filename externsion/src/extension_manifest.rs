use crate::{ExtensionDependency, ExtensionIdentifier, Injector};

pub struct ExtensionManifest {
	pub identifier: ExtensionIdentifier,
	pub dependencies: Option<&'static [ExtensionDependency]>,
	pub optional_dependencies: Option<&'static [ExtensionDependency]>,
	pub installer: Injector<'static>,
}

impl PartialEq for ExtensionManifest {
	fn eq(&self, other: &Self) -> bool {
		self.identifier == other.identifier
	}
}
