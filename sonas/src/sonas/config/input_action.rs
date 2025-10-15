use oprabeli::config::Action;
use serde::{Deserialize, Serialize};

use crate::{AppEvent, util::QuadDirection};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InputAction {
	Quit,
	Suspend,
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
	TestError,
}

impl Action for InputAction {
	type AppEvent = AppEvent;

	fn app_event(&self) -> Self::AppEvent {
		match *self {
			InputAction::Quit => AppEvent::Quit,
			InputAction::Suspend => AppEvent::Suspend,
			InputAction::CursorUp => AppEvent::MoveCursor(QuadDirection::Up),
			InputAction::CursorDown => AppEvent::MoveCursor(QuadDirection::Down),
			InputAction::CursorLeft => AppEvent::MoveCursor(QuadDirection::Left),
			InputAction::CursorRight => AppEvent::MoveCursor(QuadDirection::Right),
			InputAction::ScrollDown => AppEvent::ScrollBy {
				direction: QuadDirection::Down,
				amount: 1,
			},
			InputAction::ScrollUp => AppEvent::ScrollBy {
				direction: QuadDirection::Up,
				amount: 1,
			},
			InputAction::ScrollHalfPageDown => AppEvent::ScrollByRelative {
				direction: QuadDirection::Down,
				fraction: 0.5,
			},
			InputAction::ScrollHalfPageUp => AppEvent::ScrollByRelative {
				direction: QuadDirection::Up,
				fraction: 0.5,
			},
			InputAction::ScrollFullPageDown => AppEvent::ScrollByRelative {
				direction: QuadDirection::Down,
				fraction: 1.,
			},
			InputAction::ScrollFullPageUp => AppEvent::ScrollByRelative {
				direction: QuadDirection::Up,
				fraction: 1.,
			},
			InputAction::TestError => AppEvent::TestError("test error please ignore".to_owned()),
		}
	}
}
