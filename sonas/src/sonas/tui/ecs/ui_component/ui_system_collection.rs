use std::ops;

use bevy_ecs::{component::Component, system::SystemId};
use smallvec::SmallVec;

use super::{ErrorContext, InitContext, RenderContext, UiSystemContext, UpdateContext};
use crate::tui::ecs::{Area, error_handling::UiSystemError};

const N: usize = 3;

type UiSystemId<C> = SystemId<C, Result<<C as UiSystemContext>::Result, UiSystemError>>;
pub(crate) type InitSystemId = UiSystemId<InitContext>;
pub(crate) type UpdateSystemId<T> = UiSystemId<UpdateContext<'static, T>>;
pub(crate) type RenderSystemId = UiSystemId<RenderContext<'static>>;
pub(crate) type ErrorSystemId<E> = UiSystemId<ErrorContext<'static, E>>;

pub(crate) trait UiSystemCollection:
	ops::Deref<Target = SmallVec<[UiSystemId<Self::SystemInput>; N]>>
	+ ops::DerefMut<Target = SmallVec<[UiSystemId<Self::SystemInput>; N]>>
{
	type SystemInput: UiSystemContext + 'static;
	type SystemOutput: 'static;
}

#[derive(Debug, Component, Default, Clone, derive_more::Deref, derive_more::DerefMut)]
pub(crate) struct InitSystemCollection(SmallVec<[InitSystemId; N]>);

impl UiSystemCollection for InitSystemCollection {
	type SystemInput = InitContext;
	type SystemOutput = <InitContext as UiSystemContext>::Result;
}

#[derive(Debug, Component, Clone, derive_more::Deref, derive_more::DerefMut)]
pub(crate) struct UpdateSystemCollection<T>(SmallVec<[UpdateSystemId<T>; N]>)
where
	T: 'static;

impl<T> Default for UpdateSystemCollection<T> {
	fn default() -> Self {
		Self(SmallVec::default())
	}
}

impl<T> UiSystemCollection for UpdateSystemCollection<T> {
	type SystemInput = UpdateContext<'static, T>;
	type SystemOutput = <UpdateContext<'static, T> as UiSystemContext>::Result;
}

#[derive(Debug, Component, Default, Clone, derive_more::Deref, derive_more::DerefMut)]
#[require(Area)]
pub(crate) struct RenderSystemCollection(SmallVec<[RenderSystemId; N]>);

impl UiSystemCollection for RenderSystemCollection {
	type SystemInput = RenderContext<'static>;
	type SystemOutput = <RenderContext<'static> as UiSystemContext>::Result;
}

#[derive(Debug, Component, Clone, derive_more::Deref, derive_more::DerefMut)]
#[require(Area)]
pub(crate) struct ErrorSystemCollection<E>(SmallVec<[ErrorSystemId<E>; N]>)
where
	E: 'static;

impl<E> Default for ErrorSystemCollection<E> {
	fn default() -> Self {
		Self(SmallVec::default())
	}
}
impl<E> UiSystemCollection for ErrorSystemCollection<E> {
	type SystemInput = ErrorContext<'static, E>;
	type SystemOutput = <ErrorContext<'static, E> as UiSystemContext>::Result;
}
