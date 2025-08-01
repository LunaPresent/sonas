mod click;
mod counter;
mod root;

use bevy_ecs::{
	component::Component,
	entity::Entity,
	system::{Commands, EntityCommands, Query},
};
use derive_more::From;
use ratatui::{buffer::Buffer, layout::Rect};

use super::app_event::AppEvent;
use crate::{
	ecs::{Area, EventFlow, UiComponent, Viewport},
	event::Event,
};
use click::ClickComponent;
use counter::CounterComponent;
use root::RootComponent;

#[derive(Debug, Component, From)]
#[require(Area)]
pub enum GenericComponent {
	RootComponent(RootComponent),
	CounterComponent(CounterComponent),
	ClickComponent(ClickComponent),
}

impl Default for GenericComponent {
	fn default() -> Self {
		Self::RootComponent(RootComponent::default())
	}
}

impl UiComponent<AppEvent> for GenericComponent {
	fn init(&mut self, cmd: EntityCommands) {
		match self {
			Self::RootComponent(c) => c.init(cmd),
			Self::CounterComponent(c) => c.init(cmd),
			Self::ClickComponent(c) => c.init(cmd),
		}
	}

	fn handle_event(&mut self, cmd: EntityCommands, event: &Event<AppEvent>) -> EventFlow {
		match self {
			Self::RootComponent(c) => c.handle_event(cmd, event),
			Self::CounterComponent(c) => c.handle_event(cmd, event),
			Self::ClickComponent(c) => c.handle_event(cmd, event),
		}
	}

	fn render(
		&self,
		area: Rect,
		buf: &mut Buffer,
		children: Query<(&mut Area, Option<&mut Viewport>)>,
	) {
		match self {
			Self::RootComponent(c) => c.render(area, buf, children),
			Self::CounterComponent(c) => c.render(area, buf, children),
			Self::ClickComponent(c) => c.render(area, buf, children),
		}
	}
}
