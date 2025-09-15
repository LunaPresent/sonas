mod context;
mod handle;
mod ui_system;

pub use context::*;
pub use handle::*;
pub use ui_system::*;

use bevy_ecs::{
	component::HookContext,
	system::SystemId,
	world::{DeferredWorld, EntityWorldMut},
};
use color_eyre::eyre;

use super::EventFlow;

type InitOutput = eyre::Result<()>;
type UpdateOutput = eyre::Result<EventFlow>;
type RenderOutput = eyre::Result<()>;

pub(crate) type InitSystemId = SystemId<InitContext, InitOutput>;
pub(crate) type UpdateSystemId<T> = SystemId<UpdateContext<'static, T>, UpdateOutput>;
pub(crate) type RenderSystemId = SystemId<RenderContext<'static>, RenderOutput>;

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
