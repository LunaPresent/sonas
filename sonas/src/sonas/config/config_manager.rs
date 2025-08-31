use std::{fs, path::PathBuf};

use bevy_ecs::{
	component::Component,
	system::{Commands, In, Query},
};
use color_eyre::eyre;

use super::Config;
use crate::tui::ecs::{InitInput, InitSystem};

#[derive(Debug, Component)]
#[require(InitSystem::new(Self::init))]
pub struct ConfigManager {
	file_path: PathBuf,
}

impl ConfigManager {
	pub fn new(file_path: impl Into<PathBuf>) -> Self {
		Self {
			file_path: file_path.into(),
		}
	}

	fn init(In(entity): InitInput, query: Query<&Self>, mut cmd: Commands) -> eyre::Result<()> {
		let comp = query.get(entity)?;

		let config: Config = toml::from_str(&fs::read_to_string(&comp.file_path)?)?;

		cmd.insert_resource(config.keys);

		Ok(())
	}
}
