use super::input_action::InputAction;
use crate::tui::config::KeyConfig;
use bevy_ecs::resource::Resource;
use derive_more::Deref;
use serde::Deserialize;

#[derive(Debug, Deserialize, Resource, Deref)]
pub struct Keys(KeyConfig<InputAction>);
