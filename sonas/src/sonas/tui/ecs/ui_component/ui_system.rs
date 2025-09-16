use core::mem;
use std::{convert::Infallible, marker::PhantomData};

use bevy_ecs::{
	system::{In, IntoSystem, SystemId},
	world::{EntityWorldMut, World},
};
use color_eyre::eyre;

use super::UiSystemContext;

pub trait IntoResult<T> {
	type Error;

	fn into_result(self) -> Result<T, Self::Error>;
}

impl<T> IntoResult<T> for T {
	type Error = Infallible;

	fn into_result(self) -> Result<T, Self::Error> {
		Ok(self)
	}
}

impl<T, E> IntoResult<T> for Result<T, E> {
	type Error = E;

	fn into_result(self) -> Result<T, Self::Error> {
		self
	}
}

fn handle_system_result<R, T>(In(result): In<R>) -> eyre::Result<T>
where
	R: IntoResult<T, Error: Into<eyre::Report>>,
{
	result.into_result().map_err(|e| e.into())
}

// TODO: documentation
pub struct UiSystem {
	boxed_system: Box<dyn GenericSystem + Sync + Send>,
}

impl UiSystem {
	// TODO: documentation
	pub fn new<C, R, M, S>(system: S) -> Self
	where
		C: UiSystemContext + 'static,
		R: IntoResult<C::Result, Error: Into<eyre::Report>> + 'static,
		M: 'static,
		S: IntoSystem<C, R, M> + Sync + Send + Clone + 'static,
	{
		Self {
			boxed_system: Box::new(GenericSystemImpl {
				registrar: CachedSystemRegistrar::System(system, PhantomData),
			}),
		}
	}

	pub(crate) fn register(&mut self, entity_world: &mut EntityWorldMut) {
		self.boxed_system.register(entity_world);
	}

	pub(crate) fn unregister(&mut self, entity_world: &mut EntityWorldMut) {
		self.boxed_system.unregister(entity_world);
	}
}

trait GenericSystem {
	fn register(&mut self, entity_world: &mut EntityWorldMut);
	fn unregister(&mut self, entity_world: &mut EntityWorldMut);
}

struct GenericSystemImpl<C, R, M, S>
where
	C: UiSystemContext,
{
	registrar: CachedSystemRegistrar<C, R, M, S>,
}

impl<C, R, M, S> GenericSystem for GenericSystemImpl<C, R, M, S>
where
	C: UiSystemContext + 'static,
	R: IntoResult<C::Result, Error: Into<eyre::Report>> + 'static,
	S: IntoSystem<C, R, M> + Clone + 'static,
{
	fn register(&mut self, entity_world: &mut EntityWorldMut) {
		let system_id = unsafe {
			let world = entity_world.world_mut();
			self.registrar.register(world)
		};
		entity_world.insert_if_new(C::Handle::default());
		let mut handle = entity_world
			.get_mut::<C::Handle>()
			.expect("Handle should have just been inserted");
		handle.push(system_id);
	}

	fn unregister(&mut self, entity_world: &mut EntityWorldMut) {
		let system_id = unsafe {
			let world = entity_world.world_mut();
			self.registrar.register(world)
		};
		if let Some(mut handle) = entity_world.get_mut::<C::Handle>()
			&& let Some(i) = handle
				.iter()
				.enumerate()
				.find_map(|(i, &s)| (s == system_id).then_some(i))
		{
			handle.swap_remove(i);
		}
	}
}

enum CachedSystemRegistrar<C, R, M, S>
where
	C: UiSystemContext,
{
	System(S, PhantomData<fn() -> (R, M)>),
	Id(SystemId<C, eyre::Result<C::Result>>),
}

impl<C, R, M, S> CachedSystemRegistrar<C, R, M, S>
where
	C: UiSystemContext + 'static,
	R: IntoResult<C::Result, Error: Into<eyre::Report>> + 'static,
	S: IntoSystem<C, R, M> + Clone + 'static,
{
	fn register(&mut self, world: &mut World) -> SystemId<C, eyre::Result<C::Result>> {
		match self {
			Self::System(system, _) => {
				let system = system.clone().pipe(handle_system_result);

				let system_id = world.register_system_cached(system);
				let _ = mem::replace(self, Self::Id(system_id));
				system_id
			}
			Self::Id(system_id) => *system_id,
		}
	}
}
