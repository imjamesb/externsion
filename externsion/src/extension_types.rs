use crate::ExtensionIdentifier;
use std::sync::Arc;

pub type InstalledVersion = &'static str;
pub type DependencyName = &'static str;
pub type ExtensionName = &'static str;
pub type ExpectedVersion = &'static str;
pub type RequiredDependency = bool;
pub type DependencyIdentifier = ExtensionIdentifier;
pub type BoxedExtension<T> = Box<T>;
pub type InstalledExtension<T> = Arc<BoxedExtension<T>>;

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
