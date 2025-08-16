use bevy_ecs::{
	entity::Entity,
	hierarchy::{ChildOf, Children},
	relationship::RelationshipTarget,
	resource::Resource,
	system::{In, InMut, InRef, Local, Query, Res, ResMut, RunSystemOnce as _},
	world::World,
};
use color_eyre::eyre;
use crossterm::event::MouseEventKind;
use ratatui::layout::Position;

use super::Area;
use crate::{
	ecs::ui_component::{UpdateHandle, UpdateSystemId},
	event::{Dispatch, Event, EventDispatch},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum EventFlow {
	Consume,
	Propagate,
}

#[derive(Debug, Resource)]
pub struct Focus {
	pub target: Entity,
}

#[derive(Debug, Resource)]
pub struct CursorPos {
	pub x: u16,
	pub y: u16,
}

impl Default for CursorPos {
	fn default() -> Self {
		Self {
			x: u16::MAX,
			y: u16::MAX,
		}
	}
}

#[derive(Debug)]
struct EntityUpdateInfo<E>
where
	E: 'static,
{
	entity: Entity,
	system: UpdateSystemId<E>,
}

#[derive(Debug)]
pub struct UpdateContext<E>
where
	E: 'static,
{
	update_queue: Vec<EntityUpdateInfo<E>>,
}

impl<E> Default for UpdateContext<E> {
	fn default() -> Self {
		Self {
			update_queue: Default::default(),
		}
	}
}

impl<E> UpdateContext<E>
where
	E: 'static,
{
	pub fn handle_event(
		&mut self,
		ed: EventDispatch<E>,
		world: &mut World,
		root_entity: Entity,
	) -> eyre::Result<Option<Event<E>>> {
		self.update_queue.clear();

		match ed.dispatch {
			Dispatch::Input => {
				world.run_system_once_with(Self::find_input_entities, &mut self.update_queue)?
			}
			Dispatch::Broadcast => {
				world.run_system_once_with(Self::find_broadcast_entities, &mut self.update_queue)?
			}
			Dispatch::Cursor { x, y } => world.run_system_cached_with(
				Self::find_cursor_entities,
				(&mut self.update_queue, &ed.event, root_entity, x, y),
			)?,
			Dispatch::Target(target) => world.run_system_once_with(
				Self::find_target_entities,
				(&mut self.update_queue, target),
			)?,
		}

		Ok(match ed.dispatch {
			Dispatch::Broadcast => {
				for target in self.update_queue.iter() {
					world.run_system_with(target.system, (target.entity, &ed.event))??;
				}
				Some(ed.event)
			}
			_ => {
				let mut full_propagate = true;
				for target in self.update_queue.iter() {
					let flow =
						world.run_system_with(target.system, (target.entity, &ed.event))??;
					if flow == EventFlow::Consume {
						full_propagate = false;
						break;
					}
				}
				if full_propagate { Some(ed.event) } else { None }
			}
		})
	}

	fn find_input_entities(
		InMut(targets): InMut<Vec<EntityUpdateInfo<E>>>,
		focus: Res<Focus>,
		handles: Query<&UpdateHandle<E>>,
		parents: Query<&ChildOf>,
	) {
		Self::bubble_entities(focus.target, targets, handles, parents);
	}

	fn find_broadcast_entities(
		InMut(targets): InMut<Vec<EntityUpdateInfo<E>>>,
		components: Query<(Entity, &UpdateHandle<E>)>,
	) {
		for (entity, handle) in components {
			targets.push(EntityUpdateInfo {
				entity,
				system: **handle,
			});
		}
	}

	#[allow(
		clippy::type_complexity,
		reason = "separating the tuple into a typedef makes it less clear what is going on"
	)]
	fn find_cursor_entities(
		(InMut(targets), InRef(event), In(root), In(x), In(y)): (
			InMut<Vec<EntityUpdateInfo<E>>>,
			InRef<Event<E>>,
			In<Entity>,
			In<u16>,
			In<u16>,
		),
		mut clicked: Local<Option<Entity>>,
		mut cursor_pos: ResMut<CursorPos>,
		areas: Query<&Area>,
		children: Query<&Children>,
		handles: Query<&UpdateHandle<E>>,
		parents: Query<&ChildOf>,
	) {
		cursor_pos.x = x;
		cursor_pos.y = y;

		let target = if let Event::Mouse(mouse_event) = event
			&& let MouseEventKind::Up(_) | MouseEventKind::Drag(_) = mouse_event.kind
		{
			*clicked
		} else {
			Some(Self::find_cursor_target(root, x, y, areas, children))
		};
		if let Event::Mouse(mouse_event) = event {
			match mouse_event.kind {
				MouseEventKind::Down(_) => *clicked = target,
				MouseEventKind::Up(_) => *clicked = None,
				_ => (),
			}
		}

		if let Some(target) = target {
			Self::bubble_entities(target, targets, handles, parents);
		}
	}

	fn find_target_entities(
		(InMut(targets), In(target)): (InMut<Vec<EntityUpdateInfo<E>>>, In<Entity>),
		handles: Query<&UpdateHandle<E>>,
		parents: Query<&ChildOf>,
	) {
		Self::bubble_entities(target, targets, handles, parents);
	}

	fn bubble_entities(
		head: Entity,
		targets: &mut Vec<EntityUpdateInfo<E>>,
		handles: Query<&UpdateHandle<E>>,
		parents: Query<&ChildOf>,
	) {
		if let Ok(handle) = handles.get(head) {
			targets.push(EntityUpdateInfo {
				entity: head,
				system: **handle,
			});
		}
		if let Ok(parent) = parents.get(head) {
			Self::bubble_entities(parent.parent(), targets, handles, parents);
		}
	}

	fn find_cursor_target(
		entity: Entity,
		x: u16,
		y: u16,
		areas: Query<&Area>,
		children: Query<&Children>,
	) -> Entity {
		for child in children
			.get(entity)
			.into_iter()
			.flat_map(RelationshipTarget::iter)
		{
			if let Ok(area) = areas.get(child)
				&& area.contains(Position { x, y })
			{
				return Self::find_cursor_target(child, x, y, areas, children);
			}
		}
		entity
	}
}
