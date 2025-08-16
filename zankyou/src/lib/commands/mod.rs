pub mod sources;

use sources::SourceCommand;
use zankyou_parser::CommandCategory;

#[derive(Debug, Clone, CommandCategory)]
pub enum Command {
	Source(SourceCommand),
}

impl Command {
	pub fn execute(&self) -> String {
		match self {
			Self::Source(command) => command.execute(),
		}
	}
}
