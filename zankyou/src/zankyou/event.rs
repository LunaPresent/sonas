mod queue;
mod sender;
mod task;

use bevy_ecs::entity::Entity;
use crossterm::event::{KeyEvent, MouseEvent};

pub use queue::EventQueue;
pub use sender::EventSender;

#[derive(Debug, Clone)]
pub struct EventDispatch<E> {
	pub dispatch: Dispatch,
	pub event: Event<E>,
}

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

#[derive(Debug, Clone)]
pub enum Event<E> {
	Tick,
	App(E),
	FocusGained,
	FocusLost,
	Key(KeyEvent),
	Mouse(MouseEvent),
	Paste(String),
	Resize { width: u16, height: u16 },
}
