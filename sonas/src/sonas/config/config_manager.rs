use std::{
	marker::PhantomData,
	path::PathBuf,
	sync::{
		Arc,
		atomic::{AtomicBool, Ordering},
	},
};

use ::config::{Config, File};
use bevy_ecs::{
	component::Component,
	system::{Commands, In, InRef, Query, ResMut},
};
use color_eyre::eyre::{self, OptionExt};
use config::FileFormat;
use notify::{RecommendedWatcher, RecursiveMode, Watcher as _};

use super::AppConfig;
use crate::{
	config::{Keys, Theme},
	tui::{
		ecs::{EventFlow, InitInput, InitSystem, UpdateInput, UpdateSystem},
		event::Event,
	},
};

#[derive(Debug, Component)]
#[require(InitSystem::new(Self::init), UpdateSystem::<E>::new(Self::update))]
pub struct ConfigManager<E>
where
	E: Send + Sync + 'static,
{
	file_path: Option<PathBuf>,
	watcher: Option<RecommendedWatcher>,
	changed: Arc<AtomicBool>,
	phantom_data: PhantomData<E>,
}

impl<E> ConfigManager<E>
where
	E: Send + Sync + 'static,
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
		In(entity): InitInput,
		mut query: Query<&mut Self>,
		mut cmd: Commands,
	) -> eyre::Result<()> {
		let mut comp = query.get_mut(entity)?;

		let config = comp.parse_config()?;

		cmd.insert_resource(config.keys);
		cmd.insert_resource(config.theme);

		if let Some(file_path) = comp.file_path.as_ref().and_then(|p| p.parent()) {
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
		(In(entity), InRef(event)): UpdateInput<E>,
		query: Query<&Self>,
		mut keys: ResMut<Keys>,
		mut theme: ResMut<Theme>,
	) -> eyre::Result<EventFlow> {
		let comp = query.get(entity)?;
		if let Event::Tick(_) = event
			&& comp
				.changed
				.compare_exchange(true, false, Ordering::Release, Ordering::Relaxed)
				.is_ok()
		{
			let config = comp.parse_config()?;
			*keys = config.keys;
			*theme = config.theme;
		}
		Ok(EventFlow::Propagate)
	}

	fn parse_config(&self) -> eyre::Result<AppConfig> {
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
						.ok_or_eyre("failed to convert config file path to string")?,
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
