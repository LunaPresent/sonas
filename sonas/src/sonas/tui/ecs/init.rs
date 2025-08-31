use bevy_ecs::{
	entity::Entity,
	query::Added,
	system::{Commands, Query},
	world::World,
};
use color_eyre::eyre;

use super::ui_component::InitHandle;

pub(super) fn init_components(
	query: Query<(&InitHandle, Entity), Added<InitHandle>>,
	mut commands: Commands,
) {
	let mut repeat = false;
	for (&handle, entity) in query {
		commands.queue(move |world: &mut World| -> eyre::Result<()> {
			world.run_system_with(*handle, entity)?
		});
		repeat = true;
	}
	if repeat {
		commands.run_system_cached(init_components);
	}
}
