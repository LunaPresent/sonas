mod handle;
mod ui_system;

pub use handle::*;
pub use ui_system::*;

use bevy_ecs::{
	component::HookContext,
	entity::Entity,
	system::{In, InMut, InRef, SystemId},
	world::{DeferredWorld, EntityWorldMut},
};
use color_eyre::eyre;
use ratatui::buffer::Buffer;

use super::{Event, EventFlow};

// TODO: documentation
pub type InitInput = In<Entity>;
// TODO: documentation
pub type UpdateInput<'a, T> = (In<Entity>, InRef<'a, Event<T>>);
// TODO: documentation
pub type RenderInput<'a> = (In<Entity>, InMut<'a, Buffer>);

type InitOutput = eyre::Result<()>;
type UpdateOutput = eyre::Result<EventFlow>;
type RenderOutput = eyre::Result<()>;

pub(crate) type InitSystemId = SystemId<InitInput, InitOutput>;
pub(crate) type UpdateSystemId<T> = SystemId<UpdateInput<'static, T>, UpdateOutput>;
pub(crate) type RenderSystemId = SystemId<RenderInput<'static>, RenderOutput>;

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
