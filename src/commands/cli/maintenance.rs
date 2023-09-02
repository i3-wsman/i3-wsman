use std::collections::HashMap;

use crate::{
	common::this_command, i3::workspace_maintenance, CommandFn, Commands, DEFAULT_CMD, HELP_CMD,
	WILD_CMD,
};

use super::get_workspaces;

lazy_static! {
	pub static ref CMD: String = "maintenance".to_string();
	pub static ref SUBCMDS: Commands = {
		let mut cmds = HashMap::new();
		cmds.insert(DEFAULT_CMD, exec as CommandFn);
		cmds.insert(WILD_CMD, exec as CommandFn);
		cmds.insert(HELP_CMD, help as CommandFn);
		cmds
	};
}

pub fn help(_: Vec<String>) {
	println!("{} {}", this_command(), CMD.as_str());
	println!("    Returns workspaces matching the criteria.\n\r");
	println!("    For criteria, run: {} help criteria", this_command());
}

pub fn exec(_: Vec<String>) {
	workspace_maintenance();
	get_workspaces::exec(vec![]);
}
