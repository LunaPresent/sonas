mod app_event;
mod cli;
mod component;
mod config;
mod tui;
mod util;

use color_eyre::eyre;

use app_event::AppEvent;
use cli::Cli;
use component::{ErrorReporterComponent, RootComponent};
use config::ConfigManager;

use crate::tui::app::App;

#[tokio::main]
async fn main() -> eyre::Result<()> {
	color_eyre::install()?;
	let cli = Cli::new();
	App::<AppEvent>::new()
		.with_entity(|e| {
			e.with_component(ErrorReporterComponent::new())?
				.with_component(ConfigManager::<AppEvent>::new(cli.config_path()))?
				.with_component(RootComponent::default())
		})?
		.run()
		.await
}
