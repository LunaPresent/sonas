mod context;
mod handle;
mod ui_system;

pub use context::*;
pub(crate) use handle::*;
pub use ui_system::*;

use bevy_ecs::{
	component::HookContext,
	world::{DeferredWorld, EntityWorldMut},
};

// TODO: documentation
pub trait UiComponent {
	// TODO: documentation
	fn systems() -> impl IntoIterator<Item = UiSystem>;

	// TODO: documentation
	fn register_systems(mut world: DeferredWorld, context: HookContext) {
		let mut cmd = world.commands();
		let mut entity_cmd = cmd.entity(context.entity);
		entity_cmd.queue(move |mut entity_world: EntityWorldMut| {
			for system in Self::systems() {
				system.register(&mut entity_world);
			}
		});
	}

	// TODO: documentation
	fn unregister_systems(mut world: DeferredWorld, context: HookContext) {
		let mut cmd = world.commands();
		let mut entity_cmd = cmd.entity(context.entity);
		entity_cmd.queue(move |mut entity_world: EntityWorldMut| {
			for system in Self::systems() {
				system.unregister(&mut entity_world);
			}
		});
	}
}
