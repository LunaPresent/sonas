use ratatui::layout::Rect;

use crate::util::QuadDirection;

#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
	Quit,
	Suspend,
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
	UpdateKeymap,
}
