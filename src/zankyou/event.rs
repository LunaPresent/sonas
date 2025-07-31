use color_eyre::eyre::OptionExt;
use crossterm::event::Event as CrosstermEvent;
use futures::{FutureExt, StreamExt};
use std::time::Duration;
use tokio::sync::mpsc;

const TICK_FPS: f64 = 30.0;

#[derive(Debug, Clone)]
pub struct EventDispatch {
	dispatch: Dispatch,
	event: Event,
}

impl EventDispatch {
	pub fn new(dispatch: Dispatch, event: Event) -> Self {
		Self { dispatch, event }
	}

	pub fn dispatch(&self) -> Dispatch {
		self.dispatch
	}

	pub fn event(&self) -> &Event {
		&self.event
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dispatch {
	System,
	Input,
	Broadcast,
}

#[derive(Debug, Clone)]
pub enum Event {
	Tick,
	Crossterm(CrosstermEvent),
	AppEvent(AppEvent),
}

#[derive(Debug, Clone)]
pub enum AppEvent {
	Quit,
	Increment,
	Decrement,
}

#[derive(Debug)]
pub struct EventQueue {
	sender: mpsc::UnboundedSender<EventDispatch>,
	receiver: mpsc::UnboundedReceiver<EventDispatch>,
}

impl EventQueue {
	/// Constructs a new instance of [`EventQueue`] and spawns a new thread to handle events.
	pub fn new() -> Self {
		let (sender, receiver) = mpsc::unbounded_channel();
		let actor = EventTask::new(sender.clone());
		tokio::spawn(async { actor.run().await });
		Self { sender, receiver }
	}

	/// Receives an event from the sender.
	///
	/// This function blocks until an event is received.
	///
	/// # Errors
	///
	/// This function returns an error if the sender channel is disconnected. This can happen if an
	/// error occurs in the event thread. In practice, this should not happen unless there is a
	/// problem with the underlying terminal.
	pub async fn next(&mut self) -> color_eyre::Result<EventDispatch> {
		self.receiver
			.recv()
			.await
			.ok_or_eyre("Failed to receive event")
	}

	/// Queue an app event to be sent to the event receiver.
	///
	/// This is useful for sending events to the event handler which will be processed by the next
	/// iteration of the application's event loop.
	pub fn send(&mut self, app_event: AppEvent) {
		let _ = self.sender.send(EventDispatch::new(
			Dispatch::Input,
			Event::AppEvent(app_event),
		));
	}

	/// Queue an app event to be sent to the event receiver.
	///
	/// This is useful for sending events to the event handler which will be processed by the next
	/// iteration of the application's event loop.
	pub fn broadcast(&mut self, app_event: AppEvent) {
		let _ = self.sender.send(EventDispatch::new(
			Dispatch::Broadcast,
			Event::AppEvent(app_event),
		));
	}
}

struct EventTask {
	sender: mpsc::UnboundedSender<EventDispatch>,
}

impl EventTask {
	/// Constructs a new instance of [`EventTask`].
	fn new(sender: mpsc::UnboundedSender<EventDispatch>) -> Self {
		Self { sender }
	}

	/// Runs the event thread.
	///
	/// This function emits tick events at a fixed rate and polls for crossterm events in between.
	async fn run(self) -> color_eyre::Result<()> {
		let tick_rate = Duration::from_secs_f64(1.0 / TICK_FPS);
		let mut reader = crossterm::event::EventStream::new();
		let mut tick = tokio::time::interval(tick_rate);
		loop {
			let tick_delay = tick.tick();
			let crossterm_event = reader.next().fuse();
			tokio::select! {
				_ = self.sender.closed() => {
					break;
				}
				_ = tick_delay => {
					self.send(Event::Tick);
				}
				Some(Ok(evt)) = crossterm_event => {
					self.send(Event::Crossterm(evt));
				}
			};
		}
		Ok(())
	}

	/// Sends an event to the receiver.
	fn send(&self, event: Event) {
		let _ = self
			.sender
			.send(EventDispatch::new(Dispatch::System, event));
	}
}
