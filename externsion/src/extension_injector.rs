use crate::{BaseExtension, BoxedExtension, ExtensionIdentifier, InjectionError};

pub struct ExtensionInjector<'a, T: BaseExtension + Send + Sync> {
	identifier: &'a ExtensionIdentifier,
	extension: Option<BoxedExtension<T>>,
}

impl<'a, T: BaseExtension + Send + Sync> ExtensionInjector<'a, T> {
	pub fn inject(self: &mut ExtensionInjector<'a, T>, extension: T) -> Result<(), InjectionError<'a>> {
		if self.extension.is_none() {
			self.extension = Some(Box::new(extension));
			Ok(())
		} else {
			Err(InjectionError::new(
				&self.identifier,
				"This library has already injected an extension!".to_string(),
			))
		}
	}
}

pub enum Injector<T: BaseExtension + Send + Sync> {
	Dynamic(unsafe extern "C" fn(&mut ExtensionInjector<T>)),
	Library(fn(&mut ExtensionInjector<T>)),
	Internal(BoxedExtension<T>),
}
