use crate::CONFIG;
// use serde_json;
use toml;

use crate::{DEFAULT_CMD, WILD_CMD, HELP_CMD, Commands, CommandFn};
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

pub fn help(_: Vec<String>) { }

pub fn exec(_: Vec<String>) {
	println!("{}", toml::to_string_pretty(&CONFIG.clone()).unwrap());
	// println!("{}", serde_json::to_string_pretty(&CONFIG.clone()).unwrap());
}