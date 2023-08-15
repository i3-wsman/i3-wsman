extern crate i3_ipc;

use i3_ipc::{Connect, I3};

use crate::common::{
	Direction,
	this_command,
	polybar,
	workspaces,
	outputs,
	neighbor,
	constraint,
	constraint::Constraint,
};

use crate::{
	DEFAULT_CMD, HELP_CMD, WILD_CMD,
	Commands, CommandFn
};
use std::collections::HashMap;

lazy_static! {
	pub static ref CMD: String = "next".to_string();

	pub static ref SUBCMDS: Commands = {
		let mut cmds = HashMap::new();
		cmds.insert(DEFAULT_CMD, exec as CommandFn);
		cmds.insert(WILD_CMD, exec as CommandFn);
		cmds.insert(HELP_CMD, help as CommandFn);
		cmds
	};
}

pub fn help(_: Vec<String>) {
	println!("{} {} [create|loop] [...constraints]", this_command(), CMD.as_str());
	println!("    Focuses on the next workspace\n\r");
	println!("    {} next create [...constraints]\tOn last workspace, creates a new workspace", this_command());
	println!("    {} next loop [...constraints]  \tOn last workspace, loops back to the first", this_command());
	println!("");
	println!("    For constraints, run: {} get-workspaces help", this_command());
}

pub fn exec(mut args: Vec<String>) {
	let create = args.len() > 1 && args[0] == "create";

	if create || args.len() > 1 && args[0] == "loop" {
		args.remove(0);
	}

	let mut constraints = constraint::parse(args.to_owned());

	if args.len() == 0 {
		constraints.add(Constraint::Output);
		constraints.output = outputs::focused();
	}

	let focused_ws = workspaces::focused();

	let neighbor = neighbor::get(focused_ws, constraints.clone(), Direction::Right);

	if let Some(next) = neighbor {
		let mut i3 = I3::connect().unwrap();
		let cmd = format!("workspace {}", next.name);
		i3.run_command(cmd).ok();
	} else if create {
		crate::commands::adjacent::exec(vec!["right".to_owned()]);
	} else {
		let first_ws = workspaces::first(constraints);
		let mut i3 = I3::connect().unwrap();
		let cmd = format!("workspace {}", first_ws.name);
		i3.run_command(cmd).ok();
	}

	polybar::update();
}
