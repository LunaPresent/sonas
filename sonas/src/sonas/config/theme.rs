use bevy_ecs::resource::Resource;
use ratatui::style::Color;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Colours {
	pub background: Color,
	pub overlay: Color,
}

#[derive(Debug, Deserialize, Resource)]
pub struct Theme {
	pub colours: Colours,
}
