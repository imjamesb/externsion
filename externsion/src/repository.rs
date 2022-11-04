use crate::{BaseExtension, ExtensionManifest};
use std::error::Error;

/// Definition on how the repository works.
pub trait Repository<'a, T: BaseExtension + Send + Sync> {
	/// Queue an extension to be loaded. Returns an error if
	/// a manifest with the same identifier has been queued.
	fn queue(
		&mut self,
		manifest: &'a ExtensionManifest<T>,
		source: Option<&'a str>,
	) -> Result<(), Box<dyn Error + 'a>>;
}

// todo(imjamesb): Create an error type for Queue errors
// and replace the error type for `Repository.queue`
