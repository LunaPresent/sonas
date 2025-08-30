use bevy_ecs::{
	component::Component,
	system::{Commands, In, Query},
};
use color_eyre::eyre;

use super::Config;
use crate::tui::ecs::{InitInput, InitSystem};

#[derive(Debug)]
enum ConfigSource<C> {
	Value(Option<C>),
	// TODO: add another source type to be able to do file watch
}

#[derive(Debug, Component)]
#[require(InitSystem::new(Self::init))]
pub struct ConfigManager<C>
where
	C: Send + Sync + 'static,
{
	source: ConfigSource<C>,
}

impl<C> ConfigManager<C>
where
	C: Send + Sync + 'static,
{
	pub fn from_value(config: C) -> Self {
		Self {
			source: ConfigSource::Value(Some(config)),
		}
	}

	fn init(
		In(entity): InitInput,
		mut query: Query<&mut Self>,
		mut cmd: Commands,
	) -> eyre::Result<()> {
		let mut comp = query.get_mut(entity)?;
		let config = match &mut comp.source {
			ConfigSource::Value(value) => value
				.take()
				.expect("init should never be run more than once, so this should never be None"),
		};

		cmd.insert_resource(Config(config));

		Ok(())
	}
}
