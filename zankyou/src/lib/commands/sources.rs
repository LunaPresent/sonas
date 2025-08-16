use zankyou_parser::Subcommand;

#[derive(Debug, Clone, Subcommand)]
pub enum SourceCommand {
	List,
}

impl SourceCommand {
	pub fn execute(&self) -> String {
		match self {
			Self::List => "meow!".to_string(),
		}
	}
}
