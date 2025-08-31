use bevy_ecs::{
	component::Component,
	system::{In, InMut, Query, Res},
};
use color_eyre::eyre;
use ratatui::{
	layout::{Constraint, Layout},
	style::Color,
	widgets::{Block, BorderType, Borders, WidgetRef as _},
};

use crate::tui::ecs::{Area, Focus, RenderInput, RenderSystem};

#[derive(Debug, Component, Default)]
#[require(RenderSystem::new(Self::render))]
pub struct AlbumCardComponent {}

impl AlbumCardComponent {
	fn border_colour(focus: bool) -> Color {
		if focus { Color::Magenta } else { Color::Reset }
	}

	fn render(
		(In(entity), InMut(buf)): RenderInput,
		focus: Res<Focus>,
		query: Query<(&Self, &Area)>,
	) -> eyre::Result<()> {
		let (_comp, area) = query.get(entity)?;
		let area = **area;
		let has_focus = focus.target == entity;

		let block = Block::bordered()
			.border_type(BorderType::Rounded)
			.border_style(Self::border_colour(has_focus));
		block.render_ref(area, buf);
		let area = block.inner(area);

		let [_image_area, info_area] = Layout::vertical([
			Constraint::Length((area.width as f32 / 2.2) as u16),
			Constraint::Fill(1),
		])
		.areas(area);

		let block = Block::new()
			.borders(Borders::TOP)
			.border_style(Self::border_colour(has_focus));
		block.render_ref(info_area, buf);

		Ok(())
	}
}
