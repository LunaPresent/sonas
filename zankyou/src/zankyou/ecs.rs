mod entity_commands_ext;
mod event_handling;
mod init;
mod rendering;
mod ui_component;

pub use entity_commands_ext::EntityCommandsExt;
pub use event_handling::{EventFlow, Focus};
use event_handling::{
	handle_broadcast_event, handle_input_event, handle_mouse_event, handle_target_event,
};
use init::init_components;
pub use rendering::{Area, Viewport};
pub use ui_component::UiComponent;

use std::marker::PhantomData;

use bevy_ecs::{
	component::{Component, Mutable},
	entity::Entity,
	system::RunSystemOnce,
	world::World,
};
use color_eyre::eyre;
use ratatui::Frame;

use crate::event::{Dispatch, Event, EventDispatch};
use rendering::render;

#[derive(Debug)]
pub struct ComponentSystem<C, E> {
	world: World,
	root_entity: Entity,
	phantom_data: PhantomData<(C, E)>,
}

impl<C, E> ComponentSystem<C, E>
where
	C: UiComponent<E> + Component<Mutability = Mutable> + Default,
	E: Send + Sync + Clone + 'static,
{
	pub fn new() -> ComponentSystem<C, E> {
		let mut world = World::new();
		let root_entity = world.spawn(C::default()).id();
		world.insert_resource(Focus {
			target: root_entity,
		});
		world.flush();

		ComponentSystem {
			world,
			root_entity,
			phantom_data: PhantomData,
		}
	}

	pub fn handle_event<'a>(&mut self, ed: EventDispatch<E>) -> eyre::Result<Option<Event<E>>> {
		self.world.run_system_cached(init_components::<C, E>)?;

		match ed.dispatch {
			Dispatch::Input => Ok(self
				.world
				.run_system_once_with(handle_input_event::<C, _>, ed.event)?),
			Dispatch::Broadcast => Ok(Some(
				self.world
					.run_system_once_with(handle_broadcast_event::<C, _>, ed.event)?,
			)),
			Dispatch::Cursor { x, y } => Ok(self.world.run_system_cached_with(
				handle_mouse_event::<C, _>,
				(ed.event, self.root_entity, x, y),
			)?),
			Dispatch::Target(target) => Ok(self
				.world
				.run_system_once_with(handle_target_event::<C, _>, (ed.event, target))?),
		}
	}

	pub fn draw(&mut self, frame: &mut Frame) {
		self.world
			.run_system_cached(init_components::<C, E>)
			.expect("Something broke while initialising comoponents");

		self.world
			.get_mut::<Area>(self.root_entity)
			.expect("Root element must have an Area component")
			.0 = frame.area();
		self.world
			.run_system_once_with(render::<C, E>, (self.root_entity, frame.buffer_mut()))
			.expect("Something broke while rendering");
	}
}
