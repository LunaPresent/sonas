mod app_event;
mod component;

use color_eyre::eyre;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
	ecs::ComponentSystem,
	event::{AppEvent as _, Dispatch, Event, EventDispatch, EventQueue},
	tui::Tui,
};
use app_event::AppEvent;
use component::GenericComponent;

#[derive(Debug)]
pub struct App {
	pub should_quit: bool,
	pub events: EventQueue<AppEvent>,
	pub ecs: ComponentSystem<GenericComponent, AppEvent>,
}

impl Default for App {
	fn default() -> Self {
		Self {
			should_quit: false,
			events: EventQueue::new(),
			ecs: ComponentSystem::new(),
		}
	}
}

impl App {
	pub fn new() -> Self {
		Self::default()
	}

	pub async fn run(mut self) -> eyre::Result<()> {
		let mut tui = Tui::new()?;
		tui.enter()?;
		while !self.should_quit {
			let ed = self.events.next().await?;
			self.handle_event(&mut tui, ed)?;
		}
		tui.exit()?;
		Ok(())
	}

	fn handle_event(&mut self, tui: &mut Tui, ed: EventDispatch<AppEvent>) -> eyre::Result<()> {
		if let Some(event) = self.ecs.handle_event(ed)? {
			match event {
				Event::Tick => {
					tui.draw(|frame| self.ecs.draw(frame))?;
				}
				Event::Key(key_event) => {
					if let Some(event) = self.map_key_events(key_event) {
						self.handle_event(
							tui,
							EventDispatch {
								dispatch: Dispatch::Input,
								event,
							},
						)?;
					}
				}
				Event::App(app_event) if app_event.is_quit() => self.should_quit = true,
				_ => (),
			}
		}
		Ok(())
	}

	fn map_key_events(&mut self, key_event: KeyEvent) -> Option<Event<AppEvent>> {
		match key_event.code {
			KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
				Some(Event::App(AppEvent::Quit))
			}
			KeyCode::Esc | KeyCode::Char('q') => Some(Event::App(AppEvent::Quit)),
			KeyCode::Char('k') => Some(Event::App(AppEvent::Increment)),
			KeyCode::Char('j') => Some(Event::App(AppEvent::Decrement)),
			_ => None,
		}
	}
}
