extern crate i3_ipc;

use crate::{
	common::{this_command, Direction},
	i3::{self, get_current_output, get_focused_workspace},
	polybar,
};

use crate::{CommandFn, Commands, DEFAULT_CMD, HELP_CMD, WILD_CMD};
use std::collections::HashMap;

lazy_static! {
	pub static ref CMD: String = "adjacent".to_string();
	pub static ref SUBCMDS: Commands = {
		let mut cmds = HashMap::new();
		cmds.insert(DEFAULT_CMD, exec as CommandFn);
		cmds.insert(WILD_CMD, exec as CommandFn);
		cmds.insert(HELP_CMD, help as CommandFn);
		cmds
	};
}

pub fn help(_: Vec<String>) {
	println!("{} {} <right|left>", this_command(), CMD.as_str());
	println!("    Creates a new workspace next to the current workspace\n\r");
	println!("    {} {} right", this_command(), CMD.as_str());
	println!("        Creates a workspace to the right\n\r");
	println!("    {} {} left", this_command(), CMD.as_str());
	println!("        Creates a workspace to the left\n\r");
}

pub fn exec(args: Vec<String>) {
	let output = get_current_output();
	let focused = get_focused_workspace();

	let active_groups = output.active_groups();
	let focused_group = focused.group();

	let new_group = if focused_group == "" {
		focused_group
	} else if active_groups.len() == 1 {
		active_groups[0].to_owned()
	} else {
		focused_group
	};

	let dir = args[0].parse::<Direction>().unwrap();

	let ws_to_move = match dir {
		Direction::Left => Some(focused.clone()),
		Direction::Right => focused.get_closest_neighbor(None, Some(Direction::Right)),
	};

	if let Some(mut ws) = ws_to_move {
		ws.scoot();
	}

	let new_pos = match dir {
		Direction::Left => focused.num(),
		Direction::Right => focused.num() + 1,
	};

	i3::run_command(format!("workspace {}:{}", new_pos, new_group));

	polybar::update();
}
