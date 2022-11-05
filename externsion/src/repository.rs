use crate::{
	ExtensionIdentifier, ExtensionManifest, ExtensionSource,
	GetSourceError, InstallError, QueueError, RepositoryOperation,
	SetSourceError, UnloadError,
};

/// Methods that allow manipulating the inner extension
/// repository.
pub trait Repository<'a> {
	/// Queue an extension to be loaded. Returns an error if
	/// a manifest with the same identifier has been queued.
	fn queue(
		&mut self,
		manifest: &'a ExtensionManifest,
		source: Option<&'a str>,
	) -> Result<&'a ExtensionIdentifier, QueueError<'a>>;

	/// Attempt to install an extension directly onto the
	/// repository and skip the queue. Returns an error if
	/// the extension failed to be installed for any reason.
	/// Otherwise the extension identifier will be returned.
	fn push(
		&mut self,
		manifest: &'a ExtensionManifest,
		source: Option<&'a str>,
	) -> Result<&'a ExtensionIdentifier, InstallError<'a>>;

	/// Flush all extensions in the queue and attempt to
	/// install them. If duplicates are detected it will
	/// register them into the operation error. If an
	/// extension cannot be loaded to a pending dependency
	/// it will not be installed, but it will not cause an
	/// error either. All installed extension will be emitted
	/// into the `installed` property of the
	/// `RepositoryOperation`.
	fn flush(&mut self) -> RepositoryOperation<'a>;

	/// Flush all extensions in the queue and attempt to
	/// install them. If duplicates are detected it will
	/// register them into the operation error. If an
	/// extension cannot be installed due to a pending
	/// or missing dependency, it will be registered into
	/// the error. All installed extensions will be emitted
	/// into the `installed` property of the
	/// `RepositoryOperation`.
	fn install(&mut self) -> RepositoryOperation<'a>;

	/// Attempt to remove an error from the queue. It will
	/// return ownership of the manifest. Returns an error
	/// if for some reason the manifest can not be removed
	/// from the queue.
	fn unqueue(
		&mut self,
		identifier: &'a ExtensionIdentifier,
	) -> Result<ExtensionManifest, QueueError<'a>>;

	/// Attempt to set the source for an extension. The
	/// extension to set the source for must exist within the
	/// queue.
	fn set_source(
		&mut self,
		identifier: &'a ExtensionIdentifier,
		source: ExtensionSource<'a>,
	) -> Result<(), SetSourceError<'a>>;

	/// Attempt to get the source of an extension. Returns an
	/// error if the extension does not exist within the
	/// repository. Or an option that might contain a str if
	/// a source has been set for the requested identifier.
	fn get_source(
		&self,
		identifier: &'a ExtensionIdentifier,
	) -> Result<Option<ExtensionSource<'a>>, GetSourceError<'a>>;

	/// Attempt to unload an extension from the repository.
	/// Returns an error if the extension could not be
	/// unloaded or the extension manifest if it was unloaded
	/// successfully.
	fn unload(
		&mut self,
		identifier: &'a ExtensionIdentifier,
	) -> Result<ExtensionManifest, UnloadError<'a>>;
}
