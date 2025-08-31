mod config_manager;
mod input_action;
mod keys;

pub use config_manager::ConfigManager;
pub use keys::Keys;

use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Config {
	pub keys: Keys,
}
