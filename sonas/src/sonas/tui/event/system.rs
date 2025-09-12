use crossterm::event::Event as CrosstermEvent;
use futures::{FutureExt, StreamExt};
use std::time::Duration;
use tokio::{
	sync::mpsc::{self, error::SendError},
	task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

use super::{Dispatch, Event, EventDispatch, EventError};

const TPS: f64 = 8.0;
const FPS: f64 = 30.0;

#[derive(Debug)]
pub struct EventSystem<T> {
	receiver: mpsc::UnboundedReceiver<EventDispatch<T>>,
	sender: mpsc::UnboundedSender<EventDispatch<T>>,
	cancellation_token: CancellationToken,
	join_handle: Option<JoinHandle<Result<(), SendError<EventDispatch<T>>>>>,
}

impl<T> EventSystem<T>
where
	T: Send + 'static,
{
	/// Creates a new `EventSystem`
	pub fn new() -> Self {
		let (sender, receiver) = mpsc::unbounded_channel();
		Self {
			receiver,
			sender,
			cancellation_token: CancellationToken::new(),
			join_handle: None,
		}
	}

	/// Pops an event off the queue
	///
	/// This function blocks until an event is received.
	///
	/// # Errors
	///
	/// This function returns an error if the sender channel is disconnected. This can happen if an
	/// error occurs in the event thread. In practice, this should not happen unless there is a
	/// problem with the underlying terminal.
	pub async fn next(&mut self) -> Result<EventDispatch<T>, EventError<T>> {
		self.receiver.recv().await.ok_or(EventError::Disconnected)
	}

	pub fn sender(&self) -> mpsc::UnboundedSender<EventDispatch<T>> {
		self.sender.clone()
	}

	pub fn start(&mut self) -> Result<(), EventError<T>> {
		if self.join_handle.is_some() {
			return Err(EventError::AlreadyRunning);
		}
		let sender = self.sender.clone();
		let cancellation_token = self.cancellation_token.clone();
		self.join_handle = Some(tokio::spawn(async {
			Self::run(sender, cancellation_token).await
		}));
		Ok(())
	}

	pub async fn stop(&mut self) -> Result<(), EventError<T>> {
		let join_handle = self.join_handle.take().ok_or(EventError::AlreadyStopped)?;
		self.cancellation_token.cancel();
		join_handle.await??;
		self.cancellation_token = CancellationToken::new();
		Ok(())
	}

	async fn run(
		sender: mpsc::UnboundedSender<EventDispatch<T>>,
		cancellation_token: CancellationToken,
	) -> Result<(), SendError<EventDispatch<T>>> {
		let tick_rate = Duration::from_secs_f64(1.0 / TPS);
		let frame_rate = Duration::from_secs_f64(1.0 / FPS);
		let mut crossterm_events = crossterm::event::EventStream::new();
		let mut tick_interval = tokio::time::interval(tick_rate);
		let mut render_interval = tokio::time::interval(frame_rate);
		loop {
			tokio::select! {
				_ = sender.closed() => {
					break;
				}
				_ = cancellation_token.cancelled() => {
					break;
				}
				_ = tick_interval.tick() => {
					sender.send(EventDispatch::new(Dispatch::Broadcast, Event::Tick(tick_rate)))?;
				}
				_ = render_interval.tick() => {
					sender.send(EventDispatch::new(
						Dispatch::Broadcast,
						Event::Render(frame_rate),
					))?;
				}
				Some(Ok(evt)) = crossterm_events.next().fuse() => {
					Self::handle_crossterm_event(&sender, evt)?;
				}
			};
		}
		Ok(())
	}

	fn handle_crossterm_event(
		sender: &mpsc::UnboundedSender<EventDispatch<T>>,
		evt: CrosstermEvent,
	) -> Result<(), SendError<EventDispatch<T>>> {
		match evt {
			CrosstermEvent::FocusGained => {
				sender.send(EventDispatch::new(Dispatch::Broadcast, Event::FocusGained))
			}
			CrosstermEvent::FocusLost => {
				sender.send(EventDispatch::new(Dispatch::Broadcast, Event::FocusLost))
			}
			CrosstermEvent::Key(key_event) => {
				sender.send(EventDispatch::new(Dispatch::Input, Event::Key(key_event)))
			}
			CrosstermEvent::Mouse(mouse_event) => sender.send(EventDispatch::new(
				Dispatch::Cursor {
					x: mouse_event.column,
					y: mouse_event.row,
				},
				Event::Mouse(mouse_event),
			)),
			CrosstermEvent::Paste(s) => {
				sender.send(EventDispatch::new(Dispatch::Input, Event::Paste(s)))
			}
			CrosstermEvent::Resize(width, height) => sender.send(EventDispatch::new(
				Dispatch::Broadcast,
				Event::Resize { width, height },
			)),
		}
	}
}
