use ratatui::{
	buffer::Buffer,
	layout::{Alignment, Constraint, Layout, Rect},
	style::{Color, Stylize},
	widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::app::App;

impl Widget for &App {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let block = Block::bordered()
			.title("event-driven-async-generated")
			.title_alignment(Alignment::Center)
			.border_type(BorderType::Rounded);
		let inner = block.inner(area);

		let text = "This is a tui template.\n\
			Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
			Press left and right to increment and decrement the counter respectively.";

		let paragraph = Paragraph::new(text)
			.block(block)
			.fg(Color::Cyan)
			.bg(Color::Black)
			.centered();

		paragraph.render(area, buf);

		let [counter_area] = Layout::vertical([Constraint::Length(1)])
			.margin(3)
			.areas(inner);
		let counter_text = format!("Counter: {}", self.counter);
		let counter = Paragraph::new(counter_text).fg(Color::Red).centered();
		counter.render(counter_area, buf);
	}
}
