use bevy_ecs::{
	change_detection::DetectChanges,
	component::Component,
	entity::Entity,
	system::{Commands, In, InMut, InRef, Query, Res},
};
use color_eyre::eyre;
use ratatui::{
	layout::{Constraint, Layout, Size},
	style::Stylize,
	widgets::{Block, Widget},
};

use super::{ControlPanelComponent, LibraryComponent, NavbarComponent, ScrollableComponent};
use crate::{
	app_event::AppEvent,
	config::{Keys, Theme},
	tui::{config::KeyHandler, ecs::*},
};

#[derive(Debug, Component)]
#[component(on_add = Self::register_systems)]
#[component(on_remove = Self::unregister_systems)]
pub struct RootComponent {
	control_panel: Entity,
	nav_bar: Entity,
	library_scrollable: Entity,
}

impl UiComponent for RootComponent {
	fn systems() -> impl IntoIterator<Item = UiSystem> {
		[
			UiSystem::init(Self::init),
			UiSystem::update(Self::update),
			UiSystem::render(Self::render),
		]
	}
}

impl Default for RootComponent {
	fn default() -> Self {
		Self {
			control_panel: Entity::PLACEHOLDER,
			nav_bar: Entity::PLACEHOLDER,
			library_scrollable: Entity::PLACEHOLDER,
		}
	}
}

impl RootComponent {
	fn init(
		In(entity): InitInput,
		key_config: Res<Keys>,
		mut query: Query<&mut Self>,
		mut cmd: Commands,
	) -> eyre::Result<()> {
		let mut comp = query.get_mut(entity)?;
		let library = cmd.spawn(LibraryComponent::default()).id();

		let mut ec = cmd.entity(entity);
		ec.insert_if_new(KeyHandler::new(key_config.generate_key_map()));
		comp.control_panel = ec.spawn_child(ControlPanelComponent::default()).id();
		comp.nav_bar = ec.spawn_child(NavbarComponent::default()).id();
		let mut scrollable = ec.spawn_child(ScrollableComponent::new(library, |rect| {
			Size::new(rect.width, rect.height * 3)
		}));
		comp.library_scrollable = scrollable.id();
		scrollable.add_child(library);

		Ok(())
	}

	fn update(
		(In(entity), InRef(_event)): UpdateInput<AppEvent>,
		key_config: Res<Keys>,
		mut query: Query<&mut KeyHandler<AppEvent>>,
	) -> eyre::Result<EventFlow> {
		let mut comp = query.get_mut(entity)?;

		if key_config.is_changed() && !key_config.is_added() {
			*comp = KeyHandler::new(key_config.generate_key_map());
		}
		Ok(EventFlow::Propagate)
	}

	fn render(
		(In(entity), InMut(buf)): RenderInput,
		theme: Res<Theme>,
		query: Query<&Self>,
		mut areas: Query<&mut Area>,
	) -> eyre::Result<()> {
		let comp = query.get(entity)?;
		let area = **areas.get(entity)?;

		Block::new().bg(theme.colours.background).render(area, buf);

		let [navbar_area, library_area, control_panel_area] = Layout::vertical([
			Constraint::Length(1),
			Constraint::Fill(1),
			Constraint::Length(5),
		])
		.areas(area);

		**areas.get_mut(comp.nav_bar)? = navbar_area;
		**areas.get_mut(comp.control_panel)? = control_panel_area;
		**areas.get_mut(comp.library_scrollable)? = library_area;

		Ok(())
	}
}
