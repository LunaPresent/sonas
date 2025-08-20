mod entity_commands_ext;
mod event_handling;
mod init;
mod rendering;
mod ui_component;

pub use entity_commands_ext::EntityCommandsExt;
pub use event_handling::{CursorPos, EventFlow, Focus};
pub use rendering::{Area, Viewport};
pub use ui_component::{
	InitInput, InitSystem, RenderInput, RenderSystem, UpdateInput, UpdateSystem,
};

use bevy_ecs::{bundle::Bundle, entity::Entity, world::World};
use color_eyre::eyre;
use ratatui::Frame;

use super::event::{Dispatch, Event, EventDispatch};
use event_handling::UpdateContext;
use init::init_components;
use rendering::RenderContext;

#[derive(Debug)]
pub(super) struct ComponentSystem<E>
where
	E: 'static,
{
	world: World,
	update_context: UpdateContext<E>,
	render_context: RenderContext,
}

impl<E> ComponentSystem<E>
where
	E: Send + Sync + Clone + 'static,
{
	pub fn new() -> Self {
		let mut world = World::new();
		world.insert_resource(Focus::default());
		world.insert_resource(CursorPos::default());

		ComponentSystem {
			world,
			update_context: Default::default(),
			render_context: Default::default(),
		}
	}

	pub fn add_component<B>(&mut self, component_bundle: B) -> Entity
	where
		B: Bundle,
	{
		self.world.spawn(component_bundle).id()
	}

	pub fn set_focus(&mut self, target: Entity) {
		self.world.resource_mut::<Focus>().target = target;
	}

	pub fn init(&mut self) -> eyre::Result<()> {
		self.world.flush();
		self.world.run_system_cached(init_components)?;
		Ok(())
	}

	pub fn handle_event(&mut self, ed: EventDispatch<E>) -> eyre::Result<Option<Event<E>>> {
		let event = self.update_context.handle_event(ed, &mut self.world)?;
		self.world.run_system_cached(init_components)?;

		Ok(event)
	}

	pub fn draw(&mut self, frame: &mut Frame) -> eyre::Result<()> {
		let area = frame.area();
		self.render_context
			.render(frame.buffer_mut(), area, &mut self.world)
	}
}
