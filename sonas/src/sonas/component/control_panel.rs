use bevy_ecs::{
	component::Component,
	system::{Query, Res},
};
use color_eyre::eyre;
use crossterm::event::MouseButton;
use oprabeli::{ecs::*, event::SystemEvent};
use ratatui::{
	layout::{Constraint, Layout},
	style::Stylize as _,
	widgets::{Block, Widget},
};

use crate::config::Theme;

#[derive(Debug, Component, Default, Clone, Copy)]
#[component(on_add = Self::register_systems)]
#[component(on_remove = Self::unregister_systems)]
pub struct ControlPanelComponent {
	playing: bool,
}

impl UiComponent for ControlPanelComponent {
	fn systems() -> impl IntoIterator<Item = UiSystem> {
		[UiSystem::new(Self::update), UiSystem::new(Self::render)]
	}
}

impl ControlPanelComponent {
	fn icon(self) -> &'static str {
		if self.playing { "󰏤" } else { "󰐊" }
	}

	fn update(
		context: EventContext<SystemEvent>,
		mut query: Query<&mut Self>,
	) -> eyre::Result<EventFlow> {
		let mut comp = query.get_mut(context.entity)?;

		Ok(match context.event {
			SystemEvent::Mouse(mouse_event) => match mouse_event.kind {
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
		context: RenderContext,
		theme: Res<Theme>,
		query: Query<(&Self, &Area)>,
	) -> eyre::Result<()> {
		let (comp, area) = query.get(context.entity)?;
		let area = **area;

		Block::new()
			.bg(theme.colours.overlay)
			.render(area, context.buffer);

		let [button_area] = Layout::vertical([Constraint::Length(1)])
			.flex(ratatui::layout::Flex::Center)
			.areas(area);
		let [button_area] = Layout::horizontal([Constraint::Length(2)])
			.flex(ratatui::layout::Flex::Center)
			.areas(button_area);

		comp.icon().render(button_area, context.buffer);

		Ok(())
	}
}
