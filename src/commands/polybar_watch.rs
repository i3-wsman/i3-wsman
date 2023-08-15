use i3_ipc::{
	event::{Event, Subscribe},
	I3Stream,
};

use crate::common::{
	this_command,
	polybar,
};

use crate::{DEFAULT_CMD, HELP_CMD, Commands, CommandFn};
use std::collections::HashMap;

lazy_static! {
	pub static ref CMD: String = "polybar-watch".to_string();

	pub static ref SUBCMDS: Commands = {
		let mut cmds = HashMap::new();
		cmds.insert(DEFAULT_CMD, exec as CommandFn);
		cmds.insert(HELP_CMD, help as CommandFn);
		cmds
	};
}

pub fn help(_: Vec<String>) {
	println!("{} {}", this_command(), CMD.as_str());
	println!("  Listens for i3 events and updates polybar module.");
}

pub fn exec(_: Vec<String>) {
	let mut i3 = I3Stream::conn_sub(&[Subscribe::Window, Subscribe::Workspace]).unwrap();
	for e in i3.listen() {
		match e.unwrap() {
			Event::Workspace(_) => polybar::update(),
			Event::Window(_) => polybar::update(),
			Event::Output(_) => polybar::update(),
			Event::Mode(_) => polybar::update(),
			Event::BarConfig(_) => polybar::update(),
			_ => {}
		}
	}
}
