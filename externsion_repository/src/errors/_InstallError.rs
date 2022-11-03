use externsion::*;
use std::{collections::HashMap, error::Error, fmt::Display};

#[derive(Debug)]
pub struct InstallError<'a> {
	duplicates: Option<HashMap<ExtensionName, Vec<&'a ExtensionIdentifier>>>,
	version_mismatches:
		Option<HashMap<&'a DependencyIdentifier, Vec<(ExpectedVersion, ExtensionIdentifier)>>>,
	pending_dependency:
		Option<HashMap<DependencyName, Vec<(&'a ExtensionIdentifier, &'a ExtensionDependency)>>>,
	description: String,
	caused_by: Option<Box<dyn Error>>,
	sources: HashMap<&'a ExtensionIdentifier, &'a str>,
}

impl<'a> InstallError<'a> {
	pub fn new(
		duplicates: Option<HashMap<ExtensionName, Vec<&'a ExtensionIdentifier>>>,
		version_mismatches: Option<
			HashMap<&'a DependencyIdentifier, Vec<(ExpectedVersion, ExtensionIdentifier)>>,
		>,
		pending_dependency: Option<
			HashMap<DependencyName, Vec<(&'a ExtensionIdentifier, &'a ExtensionDependency)>>,
		>,
		description: String,
		caused_by: Option<Box<dyn Error>>,
		sources: Option<HashMap<&'a ExtensionIdentifier, &'a str>>,
	) -> InstallError<'a> {
		InstallError {
			duplicates,
			version_mismatches,
			pending_dependency,
			description,
			caused_by,
			sources: {
				if let Some(sources) = sources {
					sources
				} else {
					HashMap::new()
				}
			},
		}
	}
	pub fn empty(description: String) -> InstallError<'a> {
		InstallError {
			duplicates: None,
			version_mismatches: None,
			pending_dependency: None,
			description,
			caused_by: None,
			sources: HashMap::new(),
		}
	}

	pub fn duplicates(&self) -> &Option<HashMap<ExtensionName, Vec<&'a ExtensionIdentifier>>> {
		&self.duplicates
	}

	pub fn version_mismatches(
		&self,
	) -> &Option<HashMap<&'a DependencyIdentifier, Vec<(ExpectedVersion, ExtensionIdentifier)>>> {
		&self.version_mismatches
	}

	pub fn pending_dependency(
		&self,
	) -> &Option<HashMap<DependencyName, Vec<(&'a ExtensionIdentifier, &'a ExtensionDependency)>>> {
		&self.pending_dependency
	}
}

impl Display for InstallError<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut has_written_header = false;
		let mut has_written = false;
		if let Some(duplicates) = &self.duplicates {
			let mut has_written_duplicates = false;
			for (extension_name, dupes) in duplicates.iter() {
				if dupes.len() < 2 {
					continue;
				}
				if !has_written_header {
					has_written_header = true;
					let write_result = write!(
						f,
						"{}\r\n\r\nSomething went wrong duing extension installation!\r\n\r\n{}\r\n\r\nDuplicate extensions was queued:",
						match &self.caused_by {
							Some(error) => format!("\r\n\r\nCaused by:\r\n\r\n{}", error),
							None => "".to_string()
						},
						self.description
					);
					if write_result.is_err() {
						return write_result;
					}
				}
				has_written_duplicates = true;
				has_written = false;
				{
					let write_result = write!(f, "\r\n  {} has ", extension_name);
					if write_result.is_err() {
						return write_result;
					}
				}
				for duplicate in dupes.iter() {
					let write_result = write!(
						f,
						"{}{}{}",
						match has_written {
							true => ", ",
							false => "",
						},
						duplicate,
						match self.sources.contains_key(duplicate) {
							true => {
								format!(" ({})", self.sources.get(duplicate).unwrap())
							},
							false => String::from(""),
						}
					);
					has_written = true;
					if write_result.is_err() {
						return write_result;
					}
				}
			}
			has_written = has_written_duplicates;
		}
		if let Some(pending_dependencies) = &self.pending_dependency {
			if !has_written_header {
				has_written_header = true;
				let write_result = write!(
					f,
					"{}\r\n\r\nSomething went wrong duing extension installation!\r\n\r\n{}\r\n\r\n",
					match &self.caused_by {
						Some(error) => format!("\r\n\r\nCaused by:\r\n\r\n{}", error),
						None => "".to_string(),
					},
					self.description
				);
				if write_result.is_err() {
					return write_result;
				}
			}
			if has_written {
				let write_result = write!(f, "\r\n\r\n");
				if write_result.is_err() {
					return write_result;
				}
			} else {
				has_written = true;
			}
			{
				let write_result = write!(f, "Some required dependencies were not queued and is missing:");
				if write_result.is_err() {
					return write_result;
				}
			}
			for (dependency_name, dependents) in pending_dependencies.iter() {
				if dependents.len() < 1 {
					continue;
				}
				{
					let write_result = write!(f, "\r\n  {}:", dependency_name);
					if write_result.is_err() {
						return write_result;
					}
				}
				for (identifier, dependency) in dependents.iter() {
					let write_result =
						write!(f, "\r\n    {} requires {}", identifier, dependency.expected_version);
					if write_result.is_err() {
						return write_result;
					}
				}
			}
		}
		if let Some(version_mismatches) = &self.version_mismatches {
			if !has_written_header {
				let write_result = write!(
					f,
					"{}\r\n\r\nSomething went wrong duing extension installation!\r\n\r\n{}\r\n\r\n",
					match &self.caused_by {
						Some(error) => format!("\r\n\r\nCaused by:\r\n\r\n{}", error),
						None => "".to_string(),
					},
					self.description
				);
				if write_result.is_err() {
					return write_result;
				}
			}
			if has_written {
				let write_result = write!(f, "\r\n\r\n");
				if write_result.is_err() {
					return write_result;
				}
			} else {
				has_written = true;
			}
			{
				let write_result =
					write!(f, "Some dependencies does not meet the requirements other extensions:");
				if write_result.is_err() {
					return write_result;
				}
			}
			for (dependency, dependents) in version_mismatches.iter() {
				{
					let write_result =
						write!(f, "\r\n  {} (installed: {}", dependency.name, dependency.version);
					if write_result.is_err() {
						return write_result;
					}
				}
				for (expected_version, dependent) in dependents.iter() {
					let write_result = write!(f, "\r\n    {} requires: {}", dependent, expected_version);
					if write_result.is_err() {
						return write_result;
					}
				}
			}
		}
		if has_written {
			let write_result = write!(f, "\r\n\r\n");
			if write_result.is_err() {
				return write_result;
			}
		}
		Ok(())
	}
}

impl Error for InstallError<'_> {
	fn description(&self) -> &str {
		self.description.as_str()
	}
	fn cause(&self) -> Option<&dyn Error> {
		match &self.caused_by {
			Some(error) => Some(error.as_ref()),
			None => None,
		}
	}
}
