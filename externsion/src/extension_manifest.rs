use crate::{BaseExtension, ExtensionDependency, ExtensionIdentifier, Injector};

pub struct ExtensionManifest<T: BaseExtension + Send + Sync> {
	pub identifier: ExtensionIdentifier,
	pub dependencies: Option<&'static [ExtensionDependency]>,
	pub optional_dependencies: Option<&'static [ExtensionDependency]>,
	pub installer: Injector<T>,
}

impl<T: BaseExtension + Send + Sync> PartialEq for ExtensionManifest<T> {
	fn eq(&self, other: &Self) -> bool {
		self.identifier == other.identifier
	}
}
