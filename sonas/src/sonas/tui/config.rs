mod config_manager;
mod keys;
mod util;

pub use config_manager::*;
pub use keys::*;

use bevy_ecs::resource::Resource;
use derive_more::{Deref, DerefMut};

/// A bevy resouce wrapper around the user defined config
///
/// The recommended way to spawn this in the ecs is using [`ConfigManager`]
#[derive(Debug, Resource, Deref, DerefMut)]
pub struct Config<C>(C);
