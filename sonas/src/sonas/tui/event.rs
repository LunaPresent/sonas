mod error;
mod system;

pub use error::EventError;
pub use system::EventSystem;

use std::time::Duration;

use bevy_ecs::entity::Entity;
use crossterm::event::{KeyEvent, MouseEvent};

#[derive(Debug, Clone)]
pub struct EventDispatch<T> {
	pub dispatch: Dispatch,
	pub event: Event<T>,
}

impl<T> EventDispatch<T> {
	pub fn new(dispatch: Dispatch, event: Event<T>) -> Self {
		Self { dispatch, event }
	}
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dispatch {
	Input,
	Broadcast,
	Cursor { x: u16, y: u16 },
	Target(Entity),
}

pub trait AppEvent {
	fn is_quit(&self) -> bool;
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Event<T> {
	Tick(Duration),
	App(T),
	FocusGained,
	FocusLost,
	Key(KeyEvent),
	Mouse(MouseEvent),
	Paste(String),
	Resize { width: u16, height: u16 },
}
