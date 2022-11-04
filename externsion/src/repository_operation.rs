use crate::{ExtensionIdentifier, InstallError};

pub struct RepositoryOperation<'a> {
	installed: Option<Vec<&'a ExtensionIdentifier>>,
	error: Option<InstallError<'a>>,
}
