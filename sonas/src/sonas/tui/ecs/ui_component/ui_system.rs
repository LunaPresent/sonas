use std::marker::PhantomData;

use bevy_ecs::{system::IntoSystem, world::EntityWorldMut};

use super::UiSystemContext;

// TODO: documentation
pub struct UiSystem {
	boxed_system: Box<dyn GenericSystem + Sync + Send>,
}

impl UiSystem {
	// TODO: documentation
	pub fn new<C, M, S>(system: S) -> Self
	where
		C: UiSystemContext + 'static,
		M: 'static,
		S: IntoSystem<C, C::Result, M> + Sync + Send + Clone + 'static,
	{
		Self {
			boxed_system: Box::new(GenericSystemImpl::<C, M, S> {
				system,
				phantom_data: PhantomData,
			}),
		}
	}

	pub(crate) fn register(&self, entity_world: &mut EntityWorldMut) {
		self.boxed_system.register(entity_world);
	}

	pub(crate) fn unregister(&self, entity_world: &mut EntityWorldMut) {
		self.boxed_system.unregister(entity_world);
	}
}

trait GenericSystem {
	fn register(&self, entity_world: &mut EntityWorldMut);
	fn unregister(&self, entity_world: &mut EntityWorldMut);
}

struct GenericSystemImpl<C, M, S> {
	system: S,
	phantom_data: PhantomData<fn() -> (C, M)>,
}

impl<C, M, S> GenericSystem for GenericSystemImpl<C, M, S>
where
	C: UiSystemContext + 'static,
	S: IntoSystem<C, C::Result, M> + Clone + 'static,
{
	fn register(&self, entity_world: &mut EntityWorldMut) {
		let system_id = unsafe {
			let world = entity_world.world_mut();
			world.register_system_cached(self.system.clone())
		};
		entity_world.insert_if_new(C::Handle::default());
		let mut handle = entity_world
			.get_mut::<C::Handle>()
			.expect("Handle should have just been inserted");
		handle.push(system_id);
	}

	fn unregister(&self, entity_world: &mut EntityWorldMut) {
		let system_id = unsafe {
			let world = entity_world.world_mut();
			world.register_system_cached(self.system.clone())
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
