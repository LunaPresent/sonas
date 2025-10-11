mod controls;
mod entity_builder;

pub(crate) use controls::AppControls;
pub use entity_builder::EntityBuilder;

use std::{io, time::Duration};

use color_eyre::eyre;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::{
	ecs::ComponentSystem,
	event::{AppEvent, Event, EventSystem},
	terminal::Terminal,
};
use crate::tui::event::EventDispatch;
use controls::{AppSignalReceiver, SignalType};

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
	controls: AppControls,
	signal_receiver: AppSignalReceiver,
	event_system: EventSystem<T>,
	ecs: ComponentSystem<T>,
	tick_interval: Duration,
	frame_interval: Duration,
}

impl<T> App<T>
where
	T: AppEvent + Send + Sync + 'static,
{
	/// Creates a new `App`
	pub fn new() -> Self {
		let (controls, signal_receiver) = AppControls::new();
		let event_system = EventSystem::new();
		let ecs = ComponentSystem::new(event_system.sender());
		Self {
			controls,
			signal_receiver,
			event_system,
			ecs,
			tick_interval: Duration::from_millis(100),
			frame_interval: Duration::from_nanos(1),
		}
	}

	pub fn with_entity(
		mut self,
		f: impl FnOnce(EntityBuilder<T>) -> eyre::Result<EntityBuilder<T>>,
	) -> eyre::Result<Self> {
		Ok((f)(EntityBuilder::new(self.ecs.add_entity(), self))?.app())
	}

	pub fn with_tick_interval(mut self, interval: Duration) -> Self {
		self.tick_interval = interval.max(Duration::from_millis(1));
		self
	}

	pub fn with_frame_interval(mut self, interval: Duration) -> Self {
		self.frame_interval = interval.max(Duration::from_nanos(1));
		self
	}

	/// Runs the `App` until completion
	pub async fn run(mut self) -> eyre::Result<()> {
		let mut tui = Terminal::new()?;
		tui.enter()?;
		self.ecs.init()?;
		self.event_system.start(self.tick_interval)?;
		let mut frame_interval = tokio::time::interval(self.frame_interval);
		frame_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
		loop {
			tokio::select! {
				biased;
				_ = self.signal_receiver.stop.recv() => {
					self.event_system.stop().await?;
				}
				ed = self.event_system.next() => {
					self.dispatch_event(ed?)?;
				}
				Some(signal) = self.signal_receiver.signal.recv() => {
					match signal {
						SignalType::Quit => break,
						SignalType::Suspend => self.suspend(&mut tui).await?,
					}
				}
				_ = frame_interval.tick() => {
					tui.try_draw(|frame| self.ecs.draw(frame).map_err(io::Error::other))?;
				}
			}
		}
		tui.exit()?;
		Ok(())
	}

	async fn suspend(&mut self, tui: &mut Terminal) -> eyre::Result<()> {
		self.event_system.stop().await?;
		tui.suspend()?;

		tui.clear()?;
		tui.resume()?;
		self.event_system.start(self.tick_interval)?;
		Ok(())
	}

	fn dispatch_event(&mut self, ed: EventDispatch<T>) -> eyre::Result<()> {
		let mut next_ed = Some(ed);
		while let Some(ed) = next_ed {
			let result = self.ecs.handle_event(ed)?;
			if let Some(event) = result.propagated {
				self.handle_propagated_event(event)?;
			}
			next_ed = result.requeued;
		}
		Ok(())
	}

	fn handle_propagated_event(&mut self, event: Event<T>) -> eyre::Result<()> {
		match event {
			Event::Key(key_event) => {
				self.handle_special_keys(key_event)?;
			}
			Event::App(app_event) if app_event.is_quit() => self.controls.quit()?,
			_ => (),
		}
		Ok(())
	}

	fn handle_special_keys(&mut self, key_event: KeyEvent) -> eyre::Result<()> {
		match key_event.code {
			KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => {
				self.controls.quit()?
			}
			KeyCode::Char('z') if key_event.modifiers == KeyModifiers::CONTROL => {
				self.controls.suspend()?
			}
			_ => (),
		}
		Ok(())
	}
}
