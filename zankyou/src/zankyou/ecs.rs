mod entity_commands_ext;
mod event_handling;
mod init;
mod rendering;
mod ui_component;

pub use entity_commands_ext::EntityCommandsExt;
pub use event_handling::{CursorPos, EventFlow, Focus};
pub use rendering::{Area, Viewport};
pub use ui_component::{InitSystem, RenderSystem, UpdateSystem};

use std::marker::PhantomData;

use bevy_ecs::{component::Component, entity::Entity, system::RunSystemOnce, world::World};
use color_eyre::eyre;
use ratatui::Frame;

use crate::{
	ecs::rendering::find_render_targets,
	event::{Dispatch, Event, EventDispatch},
};
use event_handling::{
	UpdateContext, find_broadcast_entities, find_cursor_entities, find_input_entities,
	find_target_entities,
};
use init::init_components;
use rendering::RenderContext;

#[derive(Debug)]
pub struct ComponentSystem<E>
where
	E: 'static,
{
	world: World,
	root_entity: Entity,
	update_targets: Vec<UpdateContext<E>>,
	render_targets: Vec<RenderContext>,
	phantom_data: PhantomData<E>,
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
			update_targets: Vec::new(),
			render_targets: Vec::new(),
			phantom_data: PhantomData,
		}
	}

	pub fn handle_event(&mut self, ed: EventDispatch<E>) -> eyre::Result<Option<Event<E>>> {
		const ERROR_MSG: &'static str = "Unexpected error in event handling system";

		self.update_targets.clear();

		match ed.dispatch {
			Dispatch::Input => self
				.world
				.run_system_once_with(find_input_entities, &mut self.update_targets)
				.expect(ERROR_MSG),
			Dispatch::Broadcast => self
				.world
				.run_system_once_with(find_broadcast_entities, &mut self.update_targets)
				.expect(ERROR_MSG),
			Dispatch::Cursor { x, y } => self
				.world
				.run_system_cached_with(
					find_cursor_entities,
					(&mut self.update_targets, &ed.event, self.root_entity, x, y),
				)
				.expect(ERROR_MSG),
			Dispatch::Target(target) => self
				.world
				.run_system_once_with(find_target_entities, (&mut self.update_targets, target))
				.expect(ERROR_MSG),
		}

		let event = match ed.dispatch {
			Dispatch::Broadcast => {
				for context in self.update_targets.iter() {
					let _ = self
						.world
						.run_system_with(context.system, (context.entity, &ed.event))?;
				}
				Some(ed.event)
			}
			_ => {
				let mut full_propagate = true;
				for context in self.update_targets.iter() {
					let flow = self
						.world
						.run_system_with(context.system, (context.entity, &ed.event))?;
					if flow == EventFlow::Consume {
						full_propagate = false;
						break;
					}
				}
				if full_propagate { Some(ed.event) } else { None }
			}
		};

		self.world.run_system_cached(init_components)?;

		Ok(event)
	}

	pub fn draw(&mut self, frame: &mut Frame) {
		**self
			.world
			.get_mut::<Area>(self.root_entity)
			.expect("Root element must have an Area component") = frame.area();

		self.render_targets.clear();

		self.world
			.run_system_once_with(
				find_render_targets,
				(&mut self.render_targets, self.root_entity),
			)
			.expect("?");

		for context in self.render_targets.iter() {
			self.world
				.run_system_with(context.system, (context.entity, frame.buffer_mut()))
				.expect("?");
		}
	}
}
