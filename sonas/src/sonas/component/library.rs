use std::iter;

use bevy_ecs::{
	component::Component,
	entity::Entity,
	system::{Commands, In, InMut, Query, Res, ResMut},
};
use color_eyre::eyre;
use ratatui::{
	layout::{Flex, Layout},
	style::Stylize as _,
	widgets::{Block, Widget as _},
};

use super::AlbumCardComponent;
use crate::{
	config::Theme,
	tui::ecs::{Area, EntityCommandsExt, Focus, InitInput, InitSystem, RenderInput, RenderSystem},
};

#[derive(Debug, Component, Default)]
#[require(InitSystem::new(Self::init), RenderSystem::new(Self::render))]
pub struct LibraryComponent {
	album_cards: Vec<Entity>,
}

impl LibraryComponent {
	fn init(
		In(entity): InitInput,
		mut focus: ResMut<Focus>,
		mut query: Query<&mut Self>,
		mut cmd: Commands,
	) -> eyre::Result<()> {
		let mut comp = query.get_mut(entity)?;
		let mut ec = cmd.entity(entity);

		comp.album_cards.reserve(50);
		for _ in 0..50 {
			comp.album_cards
				.push(ec.spawn_child(AlbumCardComponent::default()).id());
		}
		focus.target = *comp.album_cards.first().unwrap();

		Ok(())
	}

	fn render(
		(In(entity), InMut(buf)): RenderInput,
		theme: Res<Theme>,
		query: Query<&Self>,
		mut areas: Query<&mut Area>,
	) -> eyre::Result<()> {
		let comp = query.get(entity)?;
		let area = **areas.get(entity)?;

		Block::new().bg(theme.colours.background).render(area, buf);

		let card_width = 22;
		let card_height = 14;
		let horizontal_gap = 3;
		let vertical_gap = 1;
		let horizontal_fit = (area.width / (card_width + horizontal_gap)) as usize;
		let vertical_fit = (area.height / (card_height + vertical_gap)) as usize;

		let columns = Layout::horizontal(iter::repeat_n(card_width, horizontal_fit))
			.spacing(horizontal_gap)
			.flex(Flex::Center);
		let rows =
			Layout::vertical(iter::repeat_n(card_height, vertical_fit)).spacing(vertical_gap);

		for i in 0..50 {
			**areas.get_mut(comp.album_cards[i])? = Default::default();
		}
		for (y, &row) in rows.split(area).iter().enumerate() {
			for (x, &column) in columns.split(row).iter().enumerate() {
				let i = x + y * horizontal_fit;
				if i >= comp.album_cards.len() {
					break;
				}
				**areas.get_mut(comp.album_cards[i])? = column;
			}
		}

		Ok(())
	}
}
