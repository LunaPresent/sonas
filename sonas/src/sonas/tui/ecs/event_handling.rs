mod async_queue;
mod cursor_pos;
mod flow;
mod focus;
mod queue;

pub use async_queue::AsyncEventQueue;
pub use cursor_pos::CursorPos;
pub use flow::EventFlow;
pub use focus::Focus;
pub use queue::EventQueue;

use bevy_ecs::{
	entity::Entity,
	hierarchy::{ChildOf, Children},
	query::Without,
	system::{In, InMut, InRef, Local, Query, Res, ResMut, RunSystemOnce as _},
	world::World,
};
use color_eyre::eyre;
use crossterm::event::{MouseEvent, MouseEventKind};
use ratatui::layout::Position;

use crate::tui::ecs::UpdateContext;

use super::{
	Area, Dispatch, Event, EventDispatch, Viewport,
	ui_component::{UpdateSystemCollection, UpdateSystemId},
};

#[derive(Debug)]
struct EntityUpdateInfo<T>
where
	T: 'static,
{
	entity: Entity,
	system: UpdateSystemId<T>,
}

#[derive(Debug)]
pub(crate) struct UpdateSystemRunner<T>
where
	T: 'static,
{
	update_queue: Vec<EntityUpdateInfo<T>>,
}

impl<T> Default for UpdateSystemRunner<T> {
	fn default() -> Self {
		Self {
			update_queue: Default::default(),
		}
	}
}

