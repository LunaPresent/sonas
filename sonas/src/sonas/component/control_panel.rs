use bevy_ecs::{
	component::Component,
	system::{In, InMut, InRef, Query},
};
use color_eyre::eyre;
use crossterm::event::MouseButton;
use ratatui::{
	layout::{Constraint, Layout},
	widgets::Widget,
};

use crate::{
	app_event::AppEvent,
	tui::{
		ecs::{Area, EventFlow, RenderInput, RenderSystem, UpdateInput, UpdateSystem},
		event::Event,
	},
};

#[derive(Debug, Component, Default, Clone, Copy)]
#[require(
	UpdateSystem::<AppEvent>::new(Self::update),
	RenderSystem::new(Self::render)
)]
pub struct ControlPanelComponent {
	playing: bool,
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
		query: Query<(&Self, &Area)>,
	) -> eyre::Result<()> {
		let (comp, area) = query.get(entity)?;
		let area = **area;

		let [area] = Layout::vertical([Constraint::Length(1)])
			.flex(ratatui::layout::Flex::Center)
			.areas(area);
		let [area] = Layout::horizontal([Constraint::Length(2)])
			.flex(ratatui::layout::Flex::Center)
			.areas(area);

		comp.icon().render(area, buf);

		Ok(())
	}
}
