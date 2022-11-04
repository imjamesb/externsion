use crate::{BaseExtension, ExtensionIdentifier};
use std::sync::Arc;

pub type InstalledVersion<'a> = &'a str;
pub type DependencyName<'a> = &'a str;
pub type ExtensionName<'a> = &'a str;
pub type ExpectedVersion<'a> = &'a str;
pub type RequiredDependency = bool;
pub type DependencyIdentifier = ExtensionIdentifier;
pub type BoxedExtension<'a> = Box<dyn BaseExtension + Send + Sync + 'a>;
pub type InstalledExtension<'a> = Arc<BoxedExtension<'a>>;
pub type ExtensionSource<'a> = &'a str;
