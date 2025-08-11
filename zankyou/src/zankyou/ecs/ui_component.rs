use bevy_ecs::system::{EntityCommands, Query};
use ratatui::{buffer::Buffer, layout::Rect};

use super::{event_handling::EventFlow, rendering::Viewport};
use crate::{ecs::rendering::Area, event::Event};

pub trait UiComponent<E> {
	fn init(&mut self, cmd: EntityCommands) {
		let _ = cmd;
	}

	fn handle_event(&mut self, event: &Event<E>, cmd: EntityCommands) -> EventFlow {
		let _ = cmd;
		let _ = event;
		EventFlow::Propagate
	}

	fn render(
		&self,
		area: Rect,
		buf: &mut Buffer,
		children: Query<(&mut Area, Option<&mut Viewport>)>,
	);
}
