use bevy_ecs::system::{EntityCommands, Query};
use crossterm::event::MouseEvent;
use ratatui::{
	buffer::Buffer,
	layout::Rect,
	style::{Color, Stylize as _},
	widgets::{Block, Paragraph, Widget as _},
};

use crate::{
	app::app_event::AppEvent,
	ecs::{Area, EventFlow, UiComponent, Viewport},
	event::Event,
};

#[derive(Debug, Default)]
pub struct ClickComponent {
	event: Option<MouseEvent>,
}

impl UiComponent<AppEvent> for ClickComponent {
	fn handle_event(&mut self, _cmd: EntityCommands, event: &Event<AppEvent>) -> EventFlow {
		match event {
			Event::Mouse(me) => {
				self.event = Some(*me);
				EventFlow::Consume
			}
			_ => EventFlow::Propagate,
		}
	}

	fn render(
		&self,
		area: Rect,
		buf: &mut Buffer,
		_areas: Query<(&mut Area, Option<&mut Viewport>)>,
	) {
		Block::new().bg(Color::Reset).render(area, buf);
		if let Some(me) = self.event {
			let text = format!("Event: {:?}", me);
			let p = Paragraph::new(text).fg(Color::Green).centered();
			p.render(area, buf);
		}
	}
}
