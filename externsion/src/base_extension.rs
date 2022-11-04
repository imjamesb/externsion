use crate::{ExtensionContext, ExtensionIdentifier};

pub trait BaseExtension {
	/// Fired immediately when the extension has been
	/// injected.
	fn loaded(&self, _ctx: &mut ExtensionContext) {}
	/// Fired when all required extensions has been injected.
	fn ready(&self, _ctx: &mut ExtensionContext) {}
	/// Fetches a reference to the identifier of *this*
	/// extension.
	fn identifier<'a>(
		&self,
		ctx: &mut ExtensionContext<'a>,
	) -> &'a ExtensionIdentifier {
		&ctx.repository.identifier
	}
}
