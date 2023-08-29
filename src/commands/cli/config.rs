use crate::CONFIG;
use toml;

use crate::{CommandFn, Commands, DEFAULT_CMD, HELP_CMD, WILD_CMD};
use std::collections::HashMap;

lazy_static! {
	pub static ref CMD: String = "config".to_string();
	pub static ref SUBCMDS: Commands = {
		let mut cmds = HashMap::new();
		cmds.insert(DEFAULT_CMD, exec as CommandFn);
		cmds.insert(WILD_CMD, exec as CommandFn);
		cmds.insert(HELP_CMD, help as CommandFn);
		cmds
	};
}

pub fn help(_: Vec<String>) {}

pub fn exec(_: Vec<String>) {
	println!("{}", toml::to_string_pretty(&CONFIG.clone()).unwrap());
}
