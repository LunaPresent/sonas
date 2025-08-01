#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppEvent {
	Quit,
	Increment,
	Decrement,
}

impl crate::event::AppEvent for AppEvent {
	fn is_quit(&self) -> bool {
		self == &Self::Quit
	}
}
