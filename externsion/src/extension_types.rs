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
