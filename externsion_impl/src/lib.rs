mod errors;

use externsion::*;

pub trait Repository<'a, T: BaseExtension + Send + Sync> {
	/// Queue a manifest to be added to the repository. It will only be queued and not touched
	/// until you you use `<Repository>.install()`
	fn queue(&mut self, manifest: &'a ExtensionManifest<T>);
	fn install(&mut self) -> Result<(), ()>;
}

impl<'a, T: BaseExtension + Send + Sync> Repository<'a, T> for ExtensionRepository<'a, T> {}
