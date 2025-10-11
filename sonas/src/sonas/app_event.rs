use ratatui::layout::Rect;

use crate::{tui::event, util::QuadDirection};

#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
	Quit,
	MoveCursor(QuadDirection),
	ScrollBy {
		direction: QuadDirection,
		amount: u16,
	},
	ScrollByRelative {
		direction: QuadDirection,
		fraction: f32,
	},
	ScrollTo(Rect),
	TestError(String),
}

impl event::AppEvent for AppEvent {
	fn is_quit(&self) -> bool {
		self == &Self::Quit
	}
}
