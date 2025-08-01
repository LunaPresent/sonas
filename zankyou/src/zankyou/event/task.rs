use crossterm::event::Event as CrosstermEvent;
use futures::{FutureExt, StreamExt};
use std::time::Duration;
use tokio::sync::mpsc;

use super::{Dispatch, Event, EventDispatch, EventSender};

const TICK_FPS: f64 = 30.0;

pub struct EventTask<E> {
	sender: mpsc::UnboundedSender<EventDispatch<E>>,
}

impl<E> EventTask<E> {
	/// Constructs a new instance of [`EventTask`].
	pub fn new(sender: mpsc::UnboundedSender<EventDispatch<E>>) -> Self {
		Self { sender }
	}

	/// Runs the event thread.
	///
	/// This function emits tick events at a fixed rate and polls for crossterm events in between.
	pub async fn run(self) -> color_eyre::Result<()> {
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
					self.send(Dispatch::Broadcast, Event::Tick);
				}
				Some(Ok(evt)) = crossterm_event => {
					self.handle_crossterm_event(evt);
				}
			};
		}
		Ok(())
	}

	fn handle_crossterm_event(&self, evt: CrosstermEvent) {
		match evt {
			CrosstermEvent::FocusGained => {
				self.send(Dispatch::Broadcast, Event::FocusGained);
			}
			CrosstermEvent::FocusLost => {
				self.send(Dispatch::Broadcast, Event::FocusLost);
			}
			CrosstermEvent::Key(key_event) => {
				self.send(Dispatch::Input, Event::Key(key_event));
			}
			CrosstermEvent::Mouse(mouse_event) => {
				self.send(
					Dispatch::Cursor {
						x: mouse_event.column,
						y: mouse_event.row,
					},
					Event::Mouse(mouse_event),
				);
			}
			CrosstermEvent::Paste(s) => {
				self.send(Dispatch::Input, Event::Paste(s));
			}
			CrosstermEvent::Resize(width, height) => {
				self.send(Dispatch::Broadcast, Event::Resize { width, height });
			}
		}
	}
}

impl<E> EventSender<E> for EventTask<E> {
	fn sender(&self) -> &mpsc::UnboundedSender<EventDispatch<E>> {
		&self.sender
	}
}
