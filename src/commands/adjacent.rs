extern crate i3_ipc;

use crate::{
	common::{polybar, this_command, Direction},
	i3::{self, get_current_output, get_focused_workspace},
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
	eprintln!("adjacent::exec(): Getting current output");
	let output = get_current_output();
	eprintln!("adjacent::exec(): Getting focused workspace");
	let focused = get_focused_workspace();

	eprintln!("adjacent::exec(): Getting active groups");
	let active_groups = output.active_groups();
	eprintln!("\t\tactive_groups: {:?}", active_groups);
	eprintln!("adjacent::exec(): Getting group");
	let focused_group = focused.group();
	eprintln!("\t\tfocused_group: {}", focused_group);

	eprintln!("adjacent::exec(): Determining new_group");
	let new_group = if focused_group == "" {
		focused_group
	} else if active_groups.len() == 1 {
		active_groups[0].to_owned()
	} else {
		focused_group
	};
	eprintln!("\t\tnew_group: {:?}", new_group);

	eprintln!("adjacent::exec(): Parsing direction");
	let dir = args[0].parse::<Direction>().unwrap();
	eprintln!("\t\tdir: {:?}", dir);

	eprintln!("adjacent::exec(): Determining which workspace to move");
	let ws_to_move = match dir {
		Direction::Left => Some(focused.clone()),
		Direction::Right => focused.get_closest_neighbor(None, Some(Direction::Right)),
	};

	eprintln!("adjacent::exec(): Scooting, if necessary");
	if let Some(mut ws) = ws_to_move {
		eprintln!("adjacent::exec(): Scooting, as it turns out, is necessary");
		eprintln!("\t\tws_to_move: {:?}", ws.full_name());
		ws.scoot();
	}

	eprintln!("adjacent::exec(): Determining our new position");
	let new_pos = match dir {
		Direction::Left => focused.num(),
		Direction::Right => focused.num() + 1,
	};
	eprintln!("\t\tnew_pos: {:?}", new_pos);

	eprintln!("adjacent::exec(): Creating new workspace");
	eprintln!("\t\ti3-msg workspace {}:{}", new_pos, new_group);
	i3::run_command(format!("workspace {}:{}", new_pos, new_group));

	polybar::update();
}
