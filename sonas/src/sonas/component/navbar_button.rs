use bevy_ecs::{
	component::Component,
	system::{In, InMut, Query, Res},
};
use color_eyre::eyre;
use ratatui::{
	layout::{Constraint, Flex, Layout, Position},
	style::{Color, Stylize as _},
	widgets::{Block, Padding, Widget as _, WidgetRef as _},
};

use crate::tui::ecs::{Area, CursorPos, RenderInput, RenderSystem};

#[derive(Debug, Clone, Copy)]
pub enum NavbarButtonType {
	Albums,
	Artists,
	Playlists,
}

impl NavbarButtonType {
	pub fn icon(self) -> &'static str {
		match self {
			NavbarButtonType::Albums => "󰀥",
			NavbarButtonType::Artists => "",
			NavbarButtonType::Playlists => "󰲸",
		}
	}

	pub fn text(self) -> &'static str {
		match self {
			NavbarButtonType::Albums => "Albums",
			NavbarButtonType::Artists => "Artists",
			NavbarButtonType::Playlists => "Playlists",
		}
	}
}

#[derive(Debug, Component, Clone, Copy)]
#[require(RenderSystem::new(Self::render))]
pub struct NavbarButtonComponent {
	button_type: NavbarButtonType,
}

impl NavbarButtonComponent {
	pub fn new(button_type: NavbarButtonType) -> Self {
		Self { button_type }
	}

	pub fn button_type(self) -> NavbarButtonType {
		self.button_type
	}

	fn bg_colour(hovered: bool) -> Option<Color> {
		if hovered { Some(Color::Black) } else { None }
	}

	fn render(
		(In(entity), InMut(buf)): RenderInput,
		query: Query<(&Self, &Area)>,
		cursor: Res<CursorPos>,
	) -> eyre::Result<()> {
		let (comp, area) = query.get(entity)?;
		let area = **area;

		let hovered = area.contains(Position::new(cursor.x, cursor.y));

		let mut block = Block::new().padding(Padding::horizontal(1));
		if let Some(colour) = Self::bg_colour(hovered) {
			block = block.bg(colour);
		}
		block.render_ref(area, buf);

		let [text_area] = Layout::vertical(Constraint::from_lengths([1]))
			.flex(Flex::Center)
			.areas(block.inner(area));
		format!("{} {}", comp.button_type.icon(), comp.button_type.text())
			.bold()
			.render(text_area, buf);

		Ok(())
	}
}
