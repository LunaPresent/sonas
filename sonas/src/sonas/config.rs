mod config_manager;
mod input_action;
mod keys;
mod theme;

pub use config_manager::ConfigManager;
pub use keys::Keys;
pub use theme::Theme;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct AppConfig {
	keys: Keys,
	theme: Theme,
}
