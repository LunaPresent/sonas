use super::input_action::InputAction;
use bevy_ecs::resource::Resource;
use derive_more::Deref;
use oprabeli::config::KeyConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize, Resource, Deref)]
pub struct Keys(KeyConfig<InputAction>);
