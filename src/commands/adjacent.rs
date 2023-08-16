extern crate i3_ipc;

use i3_ipc::{Connect, I3};

use crate::common::{
	this_command,
	groups,
	polybar,
	workspaces,
	moves,
	outputs,
	name,
};

use crate::{
	DEFAULT_CMD, HELP_CMD, WILD_CMD,
	Commands, CommandFn
};
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
	println!("{} {} adjacent <right|left>", this_command(), CMD.as_str());
	println!("    Creates a new workspace next to the current workspace\n\r");
	println!("    {} adjacent right\t- Creates a workspace to the right", this_command());
	println!("    {} adjacent left \t- Creates a workspace to the left", this_command());
}

pub fn exec(args: Vec<String>) {
	let focused_ws = workspaces::focused();
	let focused_ws_num = focused_ws.num;
	let focused_group = name::group(&focused_ws.name);

	let active_groups = groups::active(outputs::focused());

	let group_name = if active_groups.len() == 1 {
		active_groups[0].to_owned()
	} else {
		focused_group
	};

	let new_ws_num = if args[0] == "left" {
		moves::right(focused_ws);
		focused_ws_num
	} else {
		let nn = focused_ws_num + 1;
		let ws_to_move = workspaces::by_num(nn);
		if let Some(moveit) = ws_to_move {
			moves::right(moveit);
		}
		nn
	};

	let new_ws_name = name::change_prefix(
		&group_name,
		new_ws_num
	);

	let mut i3 = I3::connect().unwrap();
	let cmd = format!("workspace {}", new_ws_name);
	i3.run_command(cmd).ok();

	polybar::update();
}
