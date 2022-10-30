mod errors;

use errors::InstallError;
use externsion::*;

pub trait Repository<'a, T: BaseExtension + Send + Sync> {
	/// Queue a manifest to be added to the repository. It will only be queued and not touched
	/// until you you use `<Repository>.install()`
	fn queue(&mut self, manifest: &'a ExtensionManifest<T>);
	/// Flush all extensions in the queue and attempt to activate them into your extension system.
	/// Returns a vector of those extension's identifiers that could be activated from the queue.
	fn flush(&mut self) -> Result<&'a Vec<&'a ExtensionIdentifier>, InstallError>;
	/// Flush all extensions and returns an error if there are missing dependencies or mismatches
	/// dependency versions. Returns a vector of those extension's identifiers that could be
	/// activated from the queue.
	fn install(&mut self) -> Result<&'a Vec<&'a ExtensionIdentifier>, InstallError>;
}

impl<'a, T: BaseExtension + Send + Sync> Repository<'a, T> for ExtensionRepository<'a, T> {
	fn queue(&mut self, manifest: &'a ExtensionManifest<T>) {
		self.queued_extensions.push(manifest);
	}
	fn flush(&mut self) -> Result<&'a Vec<&'a ExtensionIdentifier>, InstallError> {
		todo!();
	}
	fn install(&mut self) -> Result<&'a Vec<&'a ExtensionIdentifier>, InstallError> {
		todo!();
	}
}
