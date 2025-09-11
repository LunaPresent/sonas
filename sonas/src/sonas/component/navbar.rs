use bevy_ecs::{
	component::Component,
	entity::Entity,
	system::{Commands, In, InMut, Query},
};
use color_eyre::eyre;
use ratatui::layout::{Constraint, Layout};

use super::{NavbarButtonComponent, navbar_button::NavbarButtonType};
use crate::tui::ecs::*;

#[derive(Debug, Component, Default)]
#[component(on_add = Self::register_systems)]
#[component(on_remove = Self::register_systems)]
pub struct NavbarComponent {
	buttons: Vec<Entity>,
}

impl UiComponent for NavbarComponent {
	fn systems() -> impl IntoIterator<Item = UiSystem> {
		[UiSystem::init(Self::init), UiSystem::render(Self::render)]
	}
}

impl NavbarComponent {
	fn init(
		In(entity): InitInput,
		mut query: Query<&mut Self>,
		mut cmd: Commands,
	) -> eyre::Result<()> {
		let mut comp = query.get_mut(entity)?;
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

		Ok(())
	}

	fn render(
		(In(entity), InMut(_buf)): RenderInput,
		query: Query<&Self>,
		mut areas: Query<&mut Area>,
		buttons: Query<&NavbarButtonComponent>,
	) -> eyre::Result<()> {
		let comp = query.get(entity)?;
		let area = **areas.get(entity)?;

		let button_areas = Layout::horizontal(Constraint::from_lengths(comp.buttons.iter().map(
			|entity| {
				buttons
					.get(*entity)
					.map(|btn| btn.button_type().text().len() as u16 + 4)
					.unwrap_or_default()
			},
		)))
		.spacing(1)
		.split(area);
		for (&button, &button_area) in comp.buttons.iter().zip(button_areas.iter()) {
			**areas.get_mut(button)? = button_area;
		}

		Ok(())
	}
}
