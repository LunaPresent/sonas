use bevy_ecs::{
	entity::Entity,
	query::Changed,
	system::{Commands, Query},
	world::World,
};

use crate::ecs::{InitContext, ui_component::InitSystemCollection};

pub(crate) fn init_components(
	query: Query<(&mut InitSystemCollection, Entity), Changed<InitSystemCollection>>,
	mut commands: Commands,
) {
	let mut repeat = false;
	for (mut init_handle, entity) in query {
		for &system in init_handle.iter() {
			commands.queue(move |world: &mut World| {
				world.run_system_with(system, InitContext { entity })
			});
			repeat = true;
		}
		if repeat {
			init_handle.clear();
			commands.run_system_cached(init_components);
		}
	}
}
