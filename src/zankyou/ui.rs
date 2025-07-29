use ratatui::{
	buffer::Buffer,
	layout::{Alignment, Rect},
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

		let text = format!(
			"This is a tui template.\n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: {}",
			self.counter
		);

		let paragraph = Paragraph::new(text)
			.block(block)
			.fg(Color::Cyan)
			.bg(Color::Black)
			.centered();

		paragraph.render(area, buf);
	}
}
