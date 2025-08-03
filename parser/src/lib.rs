use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ParseCommandError {
	EmptyString,
	NoSubcommand,

	UnknownCategory(String),
	UnknownSubcommand(String),

	InvalidArgument(String),
	DuplicateArgument(String),
	UnexpectedArgument(String),
	MissingArgument(String),
}

#[derive(Debug, Clone)]
pub struct Arguments<'a> {
	values: HashMap<&'a str, &'a str>,
}

impl<'a> From<HashMap<&'a str, &'a str>> for Arguments<'a> {
	fn from(value: HashMap<&'a str, &'a str>) -> Self {
		Self { values: value }
	}
}

impl<'a> Arguments<'a> {
	pub fn parse(string: &'a str, options: &[&str]) -> Result<Self, ParseCommandError> {
		let mut result = HashMap::new();

		for arg in string.trim().split(' ').filter(|s| !s.is_empty()) {
			let Some((key, value)) = arg.split_once('=') else {
				return Err(ParseCommandError::InvalidArgument(arg.to_string()));
			};

			if result.contains_key(key) {
				return Err(ParseCommandError::DuplicateArgument(key.to_string()));
			}

			result.insert(key, value);
		}

		for key in result.keys() {
			if !options.contains(key) {
				return Err(ParseCommandError::UnexpectedArgument(key.to_string()));
			}
		}

		Ok(result.into())
	}

	pub fn get<T: FromStr>(&self, name: &str) -> Result<T, ParseCommandError> {
		let Ok(Some(value)) = self.get_optional(name) else {
			return Err(ParseCommandError::MissingArgument(name.to_string()));
		};

		Ok(value)
	}

	pub fn get_optional<T: FromStr>(&self, name: &str) -> Result<Option<T>, ParseCommandError> {
		let Some(value) = self.values.get(name) else {
			return Ok(None);
		};

		value
			.parse()
			.map_err(|_| ParseCommandError::InvalidArgument(name.to_string()))
			.map(Some)
	}
}
