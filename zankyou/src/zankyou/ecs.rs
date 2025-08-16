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

use bevy_ecs::{component::Component, entity::Entity, world::World};
use color_eyre::eyre;
use ratatui::Frame;

use crate::event::{Event, EventDispatch};
use event_handling::UpdateContext;
use init::init_components;
use rendering::RenderContext;

#[derive(Debug)]
pub struct ComponentSystem<E>
where
	E: 'static,
{
	world: World,
	root_entity: Entity,
	update_context: UpdateContext<E>,
	render_context: RenderContext,
}

impl<E> ComponentSystem<E>
where
	E: Send + Sync + Clone + 'static,
{
	pub fn new<C>() -> Self
	where
		C: Component + Default,
	{
		let mut world = World::new();
		let root_entity = world.spawn(C::default()).id();
		world.insert_resource(Focus {
			target: root_entity,
		});
		world.insert_resource(CursorPos::default());

		world.flush();

		world
			.run_system_cached(init_components)
			.expect("Unexpected error in component initialisation");

		ComponentSystem {
			world,
			root_entity,
			update_context: Default::default(),
			render_context: Default::default(),
		}
	}

	pub fn handle_event(&mut self, ed: EventDispatch<E>) -> eyre::Result<Option<Event<E>>> {
		let event = self
			.update_context
			.handle_event(ed, &mut self.world, self.root_entity)?;
		self.world.run_system_cached(init_components)?;

		Ok(event)
	}

	pub fn draw(&mut self, frame: &mut Frame) -> eyre::Result<()> {
		**self
			.world
			.get_mut::<Area>(self.root_entity)
			.expect("Root element must have an Area component") = frame.area();

		self.render_context
			.render(frame.buffer_mut(), &mut self.world, self.root_entity)
	}
}
