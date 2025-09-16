use std::marker::PhantomData;

use bevy_ecs::{
	component::{Component, Mutable},
	system::IntoSystem,
	world::EntityWorldMut,
};

use super::{
	InitContext, InitHandle, RenderContext, RenderHandle, UiSystemContext, UiSystemHandle,
	UpdateContext, UpdateHandle,
};

// TODO: documentation
pub struct UiSystem {
	boxed_system: Box<dyn GenericSystem + Sync + Send>,
}

impl UiSystem {
	// TODO: documentation
	pub fn init<M, S>(system: S) -> Self
	where
		M: Sync + Send + 'static,
		S: IntoSystem<InitContext, <InitContext as UiSystemContext>::Result, M>
			+ Sync
			+ Send
			+ Clone
			+ 'static,
	{
		Self::new::<InitHandle, M, S>(system)
	}

	// TODO: documentation
	pub fn update<T, M, S>(system: S) -> Self
	where
		T: 'static,
		M: Sync + Send + 'static,
		S: IntoSystem<
				UpdateContext<'static, T>,
				<UpdateContext<'static, T> as UiSystemContext>::Result,
				M,
			> + Sync
			+ Send
			+ Clone
			+ 'static,
	{
		Self::new::<UpdateHandle<T>, M, S>(system)
	}

	// TODO: documentation
	pub fn render<M, S>(system: S) -> Self
	where
		M: Sync + Send + 'static,
		S: IntoSystem<
				RenderContext<'static>,
				<RenderContext<'static> as UiSystemContext>::Result,
				M,
			> + Sync
			+ Send
			+ Clone
			+ 'static,
	{
		Self::new::<RenderHandle, M, S>(system)
	}

	pub(crate) fn register(&self, entity_world: &mut EntityWorldMut) {
		self.boxed_system.register(entity_world);
	}

	pub(crate) fn unregister(&self, entity_world: &mut EntityWorldMut) {
		self.boxed_system.unregister(entity_world);
	}

	fn new<H, M, S>(system: S) -> Self
	where
		H: UiSystemHandle + Component<Mutability = Mutable> + Default,
		M: Sync + Send + 'static,
		S: IntoSystem<H::SystemInput, H::SystemOutput, M> + Sync + Send + Clone + 'static,
	{
		Self {
			boxed_system: Box::new(GenericSystemImpl::<H, M, S> {
				system,
				phantom_data: PhantomData,
			}),
		}
	}
}

trait GenericSystem {
	fn register(&self, entity_world: &mut EntityWorldMut);
	fn unregister(&self, entity_world: &mut EntityWorldMut);
}

struct GenericSystemImpl<H, M, S> {
	system: S,
	phantom_data: PhantomData<fn() -> (H, M)>,
}

impl<H, M, S> GenericSystem for GenericSystemImpl<H, M, S>
where
	H: UiSystemHandle + Component<Mutability = Mutable> + Default,
	M: Sync + Send + 'static,
	S: IntoSystem<H::SystemInput, H::SystemOutput, M> + Sync + Send + Clone + 'static,
{
	fn register(&self, entity_world: &mut EntityWorldMut) {
		let system_id = unsafe {
			let world = entity_world.world_mut();
			world.register_system_cached(self.system.clone())
		};
		entity_world.insert_if_new(H::default());
		let mut handle = entity_world
			.get_mut::<H>()
			.expect("Handle should have just been inserted");
		handle.push(system_id);
	}

	fn unregister(&self, entity_world: &mut EntityWorldMut) {
		let system_id = unsafe {
			let world = entity_world.world_mut();
			world.register_system_cached(self.system.clone())
		};
		if let Some(mut handle) = entity_world.get_mut::<H>()
			&& let Some(i) = handle
				.iter()
				.enumerate()
				.find_map(|(i, &s)| (s == system_id).then_some(i))
		{
			handle.swap_remove(i);
		}
	}
}
