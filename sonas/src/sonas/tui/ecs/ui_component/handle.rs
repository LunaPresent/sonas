use std::ops;

use bevy_ecs::{component::Component, system::SystemId};
use color_eyre::eyre;
use smallvec::SmallVec;

use super::{InitContext, RenderContext, UiSystemContext, UpdateContext};
use crate::tui::ecs::Area;

const N: usize = 3;

type UiSystemId<C> = SystemId<C, eyre::Result<<C as UiSystemContext>::Result>>;
pub(crate) type InitSystemId = UiSystemId<InitContext>;
pub(crate) type UpdateSystemId<T> = UiSystemId<UpdateContext<'static, T>>;
pub(crate) type RenderSystemId = UiSystemId<RenderContext<'static>>;

pub(crate) trait UiSystemHandle:
	ops::Deref<Target = SmallVec<[UiSystemId<Self::SystemInput>; N]>>
	+ ops::DerefMut<Target = SmallVec<[UiSystemId<Self::SystemInput>; N]>>
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
