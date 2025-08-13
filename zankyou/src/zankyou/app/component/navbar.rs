use std::iter;

use bevy_ecs::{
	component::Component,
	entity::Entity,
	system::{Commands, In, InMut, Query},
};
use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Layout},
};

use crate::ecs::{Area, EntityCommandsExt as _, InitSystem, RenderSystem};

use super::{NavbarButtonComponent, navbar_button::NavbarButtonType};

#[derive(Debug, Component, Default)]
#[require(InitSystem::new(Self::init), RenderSystem::new(Self::render))]
pub struct NavbarComponent {
	buttons: Vec<Entity>,
}

impl NavbarComponent {
	fn init(In(entity): In<Entity>, mut query: Query<&mut Self>, mut cmd: Commands) {
		let mut comp = query.get_mut(entity).expect("?");
		let mut ec = cmd.entity(entity);

		comp.buttons.reserve(3);
		comp.buttons.push(
			ec.spawn_child(NavbarButtonComponent::new(NavbarButtonType::Albums))
				.id(),
		);
		comp.buttons.push(
			ec.spawn_child(NavbarButtonComponent::new(NavbarButtonType::Artists))
				.id(),
		);
		comp.buttons.push(
			ec.spawn_child(NavbarButtonComponent::new(NavbarButtonType::Playlists))
				.id(),
		);
	}

	fn render(
		(In(entity), InMut(buf)): (In<Entity>, InMut<Buffer>),
		query: Query<&Self>,
		mut areas: Query<&mut Area>,
	) {
		let comp = query.get(entity).expect("?");
		let area = **areas.get(entity).expect("?");

		let button_areas = Layout::vertical(Constraint::from_lengths(iter::repeat_n(
			1,
			comp.buttons.len(),
		)))
		.spacing(1)
		.split(area);
		for (&button, &button_area) in comp.buttons.iter().zip(button_areas.iter()) {
			if let Ok(mut area) = areas.get_mut(button) {
				**area = button_area;
			}
		}
	}
}
