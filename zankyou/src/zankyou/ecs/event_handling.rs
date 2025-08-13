use bevy_ecs::{
	entity::Entity,
	hierarchy::{ChildOf, Children},
	relationship::RelationshipTarget,
	resource::Resource,
	system::{In, InMut, InRef, Local, Query, Res, ResMut, SystemId},
};
use crossterm::event::MouseEventKind;
use ratatui::layout::Position;

use super::Area;
use crate::{ecs::ui_component::UpdateHandle, event::Event};

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
pub struct UpdateContext<E>
where
	E: 'static,
{
	pub entity: Entity,
	pub system: SystemId<(In<Entity>, InRef<'static, Event<E>>), EventFlow>,
}

pub fn find_input_entities<E>(
	InMut(targets): InMut<Vec<UpdateContext<E>>>,
	focus: Res<Focus>,
	handles: Query<&UpdateHandle<E>>,
	parents: Query<&ChildOf>,
) where
	E: 'static,
{
	bubble_entities(focus.target, targets, handles, parents);
}

pub fn find_broadcast_entities<E>(
	InMut(targets): InMut<Vec<UpdateContext<E>>>,
	components: Query<(Entity, &UpdateHandle<E>)>,
) where
	E: 'static,
{
	for (entity, handle) in components {
		targets.push(UpdateContext {
			entity,
			system: **handle,
		});
	}
}

pub fn find_cursor_entities<E>(
	(InMut(targets), InRef(event), In(root), In(x), In(y)): (
		InMut<Vec<UpdateContext<E>>>,
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
) where
	E: 'static,
{
	cursor_pos.x = x;
	cursor_pos.y = y;

	let target = if let Event::Mouse(mouse_event) = event
		&& let MouseEventKind::Up(_) | MouseEventKind::Drag(_) = mouse_event.kind
	{
		*clicked
	} else {
		Some(find_cursor_target(root, x, y, areas, children))
	};
	if let Event::Mouse(mouse_event) = event {
		match mouse_event.kind {
			MouseEventKind::Down(_) => *clicked = target,
			MouseEventKind::Up(_) => *clicked = None,
			_ => (),
		}
	}

	if let Some(target) = target {
		bubble_entities(target, targets, handles, parents);
	}
}

pub fn find_target_entities<E>(
	(InMut(targets), In(target)): (InMut<Vec<UpdateContext<E>>>, In<Entity>),
	handles: Query<&UpdateHandle<E>>,
	parents: Query<&ChildOf>,
) where
	E: 'static,
{
	bubble_entities(target, targets, handles, parents);
}

fn bubble_entities<E>(
	head: Entity,
	targets: &mut Vec<UpdateContext<E>>,
	handles: Query<&UpdateHandle<E>>,
	parents: Query<&ChildOf>,
) where
	E: 'static,
{
	if let Ok(handle) = handles.get(head) {
		targets.push(UpdateContext {
			entity: head,
			system: **handle,
		});
	}
	if let Ok(parent) = parents.get(head) {
		bubble_entities(parent.parent(), targets, handles, parents);
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
			return find_cursor_target(child, x, y, areas, children);
		}
	}
	entity
}
