use bevy_ecs::system::{EntityCommands, Query};
use ratatui::{
	buffer::Buffer,
	layout::Rect,
	style::{Color, Stylize as _},
	widgets::{Paragraph, Widget as _},
};

use crate::{
	app::app_event::AppEvent,
	ecs::{Area, EventFlow, Focus, UiComponent, Viewport},
	event::Event,
};

#[derive(Debug, Default)]
pub struct CounterComponent {
	value: i32,
}

impl UiComponent<AppEvent> for CounterComponent {
	fn init(&mut self, mut cmd: EntityCommands) {
		cmd.queue(|mut e: bevy_ecs::world::EntityWorldMut<'_>| {
			e.resource_mut::<Focus>().target = e.id()
		});
	}

	fn handle_event(&mut self, _cmd: EntityCommands, event: &Event<AppEvent>) -> EventFlow {
		match event {
			Event::App(AppEvent::Increment) => {
				self.value += 1;
				EventFlow::Consume
			}
			Event::App(AppEvent::Decrement) => {
				self.value -= 1;
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
		let counter_text = format!("Counter: {}", self.value);
		let counter = Paragraph::new(counter_text).fg(Color::Red).centered();
		counter.render(area, buf);
	}
}
