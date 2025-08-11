use bevy_ecs::{
	bundle::Bundle,
	entity::Entity,
	hierarchy::ChildOf,
	relationship::Relationship,
	system::{EntityCommands, In, IntoSystem},
	world::EntityWorldMut,
};

pub trait EntityCommandsExt {
	fn spawn_child(&mut self, bundle: impl Bundle) -> EntityCommands<'_>;
	fn spawn_related<R: Relationship>(&mut self, bundle: impl Bundle) -> EntityCommands<'_>;
	fn run_system_cached<O, M, S>(&mut self, system: S)
	where
		O: 'static,
		M: 'static,
		S: IntoSystem<In<Entity>, O, M> + Send + 'static;
}

impl EntityCommandsExt for EntityCommands<'_> {
	fn spawn_child(&mut self, bundle: impl Bundle) -> EntityCommands<'_> {
		self.spawn_related::<ChildOf>(bundle)
	}

	fn spawn_related<R: Relationship>(&mut self, bundle: impl Bundle) -> EntityCommands<'_> {
		let parent = self.id();
		self.commands_mut().spawn((bundle, R::from(parent)))
	}

	fn run_system_cached<O, M, S>(&mut self, system: S)
	where
		O: 'static,
		M: 'static,
		S: IntoSystem<In<Entity>, O, M> + Send + 'static,
	{
		let entity = self.id();
		self.queue(move |ew: EntityWorldMut| {
			ew.into_world_mut().run_system_cached_with(system, entity)
		});
	}
}
