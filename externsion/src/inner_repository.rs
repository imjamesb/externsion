use crate::{
	DependencyName, ExtensionIdentifier, ExtensionManifest, ExtensionName,
	ExtensionSource, InstalledExtension, State,
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
	pub queued_data: HashMap<&'a ExtensionIdentifier, State>,
	// When a dependency becomes dependent on another
	// extension it should be removed from the
	// `queue_extensions` property and moved into a seperate
	// queue for pending extensions.
	pub pending_extensions:
		HashMap<DependencyName<'a>, Vec<&'a ExtensionIdentifier>>,
	pub pending_extensions_r:
		HashMap<&'a ExtensionIdentifier, Vec<DependencyName<'a>>>,
	/// Used to keep track of what extension identifiers has
	/// been visited.
	pub maybe_duplicates:
		HashMap<ExtensionName<'a>, Vec<&'a ExtensionIdentifier>>,
	// Installed extensions are referred to by name,
	pub installed_name_identifier_map:
		HashMap<ExtensionName<'a>, &'a ExtensionIdentifier>,
	pub installed_extensions:
		HashMap<ExtensionName<'a>, InstalledExtension<'a>>,
	pub installed_data: HashMap<&'a ExtensionIdentifier, State>,
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
		data: Option<State>,
	) -> &'a ExtensionIdentifier {
		self.queued_extensions.push(&manifest.identifier);
		self.queued_manifests.insert(&manifest.identifier, manifest);
		if let Some(data) = data {
			self.queued_data.insert(&manifest.identifier, data);
		}
		&manifest.identifier
	}
}
