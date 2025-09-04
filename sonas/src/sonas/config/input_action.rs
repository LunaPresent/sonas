use serde::{Deserialize, Serialize};

use crate::{AppEvent, app_event::Direction, tui::config::Action};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InputAction {
	Quit,
	CursorUp,
	CursorDown,
	CursorLeft,
	CursorRight,
	ScrollDown,
	ScrollUp,
	ScrollHalfPageDown,
	ScrollHalfPageUp,
	ScrollFullPageDown,
	ScrollFullPageUp,
}

impl Action for InputAction {
	type AppEvent = AppEvent;

	fn app_event(&self) -> Self::AppEvent {
		match *self {
			InputAction::Quit => AppEvent::Quit,
			InputAction::CursorUp => AppEvent::MoveCursor(Direction::Up),
			InputAction::CursorDown => AppEvent::MoveCursor(Direction::Down),
			InputAction::CursorLeft => AppEvent::MoveCursor(Direction::Left),
			InputAction::CursorRight => AppEvent::MoveCursor(Direction::Right),
			InputAction::ScrollDown => AppEvent::ScrollBy {
				direction: Direction::Down,
				amount: 1,
			},
			InputAction::ScrollUp => AppEvent::ScrollBy {
				direction: Direction::Up,
				amount: 1,
			},
			InputAction::ScrollHalfPageDown => AppEvent::ScrollByRelative {
				direction: Direction::Down,
				fraction: 0.5,
			},
			InputAction::ScrollHalfPageUp => AppEvent::ScrollByRelative {
				direction: Direction::Up,
				fraction: 0.5,
			},
			InputAction::ScrollFullPageDown => AppEvent::ScrollByRelative {
				direction: Direction::Down,
				fraction: 1.,
			},
			InputAction::ScrollFullPageUp => AppEvent::ScrollByRelative {
				direction: Direction::Up,
				fraction: 1.,
			},
		}
	}
}
