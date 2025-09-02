mod app_event;
mod cli;
mod component;
mod config;
mod tui;

use color_eyre::eyre;

use app_event::AppEvent;
use cli::Cli;
use component::RootComponent;
use config::ConfigManager;
use tui::app::App;

#[tokio::main]
async fn main() -> eyre::Result<()> {
	color_eyre::install()?;
	let cli = Cli::new();
	let app = App::<AppEvent>::new()
		.with_component(ConfigManager::new(cli.config_path()))?
		.with_main_component(RootComponent::default())?;
	app.run().await
}
