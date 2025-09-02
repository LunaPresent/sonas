use std::path::PathBuf;

use ::config::{Config, File};
use bevy_ecs::{
	component::Component,
	system::{Commands, In, Query},
};
use color_eyre::eyre::{self, OptionExt};
use config::FileFormat;

use super::AppConfig;
use crate::tui::ecs::{InitInput, InitSystem};

#[derive(Debug, Component)]
#[require(InitSystem::new(Self::init))]
pub struct ConfigManager {
	file_path: Option<PathBuf>,
}

impl ConfigManager {
	pub fn new(file_path: Option<PathBuf>) -> Self {
		Self { file_path }
	}

	fn init(In(entity): InitInput, query: Query<&Self>, mut cmd: Commands) -> eyre::Result<()> {
		let comp = query.get(entity)?;

		let config = comp.parse_config()?;

		cmd.insert_resource(config.keys);
		cmd.insert_resource(config.theme);

		Ok(())
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

	#[test]
	fn default_config_ok() {
		let config_manager = ConfigManager::new(None);
		assert!(config_manager.parse_config().is_ok());
	}
}
