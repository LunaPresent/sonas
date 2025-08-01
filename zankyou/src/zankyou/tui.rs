use std::{
	io::{Stdout, stdout},
	ops::{Deref, DerefMut},
};

use color_eyre::Result;
use crossterm::{
	cursor,
	event::{DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture},
	terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend as Backend;

pub struct Tui {
	pub terminal: ratatui::Terminal<Backend<Stdout>>,
}

impl Tui {
	pub fn new() -> Result<Self> {
		Ok(Self {
			terminal: ratatui::Terminal::new(Backend::new(stdout()))?,
		})
	}

	pub fn enter(&mut self) -> Result<()> {
		crossterm::terminal::enable_raw_mode()?;
		crossterm::execute!(stdout(), EnterAlternateScreen, cursor::Hide)?;
		crossterm::execute!(stdout(), EnableMouseCapture)?;
		crossterm::execute!(stdout(), EnableBracketedPaste)?;
		Ok(())
	}

	pub fn exit(&mut self) -> Result<()> {
		if crossterm::terminal::is_raw_mode_enabled()? {
			self.flush()?;
			crossterm::execute!(stdout(), DisableBracketedPaste)?;
			crossterm::execute!(stdout(), DisableMouseCapture)?;
			crossterm::execute!(stdout(), LeaveAlternateScreen, cursor::Show)?;
			crossterm::terminal::disable_raw_mode()?;
		}
		Ok(())
	}

	pub fn suspend(&mut self) -> Result<()> {
		self.exit()?;
		#[cfg(not(windows))]
		signal_hook::low_level::raise(signal_hook::consts::signal::SIGTSTP)?;
		Ok(())
	}

	pub fn resume(&mut self) -> Result<()> {
		self.enter()?;
		Ok(())
	}
}

impl Deref for Tui {
	type Target = ratatui::Terminal<Backend<Stdout>>;

	fn deref(&self) -> &Self::Target {
		&self.terminal
	}
}

impl DerefMut for Tui {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.terminal
	}
}

impl Drop for Tui {
	fn drop(&mut self) {
		self.exit().unwrap();
	}
}
