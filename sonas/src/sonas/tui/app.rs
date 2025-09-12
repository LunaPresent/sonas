use std::io;

use bevy_ecs::bundle::Bundle;
use color_eyre::eyre;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::{
	ecs::ComponentSystem,
	event::{AppEvent, Event, EventSystem},
	terminal::Terminal,
};

/// Simple interface to setup and run your application
///
/// The `App` features a simple builder-style API with two actions:
/// - Add top-level components using [App::with_component] and [App::with_main_component]
/// - Run the app until completion with [App::run]
///
/// # Examples
/// ```
/// #[derive(Debug, PartialEq, Eq)]
/// enum MyAppEvent {
///     Quit,
///     CursorUp,
///     CursorDown,
/// }
/// impl AppEvent for MyAppEvent {
///     fn is_quit(&self) -> bool {
///         self == &Self::Quit
///     }
/// }
///
/// # #[derive(Default, bevy_ecs::component::Component)]
/// # struct ConfigManager;
/// # #[derive(Default, bevy_ecs::component::Component)]
/// # struct RootComponent;
///
/// let app = App::<MyAppEvent>::new()
///     .with_component(ConfigManager::default())?
///     .with_main_component(RootComponent::default())?;
/// app.run().await?;
/// ```
#[derive(Debug)]
pub struct App<T>
where
	T: 'static,
{
	should_quit: bool,
	should_suspend: bool,
	event_system: EventSystem<T>,
	ecs: ComponentSystem<T>,
}

impl<T> App<T>
where
	T: AppEvent + Send + Sync + 'static,
{
	/// Creates a new `App`
	pub fn new() -> Self {
		let event_system = EventSystem::new();
		let ecs = ComponentSystem::new(event_system.sender());
		Self {
			should_quit: false,
			should_suspend: false,
			event_system,
			ecs,
		}
	}

	/// Adds a new component to the bevy ecs
	pub fn with_component(mut self, component_bundle: impl Bundle) -> eyre::Result<Self> {
		self.ecs.add_component(component_bundle);
		self.ecs.init()?;
		Ok(self)
	}

	/// Adds a new component to the bevy ecs and focusses it
	pub fn with_main_component(mut self, component_bundle: impl Bundle) -> eyre::Result<Self> {
		let entity = self.ecs.add_component(component_bundle);
		self.ecs.set_focus(entity);
		self.ecs.init()?;
		Ok(self)
	}

	/// Runs the `App` until completion
	pub async fn run(mut self) -> eyre::Result<()> {
		let mut tui = Terminal::new()?;
		tui.enter()?;
		self.ecs.init()?;
		self.event_system.start()?;
		while !self.should_quit {
			let mut next_ed = Some(self.event_system.next().await?);
			while let Some(ed) = next_ed {
				let result = self.ecs.handle_event(ed)?;
				if let Some(event) = result.propagated {
					self.handle_propagated_event(&mut tui, event)?;
				}
				next_ed = result.requeued;
			}
			if self.should_suspend {
				self.should_suspend = false;
				self.event_system.stop().await?;
				tui.suspend()?;

				tui.clear()?;
				tui.resume()?;
				self.event_system.start()?;
			}
		}
		self.event_system.stop().await?;
		tui.exit()?;
		Ok(())
	}

	fn handle_propagated_event(&mut self, tui: &mut Terminal, event: Event<T>) -> eyre::Result<()> {
		match event {
			Event::Render(_) => {
				tui.try_draw(|frame| self.ecs.draw(frame).map_err(io::Error::other))?;
			}
			Event::Key(key_event) => {
				self.handle_special_keys(key_event);
			}
			Event::App(app_event) if app_event.is_quit() => self.should_quit = true,
			_ => (),
		}
		Ok(())
	}

	fn handle_special_keys(&mut self, key_event: KeyEvent) {
		match key_event.code {
			KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => {
				self.should_quit = true;
			}
			KeyCode::Char('z') if key_event.modifiers == KeyModifiers::CONTROL => {
				self.should_suspend = true;
			}
			_ => (),
		}
	}
}
