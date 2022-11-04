mod errors;

use std::{collections::HashMap, error::Error, fmt::Display, sync::Arc};

pub use errors::InjectionError;
pub use gotham_state::State;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ExtensionIdentifier {
	pub name: &'static str,
	pub version: &'static str,
}

impl ExtensionIdentifier {
	pub fn new(name: &'static str, version: &'static str) -> Self {
		ExtensionIdentifier { name, version }
	}
}

impl Clone for ExtensionIdentifier {
	fn clone(&self) -> Self {
		ExtensionIdentifier {
			name: self.name,
			version: self.version,
		}
	}
}

impl Display for ExtensionIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "'{}@{}'", self.name, self.version)
	}
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ExtensionDependency {
	pub name: &'static str,
	pub expected_version: &'static str,
}

impl Clone for ExtensionDependency {
	fn clone(&self) -> Self {
		ExtensionDependency {
			name: self.name,
			expected_version: self.expected_version,
		}
	}
}

pub trait BaseExtension {
	// Fired immediately when the extension has been injected.
	fn loaded(&self, _ctx: &mut ExtensionContext) {}
	// Fired when all required extensions has been injected..
	fn ready(&self, _ctx: &mut ExtensionContext) {}
	/// Fetches a reference to the identifier of *this* extension.
	fn identifier<'a>(&self, ctx: &mut ExtensionContext<'a>) -> &'a ExtensionIdentifier {
		&ctx.repository.identifier
	}
}

pub type BoxedExtension<T> = Box<T>;
pub type InstalledExtension<T> = Arc<BoxedExtension<T>>;

pub struct ExtensionInjector<'a, T: BaseExtension + Send + Sync> {
	identifier: &'a ExtensionIdentifier,
	extension: Option<BoxedExtension<T>>,
}

impl<'a, T: BaseExtension + Send + Sync> ExtensionInjector<'a, T> {
	pub fn inject(self: &mut ExtensionInjector<'a, T>, extension: T) -> Result<(), InjectionError<'a>> {
		if self.extension.is_none() {
			self.extension = Some(Box::new(extension));
			Ok(())
		} else {
			Err(InjectionError::new(
				&self.identifier,
				"This library has already injected an extension!".to_string(),
			))
		}
	}
}

pub enum Injector<T: BaseExtension + Send + Sync> {
	Dynamic(unsafe extern "C" fn(&mut ExtensionInjector<T>)),
	Library(fn(&mut ExtensionInjector<T>)),
	Internal(BoxedExtension<T>),
}

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

pub struct ExtensionContext<'a> {
	/// A state that is shared across every loaded extension.
	pub shared_state: &'a mut State,
	/// A state that belongs to the extension.
	pub state: &'a mut State,
	/// A proxy to the repository.
	repository: &'a RepositoryProxy<'a>,
}

impl<'a> ExtensionContext<'a> {}

pub struct RepositoryProxy<'a> {
	identifier: &'a ExtensionIdentifier,
}

impl<'a> RepositoryProxy<'a> {
	/// Get a list of extension identifiers that depend on *this* extension.
	pub fn get_dependent_identifiers() -> Vec<&'a ExtensionIdentifier> {
		todo!();
	}

	/// Get a dependency from the repository. This function returns an error
	/// if *this* extension does not directly depend on the requested extension.
	/// Returns None if the extension is not installed on the repository or
	/// Some(extension) otherwise.
	pub fn get_dependency<T: BaseExtension + Send + Sync>() -> Result<Option<&'a T>, Box<dyn Error + 'a>> {
		todo!();
	}
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct IdentifierContainer {
	identifier: Arc<ExtensionIdentifier>,
}

impl IdentifierContainer {
	pub fn name(&self) -> &'static str {
		self.identifier.name
	}

	pub fn version(&self) -> &'static str {
		self.identifier.name
	}
}

impl Display for IdentifierContainer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "'{}@{}'", self.name(), self.version())
	}
}

pub type InstalledVersion = &'static str;
pub type DependencyName = &'static str;
pub type ExtensionName = &'static str;
pub type ExpectedVersion = &'static str;
pub type RequiredDependency = bool;
pub type DependencyIdentifier = ExtensionIdentifier;

pub struct ExtensionRepository<'a, T: BaseExtension + Send + Sync> {
	pub installed_extensions: Vec<&'a T>,
	pub loaded_extensions: HashMap<&'static str, (&'a ExtensionIdentifier, &'a T)>,
	pub queued_extensions: Vec<&'a ExtensionManifest<T>>,
	pub duplicates: HashMap<ExtensionName, Vec<&'a ExtensionIdentifier>>,
	pub version_mismatches: HashMap<&'a DependencyIdentifier, Vec<(ExpectedVersion, ExtensionIdentifier)>>,
	pub pending_dependency: HashMap<DependencyName, Vec<(&'a ExtensionIdentifier, &'a ExtensionDependency)>>,
	pub pending_counter: HashMap<&'a ExtensionIdentifier, u32>,
	pub extension_sources: HashMap<&'a ExtensionIdentifier, Option<&'a str>>,
}

impl<T: BaseExtension + Send + Sync> ExtensionRepository<'_, T> {
	pub fn new() -> Self {
		ExtensionRepository {
			installed_extensions: Vec::new(),
			loaded_extensions: HashMap::new(),
			queued_extensions: Vec::new(),
			duplicates: HashMap::new(),
			version_mismatches: HashMap::new(),
			pending_dependency: HashMap::new(),
			pending_counter: HashMap::new(),
			extension_sources: HashMap::new(),
		}
	}
}
