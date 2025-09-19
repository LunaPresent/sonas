use bevy_ecs::component::Component;
use color_eyre::eyre;

use crate::tui::ecs::*;

#[derive(Debug, Component)]
#[component(on_add = Self::register_systems)]
#[component(on_remove = Self::unregister_systems)]
pub struct LoggerComponent {}

impl UiComponent for LoggerComponent {
	fn systems() -> impl IntoIterator<Item = UiSystem> {
		[UiSystem::new(Self::log_error)]
	}
}

impl LoggerComponent {
	pub fn new() -> Self {
		Self {}
	}

	fn log_error(_context: ErrorContext<eyre::Report>) -> eyre::Result<ErrorFlow> {
		let _ = _context.error;
		let _ = _context.entity;
		let _ = ErrorFlow::Catch;
		let _ = ErrorFlow::Propagate;
		Ok(ErrorFlow::Explode)
	}
}
