use externsion::{
	ExtensionData, ExtensionIdentifier, ExtensionManifest,
	ExtensionSource, InnerRepository, InstallError, QueueError,
	Repository as ExtRepository, RepositoryOperation, UnloadError,
	UnqueueError,
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
	// Note that any failed queued extensions will have to be
	// unloaded manually.
	fn queue(
		&mut self,
		manifest: &'a ExtensionManifest,
		source: Option<ExtensionSource<'a>>,
		data: Option<ExtensionData>,
	) -> Result<&'a ExtensionIdentifier, QueueError<'a>> {
		if self.inner.queued_extensions.contains(&&manifest.identifier)
			|| self
				.inner
				.pending_extensions_r
				.contains_key(&manifest.identifier)
		{
			Err(QueueError::new(
				(
					&manifest.identifier,
					match self.inner.sources.get(&manifest.identifier) {
						Some(source) => Some(source),
						_ => None,
					},
				),
				(&manifest.identifier, source),
				data,
			))
		} else {
			self.inner.queued_extensions.push(&manifest.identifier);
			if let Some(data) = data {
				self.inner.queued_data.insert(&manifest.identifier, data);
			}
			if let Some(source) = source {
				self.inner.sources.insert(&manifest.identifier, source);
			}
			self.inner
				.queued_manifests
				.insert(&manifest.identifier, manifest);
			Ok(&manifest.identifier)
		}
	}

	fn unqueue(
		&mut self,
		identifier: &'a ExtensionIdentifier,
	) -> Result<(&'a ExtensionManifest, Option<ExtensionData>), UnqueueError>
	{
		let Some(index) = self
			.inner
			.queued_extensions
			.iter()
			.position(|x| x == &identifier) else {
				return Err(UnqueueError::new(format!(
					"Extension {} is not in the repository queue!",
					identifier
				)))
			};
		self.inner.queued_extensions.remove(index);
		self.inner.sources.remove(identifier);
		Ok((
			self.inner.queued_manifests.remove(identifier).unwrap(),
			self.inner.queued_data.remove(identifier),
		))
	}

	fn push(
		&mut self,
		manifest: &'a ExtensionManifest,
		source: Option<&'a str>,
		data: Option<ExtensionData>,
	) -> Result<&'a ExtensionIdentifier, InstallError<'a>> {
		todo!();
	}

	fn flush(&mut self) -> RepositoryOperation<'a> {
		todo!();
	}

	fn install(&mut self) -> RepositoryOperation<'a> {
		todo!();
	}

	fn unload(
		&mut self,
		identifier: &'a ExtensionIdentifier,
	) -> Result<
		(&'a ExtensionManifest, Option<ExtensionData>),
		UnloadError<'a>,
	> {
		todo!();
	}
}
