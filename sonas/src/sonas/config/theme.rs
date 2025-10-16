use oprabeli::bevy_ecs;
use oprabeli::bevy_ecs::resource::Resource;
use oprabeli::ratatui::style::Color;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Colours {
	pub background: Color,
	pub overlay: Color,
	pub border_active: Color,
	pub border_inactive: Color,
	pub border_error: Color,
}

#[derive(Debug, Deserialize, Resource)]
pub struct Theme {
	pub colours: Colours,
}
