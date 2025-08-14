use std::{marker::PhantomData, sync::Arc};

use bevy_ecs::{
	component::{Component, HookContext},
	entity::Entity,
	system::{In, InMut, InRef, IntoSystem, SystemId, SystemInput},
	world::{DeferredWorld, World},
};
use color_eyre::eyre;
use derive_more::Deref;
use ratatui::buffer::Buffer;

use super::{Area, event_handling::EventFlow};
use crate::event::Event;

pub type InitInput = In<Entity>;
pub type UpdateInput<'a, E> = (In<Entity>, InRef<'a, Event<E>>);
pub type RenderInput<'a> = (In<Entity>, InMut<'a, Buffer>);

pub type InitSystemId = SystemId<InitInput, eyre::Result<()>>;
pub type UpdateSystemId<E> = SystemId<UpdateInput<'static, E>, eyre::Result<EventFlow>>;
pub type RenderSystemId = SystemId<RenderInput<'static>, eyre::Result<()>>;

#[derive(Component)]
#[component(on_add = Self::register_system)]
pub struct InitSystem {
	system_registrar: Arc<dyn GenericSystemRegistrar<InitInput, eyre::Result<()>> + Sync + Send>,
}

impl InitSystem {
	pub fn new<M, S>(system: S) -> Self
	where
		M: Sync + Send + 'static,
		S: IntoSystem<InitInput, eyre::Result<()>, M> + Sync + Send + Clone + 'static,
	{
		Self {
			system_registrar: Arc::new(SystemRegistrar::new(system)),
		}
	}

	// This function is just copy pasted across all three *System impls, with the sole
	// difference of the *Handle being different. You can however, and this is true,
	// kiss my DRY arse if you think I'm generalising this function
	fn register_system(mut world: DeferredWorld, context: HookContext) {
		world.commands().queue(move |world: &mut World| {
			let system_registrar = world
				.get::<Self>(context.entity)
				.expect("Unexpected error getting reference to system registrar")
				.system_registrar
				.clone();
			let system_id = system_registrar.register_system(world);
			let mut entity = world.get_entity_mut(context.entity)?;
			entity.insert(InitHandle(system_id));
			Ok::<_, eyre::Error>(())
		});
	}
}

#[derive(Component)]
#[component(on_add = Self::register_system)]
pub struct UpdateSystem<E>
where
	E: Sync + Send + 'static,
{
	system_registrar: Arc<
		dyn GenericSystemRegistrar<UpdateInput<'static, E>, eyre::Result<EventFlow>> + Sync + Send,
	>,
	phantom_data: PhantomData<E>,
}

impl<E> UpdateSystem<E>
where
	E: Sync + Send + 'static,
{
	pub fn new<M, S>(system: S) -> Self
	where
		M: Sync + Send + 'static,
		S: IntoSystem<UpdateInput<'static, E>, eyre::Result<EventFlow>, M>
			+ Sync
			+ Send
			+ Clone
			+ 'static,
	{
		Self {
			system_registrar: Arc::new(SystemRegistrar::new(system)),
			phantom_data: PhantomData,
		}
	}

	fn register_system(mut world: DeferredWorld, context: HookContext) {
		world.commands().queue(move |world: &mut World| {
			let system_registrar = world
				.get::<Self>(context.entity)
				.expect("Unexpected error getting reference to system registrar")
				.system_registrar
				.clone();
			let system_id = system_registrar.register_system(world);
			let mut entity = world.get_entity_mut(context.entity)?;
			entity.insert(UpdateHandle(system_id));
			Ok::<_, eyre::Error>(())
		});
	}
}

#[derive(Component)]
#[component(on_add = Self::register_system)]
pub struct RenderSystem {
	system_registrar:
		Arc<dyn GenericSystemRegistrar<RenderInput<'static>, eyre::Result<()>> + Sync + Send>,
}

impl RenderSystem {
	pub fn new<M, S>(system: S) -> Self
	where
		M: Sync + Send + 'static,
		S: IntoSystem<RenderInput<'static>, eyre::Result<()>, M> + Sync + Send + Clone + 'static,
	{
		Self {
			system_registrar: Arc::new(SystemRegistrar::new(system)),
		}
	}

	fn register_system(mut world: DeferredWorld, context: HookContext) {
		world.commands().queue(move |world: &mut World| {
			let system_registrar = world
				.get::<Self>(context.entity)
				.expect("Unexpected error getting reference to system registrar")
				.system_registrar
				.clone();
			let system_id = system_registrar.register_system(world);
			let mut entity = world.get_entity_mut(context.entity)?;
			entity.insert(RenderHandle(system_id));
			Ok::<_, eyre::Error>(())
		});
	}
}

#[derive(Debug, Component, Clone, Copy, Deref)]
pub struct InitHandle(InitSystemId);

#[derive(Debug, Component, Clone, Copy, Deref)]
pub struct UpdateHandle<E>(UpdateSystemId<E>)
where
	E: 'static;

#[derive(Debug, Component, Clone, Copy, Deref)]
#[require(Area)]
pub struct RenderHandle(RenderSystemId);

trait GenericSystemRegistrar<I, O>
where
	I: SystemInput + 'static,
	O: 'static,
{
	fn register_system(&self, world: &mut World) -> SystemId<I, O>;
}

#[derive(Debug)]
struct SystemRegistrar<I, O, M, S>
where
	I: SystemInput + 'static,
	O: 'static,
	M: Sync + Send + 'static,
	S: IntoSystem<I, O, M> + Sync + Send + Clone + 'static,
{
	system: S,
	phantom_data: PhantomData<(I, O, M)>,
}

impl<I, O, M, S> SystemRegistrar<I, O, M, S>
where
	I: SystemInput + 'static,
	O: 'static,
	M: Sync + Send + 'static,
	S: IntoSystem<I, O, M> + Sync + Send + Clone + 'static,
{
	pub fn new(system: S) -> Self {
		Self {
			system,
			phantom_data: PhantomData,
		}
	}
}

impl<I, O, M, S> GenericSystemRegistrar<I, O> for SystemRegistrar<I, O, M, S>
where
	I: SystemInput + 'static,
	O: 'static,
	M: Sync + Send + 'static,
	S: IntoSystem<I, O, M> + Sync + Send + Clone + 'static,
{
	fn register_system(&self, world: &mut World) -> SystemId<I, O> {
		world.register_system_cached(self.system.clone())
	}
}
