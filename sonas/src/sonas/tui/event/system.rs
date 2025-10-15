use crossterm::event::Event as CrosstermEvent;
use futures::{FutureExt, StreamExt};
use std::time::Duration;
use tokio::{
	sync::mpsc::{self, error::SendError},
	task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

use crate::tui::event::SystemEvent;

use super::{DispatchMethod, EventDispatch, EventError};

#[derive(Debug)]
pub struct EventSystem {
	receiver: mpsc::UnboundedReceiver<EventDispatch<SystemEvent>>,
	sender: mpsc::UnboundedSender<EventDispatch<SystemEvent>>,
	cancellation_token: CancellationToken,
	join_handle: Option<JoinHandle<Result<(), SendError<EventDispatch<SystemEvent>>>>>,
}

impl EventSystem {
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
	pub async fn next(&mut self) -> Result<EventDispatch<SystemEvent>, EventError> {
		self.receiver.recv().await.ok_or(EventError::Disconnected)
	}

	pub fn start(&mut self, tick_interval: Duration) -> Result<(), EventError> {
		if self.join_handle.is_some() {
			return Err(EventError::AlreadyRunning);
		}
		let sender = self.sender.clone();
		let cancellation_token = self.cancellation_token.clone();
		self.join_handle = Some(tokio::spawn(async move {
			Self::run(tick_interval, sender, cancellation_token).await
		}));
		Ok(())
	}

	pub async fn stop(&mut self) -> Result<(), EventError> {
		let join_handle = self.join_handle.take().ok_or(EventError::AlreadyStopped)?;
		self.cancellation_token.cancel();
		join_handle.await??;
		self.cancellation_token = CancellationToken::new();
		Ok(())
	}

	pub fn is_running(&self) -> bool {
		self.join_handle.is_some()
	}

	async fn run(
		tick_interval: Duration,
		sender: mpsc::UnboundedSender<EventDispatch<SystemEvent>>,
		cancellation_token: CancellationToken,
	) -> Result<(), SendError<EventDispatch<SystemEvent>>> {
		let mut crossterm_events = crossterm::event::EventStream::new();
		let mut tick_interval_event = tokio::time::interval(tick_interval);
		loop {
			tokio::select! {
				_ = sender.closed() => {
					break;
				}
				_ = cancellation_token.cancelled() => {
					break;
				}
				_ = tick_interval_event.tick() => {
					sender.send(EventDispatch::new(
						DispatchMethod::Broadcast,
						SystemEvent::Tick(tick_interval),
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
		sender: &mpsc::UnboundedSender<EventDispatch<SystemEvent>>,
		evt: CrosstermEvent,
	) -> Result<(), SendError<EventDispatch<SystemEvent>>> {
		match evt {
			CrosstermEvent::FocusGained => sender.send(EventDispatch::new(
				DispatchMethod::Broadcast,
				SystemEvent::FocusGained,
			)),
			CrosstermEvent::FocusLost => sender.send(EventDispatch::new(
				DispatchMethod::Broadcast,
				SystemEvent::FocusLost,
			)),
			CrosstermEvent::Key(key_event) => sender.send(EventDispatch::new(
				DispatchMethod::Input,
				SystemEvent::Key(key_event),
			)),
			CrosstermEvent::Mouse(mouse_event) => sender.send(EventDispatch::new(
				DispatchMethod::Cursor {
					x: mouse_event.column,
					y: mouse_event.row,
					kind: mouse_event.kind,
				},
				SystemEvent::Mouse(mouse_event),
			)),
			CrosstermEvent::Paste(s) => sender.send(EventDispatch::new(
				DispatchMethod::Input,
				SystemEvent::Paste(s),
			)),
			CrosstermEvent::Resize(width, height) => sender.send(EventDispatch::new(
				DispatchMethod::Broadcast,
				SystemEvent::Resize { width, height },
			)),
		}
	}
}
