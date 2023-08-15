use crate::common::{
	this_command,
	polybar,
};

use crate::{DEFAULT_CMD, HELP_CMD, Commands, CommandFn};
use std::collections::HashMap;

lazy_static! {
	pub static ref CMD: String = "poke-poly".to_string();

	pub static ref SUBCMDS: Commands = {
		let mut cmds = HashMap::new();
		cmds.insert(DEFAULT_CMD, exec as CommandFn);
		cmds.insert(HELP_CMD, help as CommandFn);
		cmds
	};
}

pub fn help(_: Vec<String>) {
	println!("{} {}", this_command(), CMD.as_str());
	println!("    Pokes polybar to update");
}

pub fn exec(_: Vec<String>) {
	polybar::update();
}