impl<T> UpdateSystemRunner<T>
where
	T: 'static,
{
	pub fn handle_event(
		&mut self,
		ed: EventDispatch<T>,
		world: &mut World,
	) -> eyre::Result<Option<Event<T>>> {
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
				(&mut self.update_queue, &ed.event, x, y),
			)??,
			Dispatch::Target(target) => world.run_system_once_with(
				Self::find_target_entities,
				(&mut self.update_queue, target),
			)?,
		}

		Ok(match ed.dispatch {
			Dispatch::Broadcast => {
				for target in &self.update_queue {
					world.run_system_with(
						target.system,
						UpdateContext {
							entity: target.entity,
							event: &ed.event,
						},
					)??;
				}
				Some(ed.event)
			}
			_ => self.target_dispatch(ed.event, world)?,
		})
	}

	fn target_dispatch(
		&self,
		event: Event<T>,
		world: &mut World,
	) -> eyre::Result<Option<Event<T>>> {
		let mut consume = false;
		let mut prev_entity = Entity::PLACEHOLDER;

		for target in &self.update_queue {
			// make multiple systems on one entity order independent by firing all of them even if
			// one returned consume
			if consume && prev_entity != target.entity {
				break;
			}
			prev_entity = target.entity;

			let flow = world.run_system_with(
				target.system,
				UpdateContext {
					entity: target.entity,
					event: &event,
				},
			)??;
			match flow {
				EventFlow::Consume => consume = true,
				EventFlow::Propagate => (),
			}
		}
		if consume { Ok(None) } else { Ok(Some(event)) }
	}

	fn find_input_entities(
		InMut(targets): InMut<Vec<EntityUpdateInfo<T>>>,
		focus: Res<Focus>,
		handles: Query<&UpdateSystemCollection<T>>,
		parents: Query<&ChildOf>,
	) {
		Self::bubble_entities(focus.target, targets, handles, parents);
	}

	fn find_broadcast_entities(
		InMut(targets): InMut<Vec<EntityUpdateInfo<T>>>,
		components: Query<(Entity, &UpdateSystemCollection<T>)>,
	) {
		for (entity, handle) in components {
			for &system in handle.iter() {
				targets.push(EntityUpdateInfo { entity, system });
			}
		}
	}

	#[allow(
		clippy::type_complexity,
		reason = "separating the tuple into a typedef makes it less clear what is going on"
	)]
	#[allow(
		clippy::too_many_arguments,
		reason = "most of the arguments are injected by bevy"
	)]
	fn find_cursor_entities(
		(InMut(targets), InRef(event), In(x), In(y)): (
			InMut<Vec<EntityUpdateInfo<T>>>,
			InRef<Event<T>>,
			In<u16>,
			In<u16>,
		),
		mut clicked: Local<Option<Entity>>,
		mut cursor_pos: ResMut<CursorPos>,
		broadcast_components: Query<(Entity, &UpdateSystemCollection<T>)>,
		root_entities: Query<Entity, Without<ChildOf>>,
		areas: Query<(Option<&Area>, Option<&Children>, Option<&Viewport>)>,
		handles: Query<&UpdateSystemCollection<T>>,
		parents: Query<&ChildOf>,
	) -> eyre::Result<()> {
		cursor_pos.x = x;
		cursor_pos.y = y;

		let target = match event {
			Event::Mouse(MouseEvent {
				kind: MouseEventKind::Up(_) | MouseEventKind::Drag(_),
				..
			}) => *clicked,
			Event::Mouse(MouseEvent {
				kind: MouseEventKind::Moved,
				..
			}) => {
				for (entity, handle) in broadcast_components {
					for &system in handle.iter() {
						targets.push(EntityUpdateInfo { entity, system });
					}
				}
				return Ok(());
			}
			_ => Self::find_cursor_target(x, y, root_entities, areas)?,
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
		Ok(())
	}

	fn find_target_entities(
		(InMut(targets), In(target)): (InMut<Vec<EntityUpdateInfo<T>>>, In<Entity>),
		handles: Query<&UpdateSystemCollection<T>>,
		parents: Query<&ChildOf>,
	) {
		Self::bubble_entities(target, targets, handles, parents);
	}

	fn bubble_entities(
		head: Entity,
		targets: &mut Vec<EntityUpdateInfo<T>>,
		handles: Query<&UpdateSystemCollection<T>>,
		parents: Query<&ChildOf>,
	) {
		if let Ok(handle) = handles.get(head) {
			for &system in handle.iter() {
				targets.push(EntityUpdateInfo {
					entity: head,
					system,
				});
			}
		}
		if let Ok(parent) = parents.get(head) {
			Self::bubble_entities(parent.parent(), targets, handles, parents);
		}
	}

	fn find_cursor_target(
		x: u16,
		y: u16,
		root_entities: Query<Entity, Without<ChildOf>>,
		areas: Query<(Option<&Area>, Option<&Children>, Option<&Viewport>)>,
	) -> eyre::Result<Option<Entity>> {
		for entity in root_entities {
			if let Some(target) = Self::find_cursor_target_inner(Position { x, y }, entity, areas)?
			{
				return Ok(Some(target));
			}
		}
		Ok(None)
	}

	/// result is based on the following "truth table"
	///
	/// | has area | in area | has children | result........... |
	/// | -------- | ------- | ------------ | ----------------- |
	/// | .......0 | ....../ | ...........0 | none............. |
	/// | .......1 | ......0 | ...........0 | none............. |
	/// | .......1 | ......1 | ...........0 | self............. |
	/// | .......0 | ....../ | ...........1 | recurse then none |
	/// | .......1 | ......0 | ...........1 | none............. |
	/// | .......1 | ......1 | ...........1 | recurse then self |
	fn find_cursor_target_inner(
		mut pos: Position,
		entity: Entity,
		areas: Query<(Option<&Area>, Option<&Children>, Option<&Viewport>)>,
	) -> eyre::Result<Option<Entity>> {
		let (area, children, viewport) = areas.get(entity)?;
		if let Some(area) = area
			&& !area.contains(pos)
		{
			Ok(None)
		} else {
			if let (Some(area), Some(viewport)) = (area, viewport) {
				pos.x = pos.x - area.x + viewport.offset.x;
				pos.y = pos.y - area.y + viewport.offset.y;
			}
			if let Some(children) = children {
				for &child in children {
					if let Some(target) = Self::find_cursor_target_inner(pos, child, areas)? {
						return Ok(Some(target));
					}
				}
			}
			if area.is_some() {
				Ok(Some(entity))
			} else {
				Ok(None)
			}
		}
	}
}
