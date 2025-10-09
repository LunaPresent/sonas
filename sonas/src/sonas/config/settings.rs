use core::time::Duration;

use bevy_ecs::resource::Resource;
use serde::Deserialize;
use serde_with::{DurationSecondsWithFrac, serde_as};

#[serde_as]
#[derive(Debug, Deserialize, Resource)]
#[serde(rename_all = "kebab-case")]
pub struct Settings {
	#[serde_as(as = "DurationSecondsWithFrac<f64>")]
	pub notification_timeout: Duration,
}
