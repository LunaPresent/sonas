pub use counter::CounterComponent;
use derive_more::From;
use ratatui::layout::Rect;
pub use root::RootComponent;

use crate::event::{AppEvent, Event};

mod counter;
mod root;

pub enum EventFlow {
	/// Signifies the event should not be further handled by other components
	Consume,
	/// Signifies the event should by passed through to the parent component
	Propagate,
}

#[derive(Debug, From)]
pub enum GenericComponentRef<'a> {
	RootComponent(&'a RootComponent),
	CounterComponent(&'a CounterComponent),
}

#[derive(Debug, From)]
pub enum GenericComponentRefMut<'a> {
	RootComponent(&'a mut RootComponent),
	CounterComponent(&'a mut CounterComponent),
}

impl<'a> GenericComponentRefMut<'a> {
	fn handle_event(self, event: &AppEvent) -> EventFlow {
		match self {
			GenericComponentRefMut::RootComponent(c) => c.handle_event(event),
			GenericComponentRefMut::CounterComponent(c) => c.handle_event(event),
		}
	}

	fn handle_broadcast(self, event: &AppEvent) {
		match self {
			GenericComponentRefMut::RootComponent(c) => c.handle_broadcast(event),
			GenericComponentRefMut::CounterComponent(c) => c.handle_broadcast(event),
		}
	}

	fn children(self) -> &'a [GenericComponentRef<'a>] {
		match self {
			GenericComponentRefMut::RootComponent(c) => c.children(),
			GenericComponentRefMut::CounterComponent(c) => c.children(),
		}
	}

	fn children_mut(self) -> &'a [GenericComponentRefMut<'a>] {
		match self {
			GenericComponentRefMut::RootComponent(c) => c.children_mut(),
			GenericComponentRefMut::CounterComponent(c) => c.children_mut(),
		}
	}

	fn find_focus(self) -> GenericComponentRefMut<'a> {
		match self {
			GenericComponentRefMut::RootComponent(c) => c.find_focus(),
			GenericComponentRefMut::CounterComponent(c) => c.find_focus(),
		}
	}
}

trait Component
where
	for<'a> GenericComponentRef<'a>: From<&'a Self>,
	for<'a> GenericComponentRefMut<'a>: From<&'a mut Self>,
{
	/// Handle app event
	fn handle_event(&mut self, event: &AppEvent) -> EventFlow {
		let _ = event;
		EventFlow::Propagate
	}

	/// Handle broadcast app event
	fn handle_broadcast(&mut self, event: &AppEvent) {
		let _ = self.handle_event(event);
	}

	/// Returns a slice of this component's child components
	fn children<'a>(&self) -> &[GenericComponentRef<'a>] {
		&[]
	}

	/// Returns a slice of this component's child components
	fn children_mut<'a>(&self) -> &[GenericComponentRefMut<'a>] {
		&[]
	}

	/// Returns the child component to follow in order to get to the focussed component
	///
	/// Return reference to self if this is the focussed component
	fn find_focus<'a>(&'a mut self) -> GenericComponentRefMut<'a> {
		GenericComponentRefMut::from(self)
	}

	/// Returns the bounding area of this component
	///
	/// If this returns `None`, the area will be the same as the parent component's
	fn area(&self) -> Option<Rect> {
		None
	}

	/// Returns the click box of this component
	///
	/// If this returns `None`, the click box will be the same as the parent component's
	/// A return value `Some(Rect::ZERO)` may be used to disable click behaviour on this component
	fn click_box(&self) -> Option<Rect> {
		self.area()
	}
}

#[derive(Debug)]
pub struct ComponentSystem<C>
where
	for<'a> GenericComponentRef<'a>: From<&'a C>,
	for<'a> GenericComponentRefMut<'a>: From<&'a mut C>,
{
	root: C,
}

impl<C> ComponentSystem<C>
where
	for<'a> GenericComponentRef<'a>: From<&'a C>,
	for<'a> GenericComponentRefMut<'a>: From<&'a mut C>,
{
	pub fn new(root_component: C) -> ComponentSystem<C> {
		ComponentSystem {
			root: root_component,
		}
	}

	pub fn handle_event(&mut self, event: &Event) -> Option<&AppEvent> {
		match event {
			Event::Crossterm(event) => {
				// TODO: handle terminal event
				None
			}
			Event::Input(event) => {
				todo!()
				//Some(event) if propagated
			}
			Event::Broadcast(event) => {
				// TODO: handle broadcast
				None
			}
		}
	}

	fn handle_input_event(c: GenericComponentRefMut, event: &AppEvent) -> bool {
		todo!()
	}
}
