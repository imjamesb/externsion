use crate::{BoxedExtension, ExtensionIdentifier, InjectionError};

pub struct ExtensionInjector<'a> {
	identifier: &'a ExtensionIdentifier,
	extension: Option<BoxedExtension<'a>>,
}

impl<'a> ExtensionInjector<'a> {
	pub fn inject(
		self: &mut ExtensionInjector<'a>,
		extension: BoxedExtension<'a>,
	) -> Result<(), InjectionError<'a>> {
		if self.extension.is_none() {
			self.extension = Some(extension);
			Ok(())
		} else {
			Err(InjectionError::new(
				&self.identifier,
				"This library has already injected an extension!"
					.to_string(),
			))
		}
	}
}

pub enum Injector<'a> {
	Dynamic(unsafe extern "C" fn(&mut ExtensionInjector)),
	Library(fn(&mut ExtensionInjector)),
	Internal(BoxedExtension<'a>),
}
