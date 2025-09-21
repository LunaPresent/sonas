use bevy_ecs::{component::Component, entity::Entity};
use color_eyre::eyre;

use super::App;

#[derive(Debug)]
pub struct EntityBuilder<T>
where
	T: 'static,
{
	entity: Entity,
	app: App<T>,
}

#[allow(dead_code)]
impl<T> EntityBuilder<T>
where
	T: Send + Sync + 'static,
{
	pub fn new(entity: Entity, app: App<T>) -> Self {
		Self { entity, app }
	}

	pub fn id(&self) -> Entity {
		self.entity
	}

	pub fn app(self) -> App<T> {
		self.app
	}

	pub fn with_child(mut self, f: impl FnOnce(Self) -> eyre::Result<Self>) -> eyre::Result<Self> {
		let entity = self.entity;
		let app = (f)(EntityBuilder::new(self.app.ecs.add_entity(), self.app))?.app;
		Ok(Self::new(entity, app))
	}

	pub fn with_component(mut self, component_bundle: impl Component) -> eyre::Result<Self> {
		self.app.ecs.add_component(self.entity, component_bundle);
		self.app.ecs.init()?;
		Ok(self)
	}
}
