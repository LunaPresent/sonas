mod click;
mod counter;
mod root;

use bevy_ecs::{
	component::Component,
	system::{EntityCommands, Query},
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
	Root(RootComponent),
	Counter(CounterComponent),
	Click(ClickComponent),
}

impl Default for GenericComponent {
	fn default() -> Self {
		Self::Root(RootComponent::default())
	}
}

impl UiComponent<AppEvent> for GenericComponent {
	fn init(&mut self, cmd: EntityCommands) {
		match self {
			Self::Root(c) => c.init(cmd),
			Self::Counter(c) => c.init(cmd),
			Self::Click(c) => c.init(cmd),
		}
	}

	fn handle_event(&mut self, cmd: EntityCommands, event: &Event<AppEvent>) -> EventFlow {
		match self {
			Self::Root(c) => c.handle_event(cmd, event),
			Self::Counter(c) => c.handle_event(cmd, event),
			Self::Click(c) => c.handle_event(cmd, event),
		}
	}

	fn render(
		&self,
		area: Rect,
		buf: &mut Buffer,
		children: Query<(&mut Area, Option<&mut Viewport>)>,
	) {
		match self {
			Self::Root(c) => c.render(area, buf, children),
			Self::Counter(c) => c.render(area, buf, children),
			Self::Click(c) => c.render(area, buf, children),
		}
	}
}
