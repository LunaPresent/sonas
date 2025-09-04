use bevy_ecs::resource::Resource;
use ratatui::style::Color;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Colours {
	pub background: Color,
	pub overlay: Color,
	pub border_active: Color,
	pub border_inactive: Color,
}

#[derive(Debug, Deserialize, Resource)]
pub struct Theme {
	pub colours: Colours,
}
