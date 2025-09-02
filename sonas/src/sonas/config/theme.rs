use bevy_ecs::resource::Resource;
use serde::Deserialize;

#[derive(Debug, Deserialize, Resource)]
pub struct Theme {}
