use ratatui::{
	buffer::Buffer,
	layout::Rect,
	style::{Color, Stylize as _},
	widgets::{Paragraph, Widget as _, WidgetRef},
};

use crate::{
	app::component::{Component, EventFlow},
	event::{AppEvent, Event},
};

#[derive(Debug, Default)]
pub struct CounterComponent {
	value: i32,
}

impl Component for CounterComponent {
	fn handle_event(&mut self, event: &crate::event::Event) -> super::EventFlow {
		match event {
			Event::AppEvent(AppEvent::Increment) => {
				self.value += 1;
				EventFlow::Propagate
			}
			Event::AppEvent(AppEvent::Decrement) => {
				self.value -= 1;
				EventFlow::Propagate
			}
			_ => EventFlow::Propagate,
		}
	}
}

impl WidgetRef for CounterComponent {
	fn render_ref(&self, area: Rect, buf: &mut Buffer) {
		let counter_text = format!("Counter: {}", self.value);
		let counter = Paragraph::new(counter_text).fg(Color::Red).centered();
		counter.render(area, buf);
	}
}
