use std::fmt::Debug;

use crate::State;

pub struct ExtensionData {
	pub state: State,
	unloader: fn(state: &mut State),
	unloaded: bool,
}

impl ExtensionData {
	pub fn new(unloader: fn(state: &mut State)) -> ExtensionData {
		ExtensionData {
			state: State::default(),
			unloader,
			unloaded: false,
		}
	}
	pub fn unload(&mut self) {
		if self.unloaded {
			return;
		}
		(self.unloader)(&mut self.state);
		self.unloaded = true;
	}
}

impl Debug for ExtensionData {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.state.fmt(f)
	}
}
