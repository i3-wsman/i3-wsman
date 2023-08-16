extern crate i3_ipc;

use i3_ipc::{Connect, I3};

use crate::common::{
	constraint::{Constraint, Constraints},
	this_command,
	Direction,
	polybar,
	neighbor,
	workspaces,
	outputs,
	name,
};

use crate::{
	DEFAULT_CMD, HELP_CMD, WILD_CMD,
	Commands, CommandFn
};
use std::collections::HashMap;

lazy_static! {
	pub static ref CMD: String = "reorder".to_string();

	pub static ref SUBCMDS: Commands = {
		let mut cmds = HashMap::new();
		cmds.insert(DEFAULT_CMD, exec as CommandFn);
		cmds.insert(WILD_CMD, exec as CommandFn);
		cmds.insert(HELP_CMD, help as CommandFn);
		cmds
	};
}

pub fn help(_: Vec<String>) {
	println!("{} {} reorder <right|left>", this_command(), CMD.as_str());
	println!("    Reorders the current workspaces to the left or right\n\r");
	println!("    {} {} right\t- Adjusts the order of the workspace to the right", this_command(), CMD.as_str());
	println!("    {} {} left \t- Adjusts the order of the workspace to the left", this_command(), CMD.as_str());
}

pub fn exec(args: Vec<String>) {
	let focused_ws = workspaces::focused();
	let focused_ws_num = focused_ws.num;
	let focused_ws_name = focused_ws.name.to_owned();

	let dir = if args[0] == "left" {
		Direction::Left
	} else {
		Direction::Right
	};

	let mut constraints = Constraints::new();
	constraints.add(Constraint::Output);
	constraints.output = outputs::focused();
	constraints.add(Constraint::Group);
	constraints.add(Constraint::NoGroup);

	if let Some(neighbor_ws) = neighbor::get(focused_ws, constraints, dir) {
		let neighbor_ws_num = neighbor_ws.num;
		let neighbor_ws_name = neighbor_ws.name;

		let mut i3 = I3::connect().unwrap();

		let new_fs_name = name::change_prefix(&focused_ws_name, neighbor_ws_num);
		let new_nr_name = name::change_prefix(&neighbor_ws_name, focused_ws_num);

		i3.run_command(format!("rename workspace {} to reorderinprogress:{}", focused_ws_name, focused_ws_name)).ok();
		i3.run_command(format!("rename workspace {} to {}", neighbor_ws_name, new_nr_name)).ok();
		i3.run_command(format!("rename workspace reorderinprogress:{} to {}", focused_ws_name, new_fs_name)).ok();
	}

	polybar::update();
}
