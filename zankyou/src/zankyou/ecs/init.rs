use bevy_ecs::{
	component::{Component, Mutable},
	entity::Entity,
	query::Added,
	system::{Commands, Query},
};

use super::UiComponent;

pub fn init_components<C, E>(query: Query<(&mut C, Entity), Added<C>>, mut commands: Commands)
where
	C: UiComponent<E> + Component<Mutability = Mutable>,
	E: Send + Sync + Clone + 'static,
{
	let mut repeat = false;
	for (mut comp, entity) in query {
		comp.init(commands.entity(entity));
		repeat = true;
	}
	if repeat {
		commands.run_system_cached(init_components::<C, E>);
	}
}
