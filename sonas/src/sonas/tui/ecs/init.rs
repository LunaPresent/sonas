use bevy_ecs::{
	entity::Entity,
	query::Changed,
	system::{Commands, Query},
	world::World,
};

use crate::tui::ecs::ui_component::{InitHandle, NextInitMarker, SystemHandle};

pub(crate) fn init_components(
	query: Query<(&InitHandle, &mut NextInitMarker, Entity), Changed<InitHandle>>,
	mut commands: Commands,
) {
	let mut repeat = false;
	for (init_handle, mut next_init, entity) in query {
		for &system in &init_handle.systems()[**next_init..] {
			commands.queue(move |world: &mut World| world.run_system_with(system, entity));
			**next_init += 1;
			repeat = true;
		}
		if repeat {
			commands.run_system_cached(init_components);
		}
	}
}
