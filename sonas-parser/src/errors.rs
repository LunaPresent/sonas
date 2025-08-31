use thiserror::Error;

#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum ParseCommandError {
	#[error("given input was empty")]
	EmptyString,
	#[error("no subcommand was specified")]
	NoSubcommand,
	#[error("unknown command category '{0}'")]
	UnknownCategory(String),
	#[error("unknown subcommand '{0}'")]
	UnknownSubcommand(String),
	#[error("invalid value specified for argument '{0}'")]
	InvalidArgument(String),
	#[error("duplicate argument specified '{0}'")]
	DuplicateArgument(String),
	#[error("unexpected argument '{0}' was specified")]
	UnexpectedArgument(String),
	#[error("missing argument '{0}'")]
	MissingArgument(String),
}
