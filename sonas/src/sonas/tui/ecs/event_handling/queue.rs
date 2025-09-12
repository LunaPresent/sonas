use std::collections::VecDeque;

use bevy_ecs::resource::Resource;

use crate::tui::event::{Dispatch, Event, EventDispatch};

#[derive(Debug, Resource)]
pub struct EventQueue<T>(VecDeque<EventDispatch<T>>);

impl<T> Default for EventQueue<T> {
	fn default() -> Self {
		Self(VecDeque::default())
	}
}

impl<T> EventQueue<T> {
	pub fn push(&mut self, dispatch: Dispatch, app_event: T) {
		self.0.push_back(EventDispatch {
			dispatch,
			event: Event::App(app_event),
		});
	}

	pub(crate) fn pop(&mut self) -> Option<EventDispatch<T>> {
		self.0.pop_front()
	}
}
