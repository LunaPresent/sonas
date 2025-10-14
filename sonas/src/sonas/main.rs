mod app_event;
mod cli;
mod component;
mod config;
mod tui;
mod util;

use std::time::Duration;

use color_eyre::eyre;

use app_event::AppEvent;
use cli::Cli;
use component::*;
use config::ConfigManager;

use crate::{tui::app::App, util::OctDirection};

#[tokio::main]
async fn main() -> eyre::Result<()> {
	color_eyre::install()?;
	let cli = Cli::new();
	App::new()
		.with_tick_interval(Duration::from_secs_f64(0.25))
		.with_frame_interval(Duration::from_secs_f64(1. / 144.))
		.with_entity(|e| {
			e.with_component(ErrorReporterComponent::new())?
				.with_component(ConfigManager::new(cli.config_path()))?
				.with_component(RootComponent::default())
		})?
		.with_entity(|e| e.with_component(FpsComponent::new(OctDirection::UpRight)))?
		.run()
		.await
}
