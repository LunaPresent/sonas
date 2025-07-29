use clap::Parser;
use cli::Cli;
use color_eyre::Result;

use crate::app::App;

mod app;
mod cli;
mod event;
mod ui;

#[tokio::main]
async fn main() -> Result<()> {
	color_eyre::install()?;
	let terminal = ratatui::init();
	let _ = Cli::parse();
	let app = App::new();
	let result = app.run(terminal).await;
	ratatui::restore();
	result
}
