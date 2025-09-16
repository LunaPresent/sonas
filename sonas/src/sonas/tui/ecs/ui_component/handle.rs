use std::ops;

use bevy_ecs::{component::Component, system::SystemId};
use smallvec::SmallVec;

use super::{InitContext, RenderContext, UiSystemContext, UpdateContext};
use crate::tui::ecs::Area;

const N: usize = 3;

pub(crate) type InitSystemId = SystemId<InitContext, <InitContext as UiSystemContext>::Result>;
pub(crate) type UpdateSystemId<T> =
	SystemId<UpdateContext<'static, T>, <UpdateContext<'static, T> as UiSystemContext>::Result>;
pub(crate) type RenderSystemId =
	SystemId<RenderContext<'static>, <RenderContext<'static> as UiSystemContext>::Result>;

pub(crate) trait UiSystemHandle:
	ops::Deref<Target = SmallVec<[SystemId<Self::SystemInput, Self::SystemOutput>; N]>>
	+ ops::DerefMut<Target = SmallVec<[SystemId<Self::SystemInput, Self::SystemOutput>; N]>>
{
	type SystemInput: UiSystemContext + 'static;
	type SystemOutput: 'static;
}

#[derive(Debug, Component, Default, Clone, derive_more::Deref, derive_more::DerefMut)]
pub(crate) struct InitHandle(SmallVec<[InitSystemId; N]>);

impl UiSystemHandle for InitHandle {
	type SystemInput = InitContext;
	type SystemOutput = <InitContext as UiSystemContext>::Result;
}

#[derive(Debug, Component, Clone, derive_more::Deref, derive_more::DerefMut)]
pub(crate) struct UpdateHandle<T>(SmallVec<[UpdateSystemId<T>; N]>)
where
	T: 'static;

impl<T> Default for UpdateHandle<T> {
	fn default() -> Self {
		Self(SmallVec::default())
	}
}

impl<T> UiSystemHandle for UpdateHandle<T> {
	type SystemInput = UpdateContext<'static, T>;
	type SystemOutput = <UpdateContext<'static, T> as UiSystemContext>::Result;
}

#[derive(Debug, Component, Default, Clone, derive_more::Deref, derive_more::DerefMut)]
#[require(Area)]
pub(crate) struct RenderHandle(SmallVec<[RenderSystemId; N]>);

impl UiSystemHandle for RenderHandle {
	type SystemInput = RenderContext<'static>;
	type SystemOutput = <RenderContext<'static> as UiSystemContext>::Result;
}
