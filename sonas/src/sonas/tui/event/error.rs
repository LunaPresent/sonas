use std::fmt;

use thiserror::Error;
use tokio::{sync::mpsc::error::SendError, task::JoinError};

use crate::tui::event::EventDispatch;

#[derive(Error)]
pub enum EventError<T> {
	#[error("event channel disconnected")]
	Disconnected,
	#[error("task is already running")]
	AlreadyRunning,
	#[error("task is already stopped")]
	AlreadyStopped,
	#[error("failed to join thread")]
	JoinError(#[from] JoinError),
	#[error("failed to send event")]
	SendError(#[from] SendError<EventDispatch<T>>),
}

impl<T> fmt::Debug for EventError<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(&self, f)
	}
}
