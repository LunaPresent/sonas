use std::{
	marker::PhantomData,
	path::PathBuf,
	sync::{
		Arc,
		atomic::{AtomicBool, Ordering},
	},
};

use ::config::{Config, ConfigError, File, FileFormat};
use bevy_ecs::{
	component::Component,
	system::{Commands, Query, ResMut},
};
use notify::{Error as NotifyError, RecommendedWatcher, RecursiveMode, Watcher as _};
use thiserror::Error;

use super::{AppConfig, Keys, Settings, Theme};
use crate::tui::{
	ecs::{EventFlow, InitContext, UiComponent, UiSystem, UpdateContext},
	event::Event,
};

#[derive(Debug, Error)]
pub enum ConfigManagerError {
	#[error("failed to initialise file watch on config file")]
	FailedFileWatch(#[from] NotifyError),
	#[error("failed to parse config file")]
	FailedToParse(#[from] ConfigError),
	#[error("failed to convert config file path to string")]
	InvalidPath,
}

#[derive(Debug, Component)]
#[component(on_add = Self::register_systems)]
#[component(on_remove = Self::unregister_systems)]
pub struct ConfigManager<T>
where
	T: Send + Sync + 'static,
{
	file_path: Option<PathBuf>,
	watcher: Option<RecommendedWatcher>,
	changed: Arc<AtomicBool>,
	phantom_data: PhantomData<T>,
}

impl<T> UiComponent for ConfigManager<T>
where
	T: Send + Sync + 'static,
{
	fn systems() -> impl IntoIterator<Item = UiSystem> {
		[UiSystem::new(Self::init), UiSystem::new(Self::update)]
	}
}

impl<T> ConfigManager<T>
where
	T: Send + Sync + 'static,
{
	pub fn new(file_path: Option<PathBuf>) -> Self {
		Self {
			file_path,
			watcher: None,
			changed: Arc::new(AtomicBool::new(false)),
			phantom_data: PhantomData,
		}
	}

	fn init(
		context: InitContext,
		mut query: Query<&mut Self>,
		mut cmd: Commands,
	) -> Result<(), ConfigManagerError> {
		let mut comp = query
			.get_mut(context.entity)
			.expect("Self type component should be present on the entity");

		let config = comp.parse_config()?;

		cmd.insert_resource(config.keys);
		cmd.insert_resource(config.theme);
		cmd.insert_resource(config.settings);

		if let Some(file_path) = comp
			.file_path
			.as_ref()
			.and_then(|p| p.parent())
			.filter(|p| p.exists())
		{
			let changed = comp.changed.clone();
			let mut watcher = notify::recommended_watcher(move |_event| {
				changed.store(true, Ordering::Relaxed);
			})?;
			watcher.watch(file_path, RecursiveMode::Recursive)?;
			comp.watcher = Some(watcher);
		}

		Ok(())
	}

	fn update(
		context: UpdateContext<T>,
		query: Query<&Self>,
		mut keys: ResMut<Keys>,
		mut theme: ResMut<Theme>,
		mut settings: ResMut<Settings>,
	) -> Result<EventFlow, ConfigManagerError> {
		let comp = query
			.get(context.entity)
			.expect("Self type component should be present on the entity");
		if let Event::Tick(_) = context.event
			&& comp
				.changed
				.compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed)
				.is_ok()
		{
			let config = comp.parse_config()?;
			*keys = config.keys;
			*theme = config.theme;
			*settings = config.settings;
		}
		Ok(EventFlow::Propagate)
	}

	fn parse_config(&self) -> Result<AppConfig, ConfigManagerError> {
		let mut builder = Config::builder().add_source(File::from_str(
			include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/.config/config.toml")),
			FileFormat::Toml,
		));
		if let Some(file_path) = &self.file_path {
			builder = builder.add_source(
				File::with_name(
					file_path
						.as_path()
						.to_str()
						.ok_or(ConfigManagerError::InvalidPath)?,
				)
				.required(false),
			);
		}

		Ok(builder.build()?.try_deserialize::<AppConfig>()?)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct AppEvent;

	#[test]
	fn default_config_ok() {
		let config_manager = ConfigManager::<AppEvent>::new(None);
		assert!(config_manager.parse_config().is_ok());
	}
}
