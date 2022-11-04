#![warn(elided_lifetimes_in_paths)]

mod errors;

use errors::*;
use externsion::*;

pub enum ExtensionStage {
	/// Extension is not known to the repository.
	Unknown,
	/// Extension is found in repository install queue.
	Queued,
	/// Extension is a duplicate.
	HasDuplicates,
	/// The extension is being processed, but is pending a dependency before it can be installed.
	PendingDependency,
	/// The extension is installed and active.
	Installed,
}

pub trait Repository<'a, T: BaseExtension + Send + Sync> {
	/// Queue a manifest to be added to the repository. It will only be queued and not touched
	/// until you you use `<Repository>.install()` or `<Repository>.flush()`
	fn queue(
		&mut self,
		manifest: &'a ExtensionManifest<T>,
		source: Option<&'a str>,
	) -> Result<(), InstallError<'a>>;
	/// Attempts to install an extension directly into extension repository without inserting it
	/// into the queue. Returns an error if the extension could not be installed, the extension
	/// otherwise.
	fn push(
		&mut self,
		manifest: &'a ExtensionManifest<T>,
		source: Option<&'a str>,
	) -> Result<&'a ExtensionIdentifier, InstallError<'a>>;
	/// Flush all extensions in the queue and attempt to activate them into your extension system.
	/// Returns a vector of those extension's identifiers that could be activated from the queue.
	/// Errors on duplicates.
	fn flush(&mut self) -> Result<&'a Vec<&'a ExtensionIdentifier>, InstallError<'a>>;
	/// Flush all extensions and returns an error if there are missing dependencies or mismatches
	/// dependency versions. Returns a vector of those extension's identifiers that could be
	/// activated from the queue.
	fn install(&mut self) -> Result<&'a Vec<&'a ExtensionIdentifier>, InstallError<'a>>;
	/// Set the source path of an extension.
	fn set_source(
		&mut self,
		extension: &'a ExtensionIdentifier,
		source: &'a str,
	) -> Result<(), SetSourceError<'a>>;
	/// Get the stage of an extension in the repository.
	fn stage(&mut self, extension: &'a ExtensionIdentifier) -> ExtensionStage;
	/// Unqueue an extension from being loaded. Returns true if the extension was unloaded, or false if the extension is
	/// not in the queue.
	fn unqueue(&mut self, extension: &'a ExtensionIdentifier) -> bool;
	/// Get the source for an extension.
	fn get_source(&self, extension: &'a ExtensionIdentifier) -> Option<&'a str>;
}

impl<'a, T: BaseExtension + Send + Sync> Repository<'a, T> for InnerRepository<'a, T> {
	fn queue(
		&mut self,
		manifest: &'a ExtensionManifest<T>,
		source: Option<&'a str>,
	) -> Result<(), InstallError<'a>> {
		if self.queued_extensions.contains(&manifest) {
			Err(InstallError::empty("This extension is already queued".to_string()))
		} else {
			self.queued_extensions.push(manifest);
			if let Some(source) = source {
				if let Err(error) = self.set_source(&manifest.identifier, source) {
					return Err(InstallError::caused_by(
						"Failed to set source.".to_string(),
						Box::new(error),
					));
				}
			}
			Ok(())
		}
	}

	fn push(
		&mut self,
		manifest: &'a ExtensionManifest<T>,
		source: Option<&'a str>,
	) -> Result<&'a ExtensionIdentifier, InstallError<'a>> {
		if let Some(source) = source {
			if !self.queued_extensions.contains(&manifest) {
				self.queued_extensions.push(manifest);
			}
			let index = self
				.queued_extensions
				.iter()
				.position(|x| x.identifier == manifest.identifier)
				.unwrap();
			let result = self.set_source(&manifest.identifier, source);
			self.queued_extensions.remove(index);
			if let Err(error) = result {
				return Err(InstallError::<'a>::caused_by(
					"Could not set source.".to_string(),
					Box::new(error),
				));
			}
		}
		manifest.dependencies;
		todo!();
	}

	fn flush(&mut self) -> Result<&'a Vec<&'a ExtensionIdentifier>, InstallError<'a>> {
		todo!();
	}

	fn install(&mut self) -> Result<&'a Vec<&'a ExtensionIdentifier>, InstallError<'a>> {
		todo!();
	}

	fn set_source(
		&mut self,
		extension: &'a ExtensionIdentifier,
		source: &'a str,
	) -> Result<(), SetSourceError<'a>> {
		let stage = self.stage(extension);
		match stage {
			ExtensionStage::Unknown => {
				Err(SetSourceError::set(extension, source, "Extension is not in repository!"))
			},
			ExtensionStage::PendingDependency => Err(SetSourceError::<'a>::replace(
				extension,
				match self.extension_sources.get(extension).unwrap_or_else(|| &None) {
					Some(value) => Some(value),
					_ => None,
				},
				source,
				"Extension is being processed and loaded into repository!",
			)),
			ExtensionStage::HasDuplicates => Err(SetSourceError::replace(
				extension,
				match self.extension_sources.get(extension).unwrap_or_else(|| &None) {
					Some(value) => Some(value),
					_ => None,
				},
				source,
				"Extension is being processed and loaded into repository!",
			)),
			ExtensionStage::Installed => Err(SetSourceError::replace(
				extension,
				match self.extension_sources.get(extension).unwrap_or_else(|| &None) {
					Some(value) => Some(value),
					_ => None,
				},
				source,
				"Extension is already installed!",
			)),
			ExtensionStage::Queued => {
				if let Some(Some(src)) = self.extension_sources.get(extension) {
					Err(SetSourceError::replace(extension, Some(src), source, "Source is already set!"))
				} else {
					self.extension_sources.insert(extension, Some(source));
					Ok(())
				}
			},
		}
	}

	fn stage(&mut self, extension: &'a ExtensionIdentifier) -> ExtensionStage {
		let is_some = {
			self.queued_extensions
				.iter()
				.position(|x| &x.identifier == extension)
				.is_some()
		};
		if is_some {
			ExtensionStage::Queued
		} else if self.duplicates.contains_key(extension.name)
			&& self.duplicates.get(extension.name).unwrap().len() > 1
		{
			ExtensionStage::HasDuplicates
		} else if self.pending_counter.contains_key(extension)
			&& self.pending_counter.get(extension).unwrap() > &0
		{
			ExtensionStage::PendingDependency
		} else if self.loaded_extensions.contains_key(extension.name) {
			ExtensionStage::Installed
		} else {
			ExtensionStage::Unknown
		}
	}

	fn unqueue(&mut self, extension: &'a ExtensionIdentifier) -> bool {
		if let Some(index) = self
			.queued_extensions
			.iter()
			.position(|x| &x.identifier == extension)
		{
			self.queued_extensions.remove(index);
			true
		} else {
			false
		}
	}

	fn get_source(&self, extension: &'a ExtensionIdentifier) -> Option<&'a str> {
		if let Some(source) = self.extension_sources.get(extension) {
			if let Some(source) = source {
				Some(source)
			} else {
				None
			}
		} else {
			None
		}
	}
}
