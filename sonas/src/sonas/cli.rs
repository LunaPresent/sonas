use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version = version(), about)]
pub struct Cli;

const VERSION_MESSAGE: &str = concat!(
	env!("CARGO_PKG_VERSION"),
	"-",
	env!("VERGEN_GIT_DESCRIBE"),
	" (",
	env!("VERGEN_BUILD_DATE"),
	")"
);

pub fn version() -> String {
	let author = clap::crate_authors!();

	format!(
		"\
{VERSION_MESSAGE}

Authors: {author}"
	)
}
