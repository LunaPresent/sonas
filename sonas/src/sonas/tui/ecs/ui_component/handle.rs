use std::ops;

use bevy_ecs::{
	component::Component,
	system::{SystemId, SystemInput},
};
use smallvec::SmallVec;

use super::{
	InitInput, InitOutput, InitSystemId, RenderInput, RenderOutput, RenderSystemId, UpdateInput,
	UpdateOutput, UpdateSystemId,
};
use crate::tui::ecs::Area;

const N: usize = 3;

pub(crate) trait SystemHandle:
	ops::Deref<Target = SmallVec<[SystemId<Self::SystemInput, Self::SystemOutput>; N]>>
	+ ops::DerefMut<Target = SmallVec<[SystemId<Self::SystemInput, Self::SystemOutput>; N]>>
{
	type SystemInput: SystemInput + 'static;
	type SystemOutput: 'static;
}

#[derive(Debug, Component, Default, Clone, derive_more::Deref, derive_more::DerefMut)]
pub struct InitHandle(SmallVec<[InitSystemId; N]>);

impl SystemHandle for InitHandle {
	type SystemInput = InitInput;
	type SystemOutput = InitOutput;
}

#[derive(Debug, Component, Clone, derive_more::Deref, derive_more::DerefMut)]
pub struct UpdateHandle<T>(SmallVec<[UpdateSystemId<T>; N]>)
where
	T: 'static;

impl<T> Default for UpdateHandle<T> {
	fn default() -> Self {
		Self(SmallVec::default())
	}
}

impl<T> SystemHandle for UpdateHandle<T> {
	type SystemInput = UpdateInput<'static, T>;
	type SystemOutput = UpdateOutput;
}

#[derive(Debug, Component, Default, Clone, derive_more::Deref, derive_more::DerefMut)]
#[require(Area)]
pub struct RenderHandle(SmallVec<[RenderSystemId; N]>);

impl SystemHandle for RenderHandle {
	type SystemInput = RenderInput<'static>;
	type SystemOutput = RenderOutput;
}
