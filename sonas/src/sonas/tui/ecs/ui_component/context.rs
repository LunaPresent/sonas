use bevy_ecs::{
	component::{Component, Mutable},
	entity::Entity,
	system::SystemInput,
};
use ratatui::buffer::Buffer;

use super::{
	ErrorSystemCollection, InitSystemCollection, RenderSystemCollection, UiSystemCollection,
	UpdateSystemCollection,
};
use crate::tui::{
	ecs::{EventFlow, error_handling::ErrorFlow},
	event::Event,
};

pub(crate) trait UiSystemContext: SystemInput {
	type Result;
	type Handle: UiSystemCollection<SystemInput = Self, SystemOutput = Self::Result>
		+ Component<Mutability = Mutable>
		+ Default;
}

#[derive(Debug, Clone, Copy)]
pub struct InitContext {
	pub entity: Entity,
}

impl UiSystemContext for InitContext {
	type Result = ();
	type Handle = InitSystemCollection;
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
	type Result = EventFlow;
	type Handle = UpdateSystemCollection<T>;
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
	type Result = ();
	type Handle = RenderSystemCollection;
}

impl SystemInput for RenderContext<'_> {
	type Param<'i> = RenderContext<'i>;
	type Inner<'i> = RenderContext<'i>;

	fn wrap(this: Self::Inner<'_>) -> Self::Param<'_> {
		this
	}
}

#[derive(Debug)]
pub struct ErrorContext<'a, E>
where
	E: 'static,
{
	pub entity: Entity,
	pub error: &'a E,
}

impl<E> UiSystemContext for ErrorContext<'static, E>
where
	E: 'static,
{
	type Result = ErrorFlow;
	type Handle = ErrorSystemCollection<E>;
}

impl<E> SystemInput for ErrorContext<'_, E>
where
	E: 'static,
{
	type Param<'i> = ErrorContext<'i, E>;
	type Inner<'i> = ErrorContext<'i, E>;

	fn wrap(this: Self::Inner<'_>) -> Self::Param<'_> {
		this
	}
}
