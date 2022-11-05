use crate::{
	DependencyName, ExpectedVersion, ExtensionData, ExtensionDependency,
	ExtensionIdentifier, ExtensionManifest, ExtensionName,
	ExtensionSource, InstalledExtension,
};
use std::collections::HashMap;

pub struct InnerRepository<'a> {
	/// Used to define a source path for an identifier.
	pub sources: HashMap<&'a ExtensionIdentifier, ExtensionSource<'a>>,
	// Queue uses ExtensionIdentifier to refer to an
	// extension
	pub queued_extensions: Vec<&'a ExtensionIdentifier>,
	pub queued_manifests:
		HashMap<&'a ExtensionIdentifier, &'a ExtensionManifest>,
	pub queued_data: HashMap<&'a ExtensionIdentifier, ExtensionData>,
	// When a dependency becomes dependent on another
	// extension it should be removed from the
	// `queue_extensions` property and moved into a seperate
	// queue for pending extensions.
	pub pending_extensions: HashMap<
		DependencyName<'a>,
		Vec<(&'a ExtensionIdentifier, ExpectedVersion<'a>)>,
	>,
	pub pending_extensions_r:
		HashMap<&'a ExtensionIdentifier, Vec<&'a ExtensionDependency>>,
	/// Used to keep track of what extension identifiers has
	/// been visited.
	pub maybe_duplicates:
		HashMap<ExtensionName<'a>, Vec<&'a ExtensionIdentifier>>,
	// Installed extensions are referred to by name,
	pub installed_name_identifier_map:
		HashMap<ExtensionName<'a>, &'a ExtensionIdentifier>,
	pub installed_extensions:
		HashMap<ExtensionName<'a>, InstalledExtension<'a>>,
	pub installed_data: HashMap<&'a ExtensionIdentifier, ExtensionData>,
	pub installed_manifests:
		HashMap<&'a ExtensionIdentifier, &'a ExtensionManifest>,
}

impl<'a> InnerRepository<'a> {
	pub fn new() -> InnerRepository<'a> {
		InnerRepository {
			sources: HashMap::new(),
			queued_extensions: Vec::new(),
			queued_manifests: HashMap::new(),
			queued_data: HashMap::new(),
			pending_extensions: HashMap::new(),
			pending_extensions_r: HashMap::new(),
			maybe_duplicates: HashMap::new(),
			installed_name_identifier_map: HashMap::new(),
			installed_extensions: HashMap::new(),
			installed_data: HashMap::new(),
			installed_manifests: HashMap::new(),
		}
	}

	pub fn set_source(
		&mut self,
		identifier: &'a ExtensionIdentifier,
		source: &'a str,
	) {
		self.sources.insert(identifier, source);
	}

	pub fn insert_into_queue(
		&mut self,
		manifest: &'a ExtensionManifest,
		data: Option<ExtensionData>,
	) -> &'a ExtensionIdentifier {
		self.queued_extensions.push(&manifest.identifier);
		self.queued_manifests.insert(&manifest.identifier, manifest);
		if let Some(data) = data {
			self.queued_data.insert(&manifest.identifier, data);
		}
		&manifest.identifier
	}

	pub fn add_pending(
		&mut self,
		identifier: &'a ExtensionIdentifier,
		dependency: &'a ExtensionDependency,
	) {
		if self.queued_extensions.contains(&identifier) {
			self.queued_extensions.remove(
				self.queued_extensions
					.iter()
					.position(|x| x == &identifier)
					.unwrap(),
			);
		}
		if let Some(vec) = self.pending_extensions.get_mut(dependency.name)
		{
			vec.push((identifier, dependency.expected_version));
		} else {
			self.pending_extensions.insert(
				dependency.name,
				vec![(identifier, dependency.expected_version)],
			);
		}
		if let Some(vec) = self.pending_extensions_r.get_mut(identifier) {
			vec.push(dependency);
		} else {
			self.pending_extensions_r
				.insert(identifier, vec![dependency]);
		}
	}

	/// Returns `true` if the identifier has no more pending
	/// dependencies.
	pub fn remove_pending(
		&mut self,
		identifier: &'a ExtensionIdentifier,
		dependency: &'a ExtensionDependency,
	) -> bool {
		let mut remove_ext = false;
		let mut remove_extr = false;
		if let Some(vec) = self.pending_extensions.get_mut(dependency.name)
		{
			if let Some(index) = vec.iter().position(|x| x.0 == identifier)
			{
				vec.remove(index);
				if vec.len() == 0 {
					remove_ext = true;
				}
			}
		}
		if remove_ext {
			self.pending_extensions.remove(dependency.name);
		}
		if let Some(vec) = self.pending_extensions_r.get_mut(identifier) {
			if let Some(index) = vec.iter().position(|x| x == &dependency)
			{
				vec.remove(index);
				if vec.len() == 0 {
					remove_extr = true;
				}
			}
		}
		if remove_extr {
			self.pending_extensions_r.remove(identifier);
		}
		remove_extr
	}
}
