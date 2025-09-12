#![allow(dead_code)]
use bevy_ecs::resource::Resource;
use tokio::sync::mpsc;

use crate::tui::event::{Dispatch, Event, EventDispatch};

#[derive(Debug, Resource)]
pub struct AsyncEventQueue<T> {
	sender: mpsc::UnboundedSender<EventDispatch<T>>,
}

impl<T> AsyncEventQueue<T> {
	pub(crate) fn new(sender: mpsc::UnboundedSender<EventDispatch<T>>) -> Self {
		Self { sender }
	}

	pub fn sender(&self) -> AsyncSender<T> {
		AsyncSender(self.sender.clone())
	}
}

#[derive(Debug, Clone)]
pub struct AsyncSender<T>(mpsc::UnboundedSender<EventDispatch<T>>);

impl<T> AsyncSender<T> {
	pub fn send(&mut self, dispatch: Dispatch, app_event: T) {
		let _ = self.0.send(EventDispatch {
			dispatch,
			event: Event::App(app_event),
		});
	}
}
