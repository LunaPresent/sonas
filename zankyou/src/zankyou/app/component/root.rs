use std::ops::DerefMut;

use bevy_ecs::{
	entity::Entity,
	system::{EntityCommands, Query},
};
use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Layout, Margin, Rect},
	symbols,
	widgets::{Block, BorderType, Borders, Widget as _, WidgetRef as _},
};

use crate::{
	app::app_event::AppEvent,
	ecs::{Area, EntityCommandsExt as _, UiComponent, Viewport},
};

use super::{ControlPanelComponent, GenericComponent, LibraryComponent, NavbarComponent};

#[derive(Debug)]
pub struct RootComponent {
	control_panel: Entity,
	nav_bar: Entity,
	library: Entity,
}

impl Default for RootComponent {
	fn default() -> Self {
		Self {
			control_panel: Entity::PLACEHOLDER,
			nav_bar: Entity::PLACEHOLDER,
			library: Entity::PLACEHOLDER,
		}
	}
}

impl UiComponent<AppEvent> for RootComponent {
	fn init(&mut self, mut cmd: EntityCommands) {
		self.control_panel = cmd
			.spawn_child(GenericComponent::from(ControlPanelComponent::default()))
			.id();
		self.nav_bar = cmd
			.spawn_child(GenericComponent::from(NavbarComponent::default()))
			.id();
		self.library = cmd
			.spawn_child(GenericComponent::from(LibraryComponent::default()))
			.id();
	}

	fn render(
		&self,
		area: Rect,
		buf: &mut Buffer,
		mut children: Query<(&mut Area, Option<&mut Viewport>)>,
	) {
		let [browser_area, control_panel_area] =
			Layout::vertical([Constraint::Fill(1), Constraint::Length(7)]).areas(area);
		let [navbar_area_fixed, navbar_area_dynamic, library_area] = Layout::horizontal([
			Constraint::Length(4),
			Constraint::Percentage(10),
			Constraint::Fill(1),
		])
		.areas(browser_area);
		let control_panel_area = control_panel_area.inner(Margin::new(1, 0));
		let navbar_area = navbar_area_fixed.union(navbar_area_dynamic);

		let control_panel_block = Block::new()
			.borders(Borders::ALL)
			.border_type(BorderType::Rounded);
		control_panel_block.render_ref(control_panel_area, buf);

		let navbar_block = Block::new()
			.borders(Borders::RIGHT)
			.border_type(BorderType::Plain);
		navbar_block.render_ref(navbar_area, buf);

		symbols::line::HORIZONTAL_UP.render(
			Rect::new(navbar_area.right() - 1, control_panel_area.top(), 1, 1),
			buf,
		);
		symbols::line::HORIZONTAL_DOWN.render(
			Rect::new(
				navbar_area.right() - 1,
				control_panel_area.bottom() - 1,
				1,
				1,
			),
			buf,
		);

		let control_panel_area = control_panel_block.inner(control_panel_area);
		let navbar_area = navbar_block.inner(navbar_area);

		if let Ok((mut area, _)) = children.get_mut(self.control_panel) {
			**area = control_panel_area;
		}
		if let Ok((mut area, _)) = children.get_mut(self.nav_bar) {
			**area = navbar_area;
		}
		if let Ok((mut area, _)) = children.get_mut(self.library) {
			**area = library_area;
		}
	}
}
