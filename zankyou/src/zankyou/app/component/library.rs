use bevy_ecs::component::Component;

use crate::ecs::Area;

#[derive(Debug, Component, Default)]
#[require(Area)]
pub struct LibraryComponent {}
