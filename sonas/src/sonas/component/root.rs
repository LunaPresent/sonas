use bevy_ecs::{
	component::Component,
	entity::Entity,
	system::{Commands, In, InMut, Query, Res},
};
use color_eyre::eyre;
use ratatui::layout::{Constraint, Layout};

use super::{ControlPanelComponent, LibraryComponent, NavbarComponent};
use crate::{
	config::UserConfig,
	tui::{
		config::{Config, KeyHandler},
		ecs::{Area, EntityCommandsExt as _, InitInput, InitSystem, RenderInput, RenderSystem},
	},
};

#[derive(Debug, Component)]
#[require(InitSystem::new(Self::init), RenderSystem::new(Self::render))]
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

impl RootComponent {
	fn init(
		In(entity): InitInput,
		config: Res<Config<UserConfig>>,
		mut query: Query<&mut Self>,
		mut cmd: Commands,
	) -> eyre::Result<()> {
		let mut comp = query.get_mut(entity)?;
		let mut ec = cmd.entity(entity);
		ec.insert_if_new(KeyHandler::new(config.keys.generate_key_map()));
		comp.control_panel = ec.spawn_child(ControlPanelComponent::default()).id();
		comp.nav_bar = ec.spawn_child(NavbarComponent::default()).id();
		comp.library = ec.spawn_child(LibraryComponent::default()).id();

		Ok(())
	}

	fn render(
		(In(entity), InMut(_buf)): RenderInput,
		query: Query<&Self>,
		mut areas: Query<&mut Area>,
	) -> eyre::Result<()> {
		let comp = query.get(entity)?;
		let area = **areas.get(entity)?;

		let [navbar_area, library_area, control_panel_area] = Layout::vertical([
			Constraint::Length(1),
			Constraint::Fill(1),
			Constraint::Length(5),
		])
		.areas(area);

		**areas.get_mut(comp.nav_bar)? = navbar_area;
		**areas.get_mut(comp.control_panel)? = control_panel_area;
		**areas.get_mut(comp.library)? = library_area;

		Ok(())
	}
}
