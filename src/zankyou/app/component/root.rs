use ratatui::{
	buffer::Buffer,
	layout::{Alignment, Constraint, Layout, Rect},
	style::{Color, Stylize as _},
	widgets::{Block, BorderType, Paragraph, Widget as _, WidgetRef},
};

use crate::app::component::{Component, CounterComponent, Ref};

#[derive(Debug, Default)]
pub struct RootComponent {
	counter: CounterComponent,
}

impl Component for RootComponent {
	fn children(&mut self) -> impl Iterator<Item = Ref> {
		std::iter::once(Ref::from(&mut self.counter as *mut _))
	}

	fn follow_focus<'a>(&'a mut self) -> super::FollowResult {
		super::FollowResult::Propagate(Ref::from(&mut self.counter as *mut _))
	}
}

impl WidgetRef for RootComponent {
	fn render_ref(&self, area: Rect, buf: &mut Buffer) {
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

		let [counter_area] = Layout::vertical([Constraint::Length(1)])
			.margin(3)
			.areas(inner);
		self.counter.render_ref(counter_area, buf);
	}
}
