use crate::{BaseExtension, ExtensionIdentifier};
use std::error::Error;

pub struct RepositoryProxy<'a> {
	pub identifier: &'a ExtensionIdentifier,
}

impl<'a> RepositoryProxy<'a> {
	/// Get a list of extension identifiers that depend on
	/// *this* extension.
	pub fn get_dependent_identifiers() -> Vec<&'a ExtensionIdentifier> {
		todo!();
	}

	/// Get a dependency from the repository. This function
	/// returns an error if *this* extension does not
	/// directly depend on the requested extension. Returns
	/// `None` if the extension is not installed on the
	/// repository or `Some(extension)` otherwise.
	pub fn get_dependency<T: BaseExtension + Send + Sync>() -> Result<Option<&'a T>, Box<dyn Error + 'a>> {
		todo!();
	}
}
