use bevy_ecs::{
	entity::Entity,
	system::{EntityCommands, Query},
};
use ratatui::{
	buffer::Buffer,
	layout::{Alignment, Constraint, Layout, Rect},
	style::{Color, Stylize as _},
	widgets::{Block, BorderType, Paragraph, Widget as _},
};

use crate::{
	app::app_event::AppEvent,
	ecs::{Area, EntityCommandsExt as _, UiComponent, Viewport},
};

use super::{GenericComponent, click::ClickComponent, counter::CounterComponent};

#[derive(Debug)]
pub struct RootComponent {
	counter: Entity,
	click: Entity,
}

impl Default for RootComponent {
	fn default() -> Self {
		Self {
			counter: Entity::PLACEHOLDER,
			click: Entity::PLACEHOLDER,
		}
	}
}

impl UiComponent<AppEvent> for RootComponent {
	fn init(&mut self, mut cmd: EntityCommands) {
		self.counter = cmd
			.spawn_child(GenericComponent::from(CounterComponent::default()))
			.id();
		self.click = cmd
			.spawn_child(GenericComponent::from(ClickComponent::default()))
			.id();
	}

	fn render(
		&self,
		area: Rect,
		buf: &mut Buffer,
		mut children: Query<(&mut Area, Option<&mut Viewport>)>,
	) {
		let block = Block::bordered()
			.title("event-driven-async-generated")
			.title_alignment(Alignment::Center)
			.border_type(BorderType::Rounded);
		let inner = block.inner(area);

		let text = "This is a tui template.\n\
			Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
			Press 'j' and 'k' to decrement and increment the counter respectively.";

		let paragraph = Paragraph::new(text)
			.block(block)
			.fg(Color::Cyan)
			.bg(Color::Black)
			.centered();

		paragraph.render(area, buf);

		let [counter_area, click_area] =
			Layout::vertical([Constraint::Length(1), Constraint::Length(25)])
				.margin(3)
				.areas(inner);
		if let Ok((mut area, _)) = children.get_mut(self.counter) {
			area.0 = counter_area;
		}
		if let Ok((mut area, _)) = children.get_mut(self.click) {
			area.0 = click_area;
		}
	}
}
