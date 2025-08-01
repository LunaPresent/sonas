use bevy_ecs::{
	bundle::Bundle, hierarchy::ChildOf, relationship::Relationship, system::EntityCommands,
};

pub trait EntityCommandsExt {
	fn spawn_child(&mut self, bundle: impl Bundle) -> EntityCommands;
	fn spawn_related<R: Relationship>(&mut self, bundle: impl Bundle) -> EntityCommands;
}

impl EntityCommandsExt for EntityCommands<'_> {
	fn spawn_child(&mut self, bundle: impl Bundle) -> EntityCommands {
		self.spawn_related::<ChildOf>(bundle)
	}

	fn spawn_related<R: Relationship>(&mut self, bundle: impl Bundle) -> EntityCommands {
		let parent = self.id();
		self.commands_mut().spawn((bundle, R::from(parent)))
	}
}
