use bevy_ecs::{
	entity::Entity,
	query::Added,
	system::{Commands, Query},
};

use crate::ecs::ui_component::InitHandle;

pub fn init_components(
	query: Query<(&InitHandle, Entity), Added<InitHandle>>,
	mut commands: Commands,
) {
	let mut repeat = false;
	for (handle, entity) in query {
		commands.run_system_with(**handle, entity);
		repeat = true;
	}
	if repeat {
		commands.run_system_cached(init_components);
	}
}
