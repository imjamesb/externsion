mod errors;

use externsion::*;

pub trait Repository<'a, T: BaseExtension + Send + Sync> {
	/// Queue a manifest to be added to the repository. It will only be queued and not touched
	/// until you you use `<Repository>.install()`
	fn queue(self: &mut ExtensionRepository<'a, T>, manifest: &'a ExtensionManifest<T>);
	fn install(self: &mut ExtensionRepository) -> Result<()>;
}

impl<'a, T: BaseExtension + Send + Sync> Repository for ExtensionRepository<'a, T> {}
