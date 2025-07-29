use crate::{
	app::component::{ComponentSystem, RootComponent},
	event::{AppEvent, Event, EventHandler},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;

mod component;

#[derive(Debug)]
pub struct App {
	pub should_quit: bool,
	pub counter: u8,
	pub events: EventHandler,
	pub components: ComponentSystem<RootComponent>,
}

impl Default for App {
	fn default() -> Self {
		Self {
			should_quit: false,
			counter: 0,
			events: EventHandler::new(),
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
			terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
			let event = match self.events.next().await? {
				Event::Crossterm(event) => match event {
					crossterm::event::Event::Key(key_event) => self
						.map_key_events(key_event)
						.map(|e| Event::Input(e))
						.unwrap_or(Event::Crossterm(event)),
					e => Event::Crossterm(e),
				},
				e => e,
			};
			if let Some(e) = self.components.handle_event(&event) {
				match e {
					&AppEvent::Quit => self.should_quit = true,
					_ => (),
				}
			}
		}
		Ok(())
	}

	fn map_key_events(&mut self, key_event: KeyEvent) -> Option<AppEvent> {
		match key_event.code {
			KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
				Some(AppEvent::Quit)
			}
			KeyCode::Esc | KeyCode::Char('q') => Some(AppEvent::Quit),
			KeyCode::Right => Some(AppEvent::Increment),
			KeyCode::Left => Some(AppEvent::Decrement),
			_ => None,
		}
	}
}
