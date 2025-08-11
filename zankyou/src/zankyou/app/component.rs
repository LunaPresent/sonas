mod control_panel;
mod library;
mod navbar;
mod navbar_button;
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
use control_panel::ControlPanelComponent;
use library::LibraryComponent;
use navbar::NavbarComponent;
use navbar_button::NavbarButtonComponent;
use root::RootComponent;

#[derive(Debug, Component, From)]
#[require(Area)]
pub enum GenericComponent {
	Root(RootComponent),
	ControlPanel(ControlPanelComponent),
	Library(LibraryComponent),
	Navbar(NavbarComponent),
	NavbarButton(NavbarButtonComponent),
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
			Self::ControlPanel(c) => c.init(cmd),
			Self::Library(c) => c.init(cmd),
			Self::Navbar(c) => c.init(cmd),
			Self::NavbarButton(c) => c.init(cmd),
		}
	}

	fn handle_event(&mut self, event: &Event<AppEvent>, cmd: EntityCommands) -> EventFlow {
		match self {
			Self::Root(c) => c.handle_event(event, cmd),
			Self::ControlPanel(c) => c.handle_event(event, cmd),
			Self::Library(c) => c.handle_event(event, cmd),
			Self::Navbar(c) => c.handle_event(event, cmd),
			Self::NavbarButton(c) => c.handle_event(event, cmd),
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
			Self::ControlPanel(c) => c.render(area, buf, children),
			Self::Library(c) => c.render(area, buf, children),
			Self::Navbar(c) => c.render(area, buf, children),
			Self::NavbarButton(c) => c.render(area, buf, children),
		}
	}
}
