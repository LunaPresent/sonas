mod entity_commands_ext;
mod error_handling;
mod event_handling;
mod init;
mod into_result;
mod rendering;
mod ui_component;

pub use entity_commands_ext::EntityCommandsExt;
pub use error_handling::ErrorFlow;
pub use event_handling::{AsyncEventQueue, CursorPos, EventFlow, EventQueue, Focus};
pub use rendering::{Area, Viewport};
pub use ui_component::{
	ErrorContext, InitContext, RenderContext, UiComponent, UiSystem, UpdateContext,
};

use bevy_ecs::{bundle::Bundle, entity::Entity, world::World};
use color_eyre::eyre;
use ratatui::Frame;
use tokio::sync::mpsc;

use super::event::{Dispatch, Event, EventDispatch};
use event_handling::UpdateSystemRunner;
use init::init_components;
use rendering::RenderSystemRunner;

#[derive(Debug)]
pub(crate) struct ComponentSystem<T>
where
	T: 'static,
{
	world: World,
	update_context: UpdateSystemRunner<T>,
	render_context: RenderSystemRunner,
}

impl<T> ComponentSystem<T>
where
	T: Send + Sync + 'static,
{
	pub fn new(event_sender: mpsc::UnboundedSender<EventDispatch<T>>) -> Self {
		let mut world = World::new();
		world.insert_resource(Focus::default());
		world.insert_resource(CursorPos::default());
		world.insert_resource(EventQueue::<T>::default());
		world.insert_resource(AsyncEventQueue::<T>::new(event_sender));

		ComponentSystem {
			world,
			update_context: Default::default(),
			render_context: Default::default(),
		}
	}

	pub fn add_entity(&mut self) -> Entity {
		self.world.spawn_empty().id()
	}

	pub fn add_component(&mut self, entity: Entity, component_bundle: impl Bundle) {
		self.world.entity_mut(entity).insert(component_bundle);
	}

	pub fn init(&mut self) -> eyre::Result<()> {
		self.world.flush();
		self.world.run_system_cached(init_components)?;
		Ok(())
	}

	pub fn handle_event(&mut self, ed: EventDispatch<T>) -> eyre::Result<HandleEventResult<T>> {
		let event = self.update_context.handle_event(ed, &mut self.world)?;
		self.world.run_system_cached(init_components)?;

		let requeued = self
			.world
			.get_resource_mut::<EventQueue<T>>()
			.and_then(|mut queue| queue.pop());

		Ok(HandleEventResult {
			propagated: event,
			requeued,
		})
	}

	pub fn draw(&mut self, frame: &mut Frame) -> eyre::Result<()> {
		let area = frame.area();
		self.render_context
			.render(frame.buffer_mut(), area, &mut self.world)
	}
}

pub(crate) struct HandleEventResult<T> {
	pub propagated: Option<Event<T>>,
	pub requeued: Option<EventDispatch<T>>,
}
