use bevy_ecs::{
	component::Component,
	entity::Entity,
	system::{In, InMut, InRef, Query},
};
use color_eyre::eyre;
use ratatui::layout::{Rect, Size};

use crate::{
	app_event::AppEvent,
	tui::{
		ecs::{Area, EventFlow, RenderInput, RenderSystem, UpdateInput, UpdateSystem, Viewport},
		event::Event,
	},
};

#[derive(Debug, Component)]
#[require(
	UpdateSystem::<AppEvent>::new(Self::update),
	RenderSystem::new(Self::render),
	Viewport
)]
pub struct ScrollableComponent<F>
where
	F: Fn(Rect) -> Size + Send + Sync + 'static,
{
	inner: Entity,
	size_fn: F,
}

impl<F> ScrollableComponent<F>
where
	F: Fn(Rect) -> Size + Send + Sync + 'static,
{
	pub fn new(inner: Entity, size_fn: F) -> Self {
		Self { inner, size_fn }
	}

	fn update(
		(In(entity), InRef(event)): UpdateInput<AppEvent>,
		mut query: Query<&mut Viewport>,
	) -> eyre::Result<EventFlow> {
		let mut viewport = query.get_mut(entity)?;
		Ok(match event {
			Event::App(AppEvent::MoveCursor(direction)) => {
				viewport.offset.x = viewport.offset.x.saturating_add_signed(direction.x());
				viewport.offset.y = viewport.offset.y.saturating_add_signed(direction.y());
				EventFlow::Consume
			}
			_ => EventFlow::Propagate,
		})
	}

	fn render(
		(In(entity), InMut(_buf)): RenderInput,
		mut query: Query<(&Self, &mut Viewport)>,
		mut areas: Query<&mut Area>,
	) -> eyre::Result<()> {
		let (comp, mut viewport) = query.get_mut(entity)?;
		let area = **areas.get(entity)?;
		viewport.size = (comp.size_fn)(area);
		viewport.clamp_offset(area.as_size())?;
		**areas.get_mut(comp.inner)? = viewport.area();

		Ok(())
	}
}
