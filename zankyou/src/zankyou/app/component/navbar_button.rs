use bevy_ecs::{
	component::Component,
	entity::Entity,
	system::{In, InMut, Query, Res},
};
use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Flex, Layout, Position},
	style::{Color, Stylize as _},
	widgets::{Block, Padding, Widget as _, WidgetRef as _},
};

use crate::ecs::{Area, CursorPos, RenderSystem};

#[derive(Debug, Clone, Copy)]
pub enum NavbarButtonType {
	Albums,
	Artists,
	Playlists,
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

	fn text(self) -> &'static str {
		match self.button_type {
			NavbarButtonType::Albums => "Albums",
			NavbarButtonType::Artists => "Artists",
			NavbarButtonType::Playlists => "Playlists",
		}
	}

	fn icon(self) -> &'static str {
		match self.button_type {
			NavbarButtonType::Albums => "󰀥",
			NavbarButtonType::Artists => "",
			NavbarButtonType::Playlists => "󰲸",
		}
	}

	fn bg_colour(hovered: bool) -> Color {
		if hovered { Color::Black } else { Color::Reset }
	}

	fn render(
		(In(entity), InMut(buf)): (In<Entity>, InMut<Buffer>),
		query: Query<&Self>,
		areas: Query<&Area>,
		cursor: Res<CursorPos>,
	) {
		let comp = query.get(entity).expect("?");
		let area = **areas.get(entity).expect("?");

		let hovered = area.contains(Position::new(cursor.x, cursor.y));

		let block = Block::new()
			.bg(Self::bg_colour(hovered))
			.padding(Padding::horizontal(1));
		block.render_ref(area, buf);

		let [text_area] = Layout::vertical(Constraint::from_lengths([1]))
			.flex(Flex::Center)
			.areas(block.inner(area));
		format!("{} {}", comp.icon(), comp.text())
			.bold()
			.render(text_area, buf);
	}
}
