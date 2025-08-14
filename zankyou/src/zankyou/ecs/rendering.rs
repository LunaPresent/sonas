use std::iter;

use bevy_ecs::{
	component::Component,
	entity::Entity,
	hierarchy::Children,
	system::{In, InMut, Query},
};
use derive_more::{Deref, DerefMut};
use ratatui::{buffer::Buffer, layout::Rect};

use crate::ecs::ui_component::{RenderHandle, RenderSystemId};

#[derive(Debug, Component, Default, Deref, DerefMut)]
pub struct Area(Rect);

#[derive(Debug, Component, Default)]
#[require(Area)]
pub struct Viewport {
	pub buf: Buffer,
	pub offset_x: u16,
	pub offset_y: u16,
}

#[derive(Debug)]
pub struct RenderContext {
	pub entity: Entity,
	pub system: RenderSystemId,
}

pub fn find_render_targets(
	(InMut(targets), In(root)): (InMut<Vec<RenderContext>>, In<Entity>),
	children: Query<&Children>,
	handles: Query<&RenderHandle>,
) {
	for entity in iter::once(root).chain(children.iter_descendants(root)) {
		if let Ok(handle) = handles.get(entity) {
			targets.push(RenderContext {
				entity,
				system: **handle,
			});
		}
	}
}

// fn combine_viewports(dst: &mut Buffer, src: &Viewport, area: Rect) {
// 	let rect = area.intersection(dst.area);
//
// 	for y_off in 0..rect.height {
// 		let y_src = src.offset_y + y_off;
// 		let i_src = src.buf.index_of(src.offset_x, y_src);
// 		let i_dst = dst.index_of(rect.x, rect.y + y_off);
//
// 		dst.content[i_dst..i_dst + rect.width as usize]
// 			.clone_from_slice(&src.buf.content[i_src..i_src + rect.width as usize]);
// 	}
// }
