use crate::tui::event;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppEvent {
	Quit,
	CursorDown,
	CursorUp,
	CursorLeft,
	CursorRight,
}

impl event::AppEvent for AppEvent {
	fn is_quit(&self) -> bool {
		self == &Self::Quit
	}
}
