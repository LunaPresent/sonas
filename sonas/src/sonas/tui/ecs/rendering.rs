use bevy_ecs::{
	component::Component,
	entity::Entity,
	hierarchy::{ChildOf, Children},
	query::Without,
	system::{In, InMut, Query, RunSystemOnce},
	world::World,
};
use color_eyre::eyre;
use derive_more::{Deref, DerefMut};
use ratatui::{
	buffer::Buffer,
	layout::{Position, Rect, Size},
};
use thiserror::Error;

use super::ui_component::{RenderHandle, RenderSystemId};

// TODO: documentation
#[derive(Debug, Component, Default, Clone, Copy, Deref, DerefMut)]
pub struct Area(Rect);

// TODO: documentation
#[derive(Debug, Component, Default)]
#[require(Area)]
pub struct Viewport {
	buf: Buffer,
	pub offset: Position,
	pub size: Size,
}

// TODO: documentation
#[derive(Debug, Error)]
pub enum ViewportError {
	#[error("viewport offset out of bounds")]
	OffsetOutOfBounds,
	#[error("viewport size too small for target area")]
	TooSmall,
	#[error("viewport component no longer exists")]
	Missing,
}

impl Viewport {
	pub fn area(&self) -> Rect {
		(Position::ORIGIN, self.size).into()
	}

	pub fn clamp_offset(&mut self, target_area_size: Size) -> Result<(), ViewportError> {
		if self.size.width < target_area_size.width || self.size.height < target_area_size.height {
			return Err(ViewportError::TooSmall);
		}

		self.offset.x = self.offset.x.min(self.size.width - target_area_size.width);
		self.offset.y = self
			.offset
			.y
			.min(self.size.height - target_area_size.height);
		Ok(())
	}

	fn resize_and_get_buf_mut(&mut self) -> &mut Buffer {
		self.buf.resize(self.area());
		&mut self.buf
	}
}

#[derive(Debug)]
struct EntityRenderInfo {
	entity: Entity,
	system: Option<RenderSystemId>,
	child_count: usize,
}

#[derive(Debug)]
struct ViewportLease {
	entity: Entity,
	end: usize,
}

#[derive(Debug, Default)]
pub(super) struct RenderContext {
	render_queue: Vec<EntityRenderInfo>,
	viewport_lease_stack: Vec<ViewportLease>,
}

impl RenderContext {
	pub fn render(&mut self, buf: &mut Buffer, area: Rect, world: &mut World) -> eyre::Result<()> {
		self.render_queue.clear();
		self.viewport_lease_stack.clear();

		world.run_system_once_with(Self::set_area_sizes, area)?;
		world.run_system_once_with(Self::find_render_targets, &mut self.render_queue)??;

		for (i, target) in self.render_queue.iter().enumerate() {
			if let Some(system) = target.system {
				world.run_system_with(system, (target.entity, buf))??;
			}

			world.run_system_once_with(
				Self::handle_viewports,
				(
					&mut self.viewport_lease_stack,
					buf,
					target.entity,
					i,
					target.child_count,
				),
			)??;
		}

		Ok(())
	}

	fn set_area_sizes(In(frame_area): In<Rect>, areas: Query<&mut Area>) {
		for mut area in areas {
			**area = frame_area;
		}
	}

	fn find_render_targets(
		InMut(targets): InMut<Vec<EntityRenderInfo>>,
		root_entities: Query<Entity, Without<ChildOf>>,
		query: Query<(Option<&RenderHandle>, Option<&Children>)>,
	) -> eyre::Result<()> {
		for root in root_entities {
			Self::recurse_render_targets(root, targets, query)?;
		}
		Ok(())
	}

	fn recurse_render_targets(
		head: Entity,
		targets: &mut Vec<EntityRenderInfo>,
		query: Query<(Option<&RenderHandle>, Option<&Children>)>,
	) -> eyre::Result<usize> {
		let idx = targets.len();
		let (handle, children) = query.get(head)?;
		let context = EntityRenderInfo {
			entity: head,
			system: handle.map(|handle| **handle),
			child_count: 0,
		};
		targets.push(context);

		let mut child_count = 0;
		if let Some(children) = children {
			for &child in children {
				child_count += Self::recurse_render_targets(child, targets, query)?;
			}
		}
		targets[idx].child_count = child_count;
		Ok(child_count + 1)
	}

	#[allow(
		clippy::type_complexity,
		reason = "separating the tuple into a typedef makes it less clear what is going on"
	)]
	fn handle_viewports(
		(InMut(lease_stack), InMut(buf), In(entity), In(index), In(child_count)): (
			InMut<Vec<ViewportLease>>,
			InMut<Buffer>,
			In<Entity>,
			In<usize>,
			In<usize>,
		),
		mut query: Query<Option<(&mut Viewport, &Area)>>,
	) -> eyre::Result<()> {
		if let Some((mut viewport, _)) = query.get_mut(entity)? {
			std::mem::swap(viewport.resize_and_get_buf_mut(), buf);
			buf.reset();
			lease_stack.push(ViewportLease {
				entity,
				end: index + child_count,
			});
		}

		while let Some(lease) = lease_stack.last()
			&& index == lease.end
		{
			let (mut viewport, area) =
				query.get_mut(lease.entity)?.ok_or(ViewportError::Missing)?;
			std::mem::swap(&mut viewport.buf, buf);
			lease_stack.pop();

			Self::combine_viewports(buf, &viewport, **area)?;
		}

		Ok(())
	}

	fn combine_viewports(
		dst: &mut Buffer,
		src: &Viewport,
		area: Rect,
	) -> Result<(), ViewportError> {
		let rect = area.intersection(dst.area);
		if src.size.width < rect.width || src.size.height < rect.height {
			return Err(ViewportError::TooSmall);
		}
		if src.offset.x > src.size.width - rect.width
			|| src.offset.y > src.size.height - rect.height
		{
			return Err(ViewportError::OffsetOutOfBounds);
		}

		for y_off in 0..rect.height {
			let y_src = src.offset.y + y_off;
			let i_src = src.buf.index_of(src.offset.x, y_src);
			let i_dst = dst.index_of(rect.x, rect.y + y_off);

			dst.content[i_dst..i_dst + rect.width as usize]
				.clone_from_slice(&src.buf.content[i_src..i_src + rect.width as usize]);
		}

		Ok(())
	}
}
