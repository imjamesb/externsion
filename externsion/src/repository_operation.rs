use crate::{ExtensionIdentifier, InstallError};

pub struct RepositoryOperation<'a> {
	pub installed: Option<Vec<&'a ExtensionIdentifier>>,
	pub error: Option<InstallError<'a>>,
}
