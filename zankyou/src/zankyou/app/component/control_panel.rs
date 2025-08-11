use bevy_ecs::system::Query;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::{
	app::app_event::AppEvent,
	ecs::{Area, UiComponent, Viewport},
};

#[derive(Debug, Default)]
pub struct ControlPanelComponent {}

impl UiComponent<AppEvent> for ControlPanelComponent {
	fn render(
		&self,
		area: Rect,
		buf: &mut Buffer,
		_children: Query<(&mut Area, Option<&mut Viewport>)>,
	) {
	}
}
