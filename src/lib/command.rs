use macros::{CommandCategory, Subcommand};
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq, CommandCategory)]
pub enum Command {
	Album(AlbumCommand),
}

#[derive(Debug, Clone, Eq, PartialEq, Subcommand)]
pub enum AlbumCommand {
	List {
		#[default(SortDirection::Descending)]
		sort: SortDirection,
	},
	ListTracks {
		id: usize,
	},
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub enum SortDirection {
	#[default]
	Ascending,
	Descending,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnknownSortDirectionError(String);

impl FromStr for SortDirection {
	type Err = UnknownSortDirectionError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase() {
			x if x == "a" || x == "asc" || x == "ascending" => Ok(Self::Ascending),
			x if x == "d" || x == "desc" || x == "descending" => Ok(Self::Descending),
			other => Err(UnknownSortDirectionError(other)),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let command = "album list".parse::<Command>().unwrap();
		assert_eq!(
			command,
			Command::Album(AlbumCommand::List {
				sort: SortDirection::Descending
			})
		);

		let command = "album list sort=desc".parse::<Command>().unwrap();
		assert_eq!(
			command,
			Command::Album(AlbumCommand::List {
				sort: SortDirection::Descending
			})
		);

		let command = "album list-tracks id=5".parse::<Command>().unwrap();
		assert_eq!(command, Command::Album(AlbumCommand::ListTracks { id: 5 }));
	}
}
