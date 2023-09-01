use crate::{
	common::this_command,
	i3::{self, Workspace},
	polybar,
};

use crate::{CommandFn, Commands, DEFAULT_CMD, HELP_CMD, WILD_CMD};
use std::collections::HashMap;

lazy_static! {
	pub static ref CMD: String = "rename".to_string();
	pub static ref SUBCMDS: Commands = {
		let mut cmds = HashMap::new();
		cmds.insert(DEFAULT_CMD, clear as CommandFn);
		cmds.insert(WILD_CMD, exec as CommandFn);
		cmds.insert(HELP_CMD, help as CommandFn);
		cmds
	};
}

pub fn help(_: Vec<String>) {
	println!(
		"{} {} <new-name> [<workspace-full-name>]",
		this_command(),
		CMD.as_str()
	);
	println!("    Changes the 'name' of the current workspace\n\r");
}

pub fn clear(_: Vec<String>) {
	exec(vec!["".to_owned()]);
}

pub fn exec(mut args: Vec<String>) {
	let group_name = args.remove(0);

	let possible_ws = if args.len() >= 1 {
		let ws_name = args.remove(0);
		Workspace::by_name(ws_name.as_str())
	} else {
		Some(i3::get_current_workspace())
	};

	if let Some(mut ws) = possible_ws {
		ws.set_name(group_name);
	} else {
		println!("No workspace named {}", args[0].clone());
	}

	polybar::update();
}
