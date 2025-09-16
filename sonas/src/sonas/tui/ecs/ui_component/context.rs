use bevy_ecs::{entity::Entity, system::SystemInput};
use color_eyre::eyre;
use ratatui::buffer::Buffer;

use super::{InitHandle, RenderHandle, UiSystemHandle, UpdateHandle};
use crate::tui::{ecs::EventFlow, event::Event};

pub(crate) trait UiSystemContext: SystemInput {
	type Result;
	type Handle: UiSystemHandle<SystemInput = Self, SystemOutput = Self::Result>;
}

#[derive(Debug, Clone, Copy)]
pub struct InitContext {
	pub entity: Entity,
}

impl UiSystemContext for InitContext {
	type Result = eyre::Result<()>;
	type Handle = InitHandle;
}

impl SystemInput for InitContext {
	type Param<'i> = InitContext;
	type Inner<'i> = InitContext;

	fn wrap(this: Self::Inner<'_>) -> Self::Param<'_> {
		this
	}
}

#[derive(Debug)]
pub struct UpdateContext<'a, T> {
	pub entity: Entity,
	pub event: &'a Event<T>,
}

impl<T> UiSystemContext for UpdateContext<'static, T>
where
	T: 'static,
{
	type Result = eyre::Result<EventFlow>;
	type Handle = UpdateHandle<T>;
}

impl<T> SystemInput for UpdateContext<'_, T>
where
	T: 'static,
{
	type Param<'i> = UpdateContext<'i, T>;
	type Inner<'i> = UpdateContext<'i, T>;

	fn wrap(this: Self::Inner<'_>) -> Self::Param<'_> {
		this
	}
}

#[derive(Debug)]
pub struct RenderContext<'a> {
	pub entity: Entity,
	pub buffer: &'a mut Buffer,
}

impl UiSystemContext for RenderContext<'static> {
	type Result = eyre::Result<()>;
	type Handle = RenderHandle;
}

impl SystemInput for RenderContext<'_> {
	type Param<'i> = RenderContext<'i>;
	type Inner<'i> = RenderContext<'i>;

	fn wrap(this: Self::Inner<'_>) -> Self::Param<'_> {
		this
	}
}
