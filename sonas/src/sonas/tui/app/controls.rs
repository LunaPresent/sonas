use color_eyre::eyre;
use tokio::sync::mpsc;

pub(crate) enum SignalType {
	Quit,
	Suspend,
}

#[derive(Debug, Clone)]
pub(crate) struct AppControls {
	stop: mpsc::UnboundedSender<()>,
	signal: mpsc::UnboundedSender<SignalType>,
}

#[derive(Debug)]
pub(crate) struct AppSignalReceiver {
	pub stop: mpsc::UnboundedReceiver<()>,
	pub signal: mpsc::UnboundedReceiver<SignalType>,
}

impl AppControls {
	pub fn new() -> (AppControls, AppSignalReceiver) {
		let stop = mpsc::unbounded_channel();
		let signal = mpsc::unbounded_channel();

		(
			AppControls {
				stop: stop.0,
				signal: signal.0,
			},
			AppSignalReceiver {
				stop: stop.1,
				signal: signal.1,
			},
		)
	}

	pub fn quit(&mut self) -> eyre::Result<()> {
		self.stop.send(())?;
		self.signal.send(SignalType::Quit)?;
		Ok(())
	}

	pub fn suspend(&mut self) -> eyre::Result<()> {
		self.stop.send(())?;
		self.signal.send(SignalType::Suspend)?;
		Ok(())
	}
}
