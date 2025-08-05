use derive_more::From;
use ratatui::{DefaultTerminal, widgets::WidgetRef};

use crate::event::{Dispatch, Event, EventDispatch};
pub use counter::CounterComponent;
pub use root::RootComponent;

mod counter;
mod root;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EventFlow {
	/// Signifies the event should not be further handled by other components
	Consume,
	/// Signifies the event should by passed through to the parent component
	Propagate,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum FollowResult {
	Consume,
	Propagate(Ref),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, From)]
enum Ref {
	RootComponent(*mut RootComponent),
	CounterComponent(*mut CounterComponent),
}

impl Ref {
	fn handle_input_event_impl<C: Component>(component: *mut C, event: &Event) -> EventFlow
	where
		Ref: From<*mut C>,
	{
		unsafe {
			if let FollowResult::Propagate(child) = (*component).follow_focus() {
				if Self::handle_input_event(child, event) == EventFlow::Propagate {
					(*component).handle_event(event)
				} else {
					EventFlow::Consume
				}
			} else {
				(*component).handle_event(event)
			}
		}
	}

	fn handle_broadcast_event_impl<C: Component>(component: *mut C, event: &Event)
	where
		Ref: From<*mut C>,
	{
		unsafe {
			(*component).handle_broadcast(event);
			for c in (*component).children() {
				c.handle_broadcast_event(event);
			}
		}
	}

	fn handle_input_event(self, event: &Event) -> EventFlow {
		match self {
			Self::RootComponent(c) => Self::handle_input_event_impl(c, event),
			Self::CounterComponent(c) => Self::handle_input_event_impl(c, event),
		}
	}

	fn handle_broadcast_event(self, event: &Event) {
		match self {
			Self::RootComponent(c) => Self::handle_broadcast_event_impl(c, event),
			Self::CounterComponent(c) => Self::handle_broadcast_event_impl(c, event),
		}
	}
}

trait Component
where
	Ref: From<*mut Self>,
{
	/// Handle app event
	fn handle_event(&mut self, event: &Event) -> EventFlow {
		let _ = event;
		EventFlow::Propagate
	}

	/// Handle broadcast app event
	fn handle_broadcast(&mut self, event: &Event) {
		let _ = self.handle_event(event);
	}

	/// Returns a slice of this component's child components
	fn children(&mut self) -> impl Iterator<Item = Ref> {
		std::iter::empty()
	}

	/// Returns the child component to follow in order to get to the focussed component
	fn follow_focus<'a>(&'a mut self) -> FollowResult {
		FollowResult::Consume
	}

	/// Returns the child component to follow in order to get to the focussed component
	fn follow_click<'a>(&'a mut self, x: u16, y: u16) -> FollowResult {
		let _ = x;
		let _ = y;
		FollowResult::Consume
	}
}

#[derive(Debug)]
pub struct ComponentSystem<C> {
	root: C,
}

#[allow(private_bounds)]
impl<C> ComponentSystem<C>
where
	Ref: From<*mut C>,
	C: WidgetRef,
{
	pub fn new(root_component: C) -> ComponentSystem<C> {
		ComponentSystem {
			root: root_component,
		}
	}

	pub fn handle_event(&mut self, ed: &EventDispatch) -> EventFlow {
		match ed.dispatch() {
			Dispatch::System => {
				// TODO: handle terminal event
				EventFlow::Propagate
			}
			Dispatch::Input => Ref::from(&mut self.root).handle_input_event(ed.event()),
			Dispatch::Broadcast => {
				Ref::from(&mut self.root).handle_broadcast_event(ed.event());
				EventFlow::Propagate
			}
		}
	}

	pub fn draw(&self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
		terminal.draw(|frame| frame.render_widget(&self.root, frame.area()))?;
		Ok(())
	}
}
