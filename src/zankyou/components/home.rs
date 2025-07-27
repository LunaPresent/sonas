use std::io::{BufRead, BufReader, Write};

use color_eyre::Result;
use interprocess::local_socket::{GenericNamespaced, Stream, ToNsName, traits::Stream as _};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, config::Config};

#[derive(Default)]
pub struct Home<'a> {
	command_tx: Option<UnboundedSender<Action>>,
	config: Config,
	text: Paragraph<'a>,
}

impl<'a> Home<'a> {
	pub fn new() -> Self {
		Self {
			command_tx: None,
			config: Default::default(),
			text: Paragraph::new("hellow orld"),
		}
	}

	fn request(r: &[u8]) -> Result<String> {
		let name = "zankyou-server.sock".to_ns_name::<GenericNamespaced>()?;
		let mut buffer = String::with_capacity(128);
		let mut conn = BufReader::new(Stream::connect(name)?);
		conn.get_mut().write_all(r)?;
		conn.read_line(&mut buffer)?;

		Ok(buffer)
	}
}

impl<'a> Component for Home<'a> {
	fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
		self.command_tx = Some(tx);
		Ok(())
	}

	fn register_config_handler(&mut self, config: Config) -> Result<()> {
		self.config = config;
		Ok(())
	}

	fn update(&mut self, action: Action) -> Result<Option<Action>> {
		match action {
			Action::Marco => {
				let reply = Self::request(b"marco\n")?;
				self.text = Paragraph::new(reply);
			}
			Action::Ping => {
				let reply = Self::request(b"ping\n")?;
				self.text = Paragraph::new(reply);
			}
			_ => {}
		}
		Ok(None)
	}

	fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
		frame.render_widget(&self.text, area);
		Ok(())
	}
}
