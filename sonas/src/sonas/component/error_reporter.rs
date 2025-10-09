use std::collections::VecDeque;

use bevy_ecs::{
	component::Component,
	entity::Entity,
	system::{Commands, Query, Res},
};
use color_eyre::eyre;
use ratatui::layout::{Constraint, Flex, Layout};

use super::ErrorPopupComponent;
use crate::{
	app_event::AppEvent,
	config::Settings,
	tui::{ecs::*, event::Event},
};

#[derive(Debug, Component, Default)]
#[component(on_add = Self::register_systems)]
#[component(on_remove = Self::unregister_systems)]
pub struct ErrorReporterComponent {
	pub error_popups: VecDeque<Entity>,
}

impl UiComponent for ErrorReporterComponent {
	fn systems() -> impl IntoIterator<Item = UiSystem> {
		[
			UiSystem::new(Self::update),
			UiSystem::new(Self::render),
			UiSystem::new(Self::handle_error),
		]
	}
}

impl ErrorReporterComponent {
	pub fn new() -> Self {
		Self::default()
	}

	fn update(context: UpdateContext<AppEvent>) -> eyre::Result<EventFlow> {
		if let Event::App(AppEvent::TestError(s)) = context.event {
			Err(eyre::eyre!(s.clone()))
		} else {
			Ok(EventFlow::Propagate)
		}
	}

	fn render(
		context: RenderContext,
		query: Query<&Self>,
		mut areas: Query<&mut Area>,
	) -> eyre::Result<()> {
		let comp = query.get(context.entity)?;
		let area = **areas.get(context.entity)?;
		let [error_area] = Layout::horizontal([Constraint::Max(60)])
			.flex(Flex::End)
			.areas(area);
		let error_areas = Layout::vertical(core::iter::repeat_n(
			Constraint::Max(7),
			comp.error_popups.len(),
		))
		.split(error_area);
		for (&error_popup, &area) in comp.error_popups.iter().zip(error_areas.iter()) {
			**areas.get_mut(error_popup)? = area;
		}
		Ok(())
	}

	fn handle_error(
		context: ErrorContext<eyre::Report>,
		settings: Res<Settings>,
		mut query: Query<&mut Self>,
		mut cmd: Commands,
	) -> eyre::Result<ErrorFlow> {
		let mut comp = query.get_mut(context.entity)?;
		let mut ec = cmd.entity(context.entity);
		comp.error_popups.push_back(
			ec.spawn_child(ErrorPopupComponent::new(
				context.error.to_string(),
				settings.notification_timeout,
			))
			.id(),
		);

		let _ = ErrorFlow::Explode;
		let _ = ErrorFlow::Propagate;
		Ok(ErrorFlow::Catch)
	}
}
