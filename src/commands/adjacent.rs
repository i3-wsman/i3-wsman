extern crate i3_ipc;

use i3_ipc::{Connect, I3};

use crate::common::{
	groups, moves, name, neighbor, outputs, polybar, this_command, workspaces, Direction,
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
	let dir = if args[0] == "left" {
		Direction::Left
	} else {
		Direction::Right
	};

	let focused_ws = workspaces::focused();

	let focused_group = name::group(&focused_ws.name);
	let active_groups = groups::active(outputs::focused());
	let name_base = if focused_group == "" {
		focused_group
	} else {
		format!(
			"x:{}",
			if active_groups.len() == 1 {
				active_groups[0].to_owned()
			} else {
				focused_group
			}
		)
	};

	let focused_ws_num = focused_ws.num;

	let ws_to_move = if dir == Direction::Left {
		Some(focused_ws)
	} else {
		neighbor::closest(focused_ws, Direction::Right)
	};

	let new_pos = match ws_to_move {
		Some(ws) => moves::scoot(ws).num - 1,
		None => focused_ws_num + 1,
	};

	let new_name = name::change_prefix(&name_base, new_pos);

	let mut i3 = I3::connect().unwrap();
	let cmd = format!("workspace {}", new_name);
	i3.run_command(cmd).ok();

	polybar::update();
}
