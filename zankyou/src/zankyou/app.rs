use crate::{
	app::component::{ComponentSystem, EventFlow, RootComponent},
	event::{AppEvent, Dispatch, Event, EventDispatch, EventQueue},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;

mod component;

#[derive(Debug)]
pub struct App {
	pub should_quit: bool,
	pub counter: u8,
	pub events: EventQueue,
	pub components: ComponentSystem<RootComponent>,
}

impl Default for App {
	fn default() -> Self {
		Self {
			should_quit: false,
			counter: 0,
			events: EventQueue::new(),
			components: ComponentSystem::new(RootComponent::default()),
		}
	}
}

impl App {
	pub fn new() -> Self {
		Self::default()
	}

	pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
		while !self.should_quit {
			self.components.draw(&mut terminal)?;
			let ed = self.events.next().await?;
			self.handle_event(ed);
		}
		Ok(())
	}

	fn handle_event(&mut self, ed: EventDispatch) {
		if self.components.handle_event(&ed) == EventFlow::Propagate {
			match ed.event() {
				Event::Crossterm(event) => match *event {
					crossterm::event::Event::Key(key_event) => {
						if let Some(event) = self.map_key_events(key_event) {
							self.handle_event(EventDispatch::new(
								Dispatch::Input,
								Event::AppEvent(event),
							));
						}
					}
					_ => (),
				},
				Event::AppEvent(AppEvent::Quit) => self.should_quit = true,
				_ => (),
			}
		}
	}

	fn map_key_events(&mut self, key_event: KeyEvent) -> Option<AppEvent> {
		match key_event.code {
			KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
				Some(AppEvent::Quit)
			}
			KeyCode::Esc | KeyCode::Char('q') => Some(AppEvent::Quit),
			KeyCode::Char('k') => Some(AppEvent::Increment),
			KeyCode::Char('j') => Some(AppEvent::Decrement),
			_ => None,
		}
	}
}
