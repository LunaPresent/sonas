mod app;
mod cli;
mod ecs;
mod event;
mod tui;

use clap::Parser;
use cli::Cli;
use color_eyre::eyre;

use crate::app::App;

#[tokio::main]
async fn main() -> eyre::Result<()> {
	color_eyre::install()?;
	let _ = Cli::parse();
	let app = App::new();
	app.run().await
}
