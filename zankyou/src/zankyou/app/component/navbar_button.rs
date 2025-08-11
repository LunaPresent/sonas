use std::ops::DerefMut;

use bevy_ecs::{
	entity::Entity,
	system::{EntityCommands, In, Query, Res},
};
use color_eyre::eyre;
use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Flex, Layout, Position, Rect},
	style::{Color, Stylize as _},
	widgets::{Block, Padding, Widget as _, WidgetRef as _},
};

use crate::{
	app::{app_event::AppEvent, component::GenericComponent},
	ecs::{Area, CursorPos, EntityCommandsExt, EventFlow, UiComponent, Viewport},
	event::Event,
};

#[derive(Debug, Clone, Copy)]
pub enum NavbarButtonType {
	Albums,
	Artists,
	Playlists,
}

#[derive(Debug, Clone, Copy)]
pub struct NavbarButtonComponent {
	button_type: NavbarButtonType,
	hovered: bool,
}

impl NavbarButtonComponent {
	pub fn new(button_type: NavbarButtonType) -> Self {
		Self {
			button_type,
			hovered: false,
		}
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

	fn bg_colour(self) -> Color {
		if self.hovered {
			Color::Black
		} else {
			Color::Reset
		}
	}
}

fn on_render(
	In(target): In<Entity>,
	cursor: Res<CursorPos>,
	mut query: Query<(&mut GenericComponent, &Area)>,
) -> eyre::Result<()> {
	let (mut component, area) = query.get_mut(target)?;
	if let GenericComponent::NavbarButton(component) = component.deref_mut() {
		component.hovered = area.contains(Position::new(cursor.x, cursor.y));
	}
	Ok(())
}

impl UiComponent<AppEvent> for NavbarButtonComponent {
	fn handle_event(&mut self, event: &Event<AppEvent>, mut cmd: EntityCommands) -> EventFlow {
		match event {
			Event::Render => {
				cmd.run_system_cached(on_render);
				EventFlow::Consume
			}
			_ => EventFlow::Propagate,
		}
	}

	fn render(
		&self,
		area: Rect,
		buf: &mut Buffer,
		_children: Query<(&mut Area, Option<&mut Viewport>)>,
	) {
		let block = Block::new()
			.bg(self.bg_colour())
			.padding(Padding::horizontal(1));
		block.render_ref(area, buf);

		let [text_area] = Layout::vertical(Constraint::from_lengths([1]))
			.flex(Flex::Center)
			.areas(block.inner(area));
		format!("{} {}", self.icon(), self.text())
			.bold()
			.render(text_area, buf);
	}
}
