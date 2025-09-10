use bevy_ecs::{
	component::Component,
	system::{SystemId, SystemInput},
};
use derive_more::{Deref, DerefMut};
use smallvec::SmallVec;

use super::{
	InitInput, InitOutput, InitSystemId, RenderInput, RenderOutput, RenderSystemId, UpdateInput,
	UpdateOutput, UpdateSystemId,
};
use crate::tui::ecs::Area;

pub trait SystemHandle {
	type SystemInput: SystemInput + 'static;
	type SystemOutput: 'static;

	fn systems(&self) -> &[SystemId<Self::SystemInput, Self::SystemOutput>];
	fn push(&mut self, system_id: SystemId<Self::SystemInput, Self::SystemOutput>);
}

#[derive(Debug, Component, Default, Clone, Deref, DerefMut)]
pub(crate) struct NextInitMarker(usize);

#[derive(Debug, Component, Default, Clone)]
#[require(NextInitMarker)]
pub struct InitHandle(SmallVec<[InitSystemId; 3]>);

impl SystemHandle for InitHandle {
	type SystemInput = InitInput;
	type SystemOutput = InitOutput;

	fn systems(&self) -> &[SystemId<Self::SystemInput, Self::SystemOutput>] {
		self.0.as_slice()
	}

	fn push(&mut self, system_id: SystemId<Self::SystemInput, Self::SystemOutput>) {
		self.0.push(system_id);
	}
}

#[derive(Debug, Component, Clone)]
pub struct UpdateHandle<E>(SmallVec<[UpdateSystemId<E>; 3]>)
where
	E: 'static;

impl<E> Default for UpdateHandle<E> {
	fn default() -> Self {
		Self(SmallVec::default())
	}
}

impl<E> SystemHandle for UpdateHandle<E> {
	type SystemInput = UpdateInput<'static, E>;
	type SystemOutput = UpdateOutput;

	fn systems(&self) -> &[SystemId<Self::SystemInput, Self::SystemOutput>] {
		self.0.as_slice()
	}

	fn push(&mut self, system_id: SystemId<Self::SystemInput, Self::SystemOutput>) {
		self.0.push(system_id);
	}
}

#[derive(Debug, Component, Default, Clone)]
#[require(Area)]
pub struct RenderHandle(SmallVec<[RenderSystemId; 3]>);

impl SystemHandle for RenderHandle {
	type SystemInput = RenderInput<'static>;
	type SystemOutput = RenderOutput;

	fn systems(&self) -> &[SystemId<Self::SystemInput, Self::SystemOutput>] {
		self.0.as_slice()
	}

	fn push(&mut self, system_id: SystemId<Self::SystemInput, Self::SystemOutput>) {
		self.0.push(system_id);
	}
}
