use std::iter;

use bevy_ecs::{
	entity::Entity,
	system::{EntityCommands, Query},
};
use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Layout, Rect},
};

use crate::{
	app::app_event::AppEvent,
	ecs::{Area, EntityCommandsExt as _, UiComponent, Viewport},
};

use super::{GenericComponent, NavbarButtonComponent, navbar_button::NavbarButtonType};

#[derive(Debug, Default)]
pub struct NavbarComponent {
	buttons: Vec<Entity>,
}

impl UiComponent<AppEvent> for NavbarComponent {
	fn init(&mut self, mut cmd: EntityCommands) {
		self.buttons.reserve(3);
		self.buttons.push(
			cmd.spawn_child(GenericComponent::from(NavbarButtonComponent::new(
				NavbarButtonType::Albums,
			)))
			.id(),
		);
		self.buttons.push(
			cmd.spawn_child(GenericComponent::from(NavbarButtonComponent::new(
				NavbarButtonType::Artists,
			)))
			.id(),
		);
		self.buttons.push(
			cmd.spawn_child(GenericComponent::from(NavbarButtonComponent::new(
				NavbarButtonType::Playlists,
			)))
			.id(),
		);
	}

	fn render(
		&self,
		area: Rect,
		_buf: &mut Buffer,
		mut children: Query<(&mut Area, Option<&mut Viewport>)>,
	) {
		let areas = Layout::vertical(Constraint::from_lengths(iter::repeat_n(
			1,
			self.buttons.len(),
		)))
		.spacing(1)
		.split(area);
		for (&button, &button_area) in self.buttons.iter().zip(areas.iter()) {
			if let Ok((mut area, _)) = children.get_mut(button) {
				**area = button_area;
			}
		}
	}
}
