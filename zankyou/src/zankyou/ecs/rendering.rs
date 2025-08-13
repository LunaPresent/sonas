use std::iter;

use bevy_ecs::{
	component::Component,
	entity::Entity,
	hierarchy::Children,
	system::{In, InMut, Query, SystemId},
};
use derive_more::{Deref, DerefMut};
use ratatui::{buffer::Buffer, layout::Rect};

use crate::ecs::ui_component::RenderHandle;

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
	pub system: SystemId<(In<Entity>, InMut<'static, Buffer>), ()>,
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

// pub fn render(
// 	(In(entity), InMut(buf)): (In<Entity>, InMut<Buffer>),
// 	components: Query<(&C, Option<&Children>), With<Area>>,
// 	areas: Query<(&mut Area, Option<&mut Viewport>)>,
// ) {
// 	render_recursive(entity, buf, components, areas);
// }
//
// fn render_recursive<C: UiComponent<E> + Component, E>(
// 	entity: Entity,
// 	buf: &mut Buffer,
// 	components: Query<(&C, Option<&Children>), With<Area>>,
// 	mut areas: Query<(&mut Area, Option<&mut Viewport>)>,
// ) {
// 	if let Ok((comp, children)) = components.get(entity) {
// 		unsafe {
// 			match areas.reborrow_unsafe().get_mut(entity) {
// 				Ok((area, Some(mut vp))) => {
// 					comp.render(vp.buf.area, &mut vp.buf, areas.reborrow_unsafe());
// 					if let Some(children) = children {
// 						for &child in children {
// 							render_recursive(
// 								child,
// 								&mut vp.buf,
// 								components,
// 								areas.reborrow_unsafe(),
// 							);
// 						}
// 					}
// 					combine_viewports(buf, &vp, area.0);
// 				}
// 				Ok((area, None)) => {
// 					comp.render(area.0, buf, areas.reborrow());
// 					if let Some(children) = children {
// 						for &child in children {
// 							render_recursive(child, buf, components, areas.reborrow());
// 						}
// 					}
// 				}
// 				_ => (),
// 			}
// 		}
// 	}
// }
//
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
