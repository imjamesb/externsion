use std::collections::HashMap;

use crate::{
	BaseExtension, DependencyIdentifier, DependencyName, ExpectedVersion, ExtensionDependency,
	ExtensionIdentifier, ExtensionManifest, ExtensionName,
};

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
