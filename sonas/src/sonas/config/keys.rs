use super::input_action::InputAction;
use derive_more::Deref;
use oprabeli::bevy_ecs;
use oprabeli::bevy_ecs::resource::Resource;
use oprabeli::config::KeyConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize, Resource, Deref)]
pub struct Keys(KeyConfig<InputAction>);
