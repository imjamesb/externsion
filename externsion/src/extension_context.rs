use crate::{RepositoryProxy, State};

pub struct ExtensionContext<'a> {
	/// A state that is shared across every loaded extension.
	pub shared_state: &'a mut State,
	/// A state that belongs to the extension.
	pub state: &'a mut State,
	/// A proxy to the repository.
	pub repository: &'a RepositoryProxy<'a>,
}

impl<'a> ExtensionContext<'a> {}
