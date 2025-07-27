use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Action {
	#[serde(skip)]
	Tick,
	#[serde(skip)]
	Render,
	#[serde(skip)]
	Resize(u16, u16),
	#[serde(skip)]
	ClearScreen,
	#[serde(skip)]
	Error(String),
	#[serde(skip)]
	Unsuspend,

	Quit,
	Suspend,
	Help,
	Marco,
	Ping,
}
