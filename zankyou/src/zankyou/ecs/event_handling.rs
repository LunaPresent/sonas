use std::iter;

use bevy_ecs::{
	component::{Component, Mutable},
	entity::Entity,
	hierarchy::{ChildOf, Children},
	relationship::RelationshipTarget,
	resource::Resource,
	system::{Commands, In, Local, Query, Res, ResMut},
};
use crossterm::event::MouseEventKind;
use ratatui::layout::Position;

use crate::event::Event;

use super::{Area, UiComponent};

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

pub fn handle_input_event<C, E>(
	In(event): In<Event<E>>,
	focus: Res<Focus>,
	parents: Query<&ChildOf>,
	components: Query<&mut C>,
	commands: Commands,
) -> Option<Event<E>>
where
	C: UiComponent<E> + Component<Mutability = Mutable>,
	E: Send + Sync + Clone + 'static,
{
	bubble_event(focus.target, event, parents, components, commands)
}

pub fn handle_broadcast_event<C, E>(
	In(event): In<Event<E>>,
	components: Query<(&mut C, Entity)>,
	mut commands: Commands,
) -> Event<E>
where
	C: UiComponent<E> + Component<Mutability = Mutable>,
	E: Send + Sync + Clone + 'static,
{
	for (mut component, entity) in components {
		component.handle_event(&event, commands.entity(entity));
	}
	event
}

pub fn handle_mouse_event<C, E>(
	(In(event), In(root), In(x), In(y)): (In<Event<E>>, In<Entity>, In<u16>, In<u16>),
	mut clicked: Local<Option<Entity>>,
	mut cursor_pos: ResMut<CursorPos>,
	areas: Query<&Area>,
	children: Query<&Children>,
	parents: Query<&ChildOf>,
	components: Query<&mut C>,
	commands: Commands,
) -> Option<Event<E>>
where
	C: UiComponent<E> + Component<Mutability = Mutable>,
	E: Send + Sync + Clone + 'static,
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

	target.and_then(|t| bubble_event(t, event, parents, components, commands))
}

pub fn handle_target_event<C, E>(
	(In(event), In(target)): (In<Event<E>>, In<Entity>),
	parents: Query<&ChildOf>,
	components: Query<&mut C>,
	commands: Commands,
) -> Option<Event<E>>
where
	C: UiComponent<E> + Component<Mutability = Mutable>,
	E: Send + Sync + Clone + 'static,
{
	bubble_event(target, event, parents, components, commands)
}

fn bubble_event<C, E>(
	target: Entity,
	event: Event<E>,
	parents: Query<&ChildOf>,
	mut components: Query<&mut C>,
	mut commands: Commands,
) -> Option<Event<E>>
where
	C: UiComponent<E> + Component<Mutability = Mutable>,
	E: Send + Sync + Clone + 'static,
{
	for target in iter::once(target).chain(parents.iter_ancestors(target)) {
		let cmd = commands.entity(target);
		if let Ok(mut component) = components.get_mut(target)
			&& component.handle_event(&event, cmd) == EventFlow::Consume
		{
			return None;
		}
	}
	Some(event)
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
