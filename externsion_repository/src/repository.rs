use externsion::{
	ExtensionData, ExtensionIdentifier, ExtensionManifest,
	ExtensionSource, GetSourceError, InnerRepository, InstallError,
	QueueError, Repository as ExtRepository, RepositoryOperation,
	SetSourceError, UnloadError,
};

pub struct Repository<'a> {
	inner: InnerRepository<'a>,
}

impl Repository<'_> {
	pub fn new<'a>() -> Repository<'a> {
		Repository {
			inner: InnerRepository::new(),
		}
	}
}

impl<'a> ExtRepository<'a> for Repository<'a> {
	fn queue(
		&mut self,
		manifest: &'a ExtensionManifest,
		source: Option<&'a str>,
		data: Option<&'a ExtensionData>,
	) -> Result<&'a ExtensionIdentifier, QueueError<'a>> {
		todo!();
	}

	fn unqueue(
		&mut self,
		identifier: &'a ExtensionIdentifier,
	) -> Result<ExtensionManifest, QueueError<'a>> {
		todo!();
	}

	fn push(
		&mut self,
		manifest: &'a ExtensionManifest,
		source: Option<&'a str>,
		data: Option<&'a ExtensionData>,
	) -> Result<&'a ExtensionIdentifier, InstallError<'a>> {
		todo!();
	}

	fn flush(&mut self) -> RepositoryOperation<'a> {
		todo!();
	}

	fn install(&mut self) -> RepositoryOperation<'a> {
		todo!();
	}

	fn set_source(
		&mut self,
		identifier: &'a ExtensionIdentifier,
		source: ExtensionSource<'a>,
	) -> Result<(), SetSourceError<'a>> {
		todo!();
	}

	fn get_source(
		&self,
		identifier: &'a ExtensionIdentifier,
	) -> Result<Option<ExtensionSource<'a>>, GetSourceError<'a>> {
		todo!();
	}

	fn unload(
		&mut self,
		identifier: &'a ExtensionIdentifier,
	) -> Result<ExtensionManifest, UnloadError<'a>> {
		todo!();
	}
}
