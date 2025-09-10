use bevy_ecs::{
	component::Component,
	system::{In, InMut, InRef, Query, Res},
};
use color_eyre::eyre;
use crossterm::event::MouseButton;
use ratatui::{
	layout::{Constraint, Layout},
	style::Stylize as _,
	widgets::{Block, Widget},
};

use crate::{
	app_event::AppEvent,
	config::Theme,
	tui::{ecs::*, event::Event},
};

#[derive(Debug, Component, Default, Clone, Copy)]
#[component(on_add = Self::register_systems)]
pub struct ControlPanelComponent {
	playing: bool,
}

impl UiComponent for ControlPanelComponent {
	fn systems() -> impl IntoIterator<Item = UiSystem> {
		[
			UiSystem::update(Self::update),
			UiSystem::render(Self::render),
		]
	}
}

impl ControlPanelComponent {
	fn icon(self) -> &'static str {
		if self.playing { "󰏤" } else { "󰐊" }
	}

	fn update(
		(In(entity), InRef(event)): UpdateInput<AppEvent>,
		mut query: Query<&mut Self>,
	) -> eyre::Result<EventFlow> {
		let mut comp = query.get_mut(entity)?;

		Ok(match event {
			Event::Mouse(mouse_event) => match mouse_event.kind {
				crossterm::event::MouseEventKind::Down(MouseButton::Left) => {
					comp.playing = !comp.playing;
					EventFlow::Consume
				}
				_ => EventFlow::Propagate,
			},
			_ => EventFlow::Propagate,
		})
	}

	fn render(
		(In(entity), InMut(buf)): RenderInput,
		theme: Res<Theme>,
		query: Query<(&Self, &Area)>,
	) -> eyre::Result<()> {
		let (comp, area) = query.get(entity)?;
		let area = **area;

		Block::new().bg(theme.colours.overlay).render(area, buf);

		let [button_area] = Layout::vertical([Constraint::Length(1)])
			.flex(ratatui::layout::Flex::Center)
			.areas(area);
		let [button_area] = Layout::horizontal([Constraint::Length(2)])
			.flex(ratatui::layout::Flex::Center)
			.areas(button_area);

		comp.icon().render(button_area, buf);

		Ok(())
	}
}
