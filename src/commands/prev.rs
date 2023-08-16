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

const ON_LAST_CREATE: &str = "create";
const ON_LAST_LOOP: &str = "loop";
const ON_LAST_STOP: &str = "stop";

lazy_static! {
	pub static ref CMD: String = "prev".to_string();

	pub static ref SUBCMDS: Commands = {
		let mut cmds = HashMap::new();
		cmds.insert(DEFAULT_CMD, exec as CommandFn);
		cmds.insert(WILD_CMD, exec as CommandFn);
		cmds.insert(HELP_CMD, help as CommandFn);
		cmds
	};
}

pub fn help(_: Vec<String>) {
	println!("{} {} [{}|{}|{}] [...constraints]", this_command(), CMD.as_str(), ON_LAST_CREATE, ON_LAST_LOOP, ON_LAST_STOP);
	println!("    Focuses on the previous workspace\n\r");
	println!("    {} next {} [...constraints]\tOn first workspace, creates a new workspace", this_command(), ON_LAST_CREATE);
	println!("    {} next {} [...constraints]  \tOn first workspace, loops back to the first", this_command(), ON_LAST_LOOP);
	println!("    {} next {} [...constraints]  \tOn first workspace, stops", this_command(), ON_LAST_STOP);
	println!("");
	println!("    For constraints, run: {} get-workspaces help", this_command());
}

pub fn exec(mut args: Vec<String>) {
	let last_action: String = match args[0].as_str() {
		ON_LAST_CREATE => args.remove(0),
		ON_LAST_LOOP => args.remove(0),
		ON_LAST_STOP => args.remove(0),
		_ => "".to_string(),
	};

	let mut constraints = constraint::parse(args.to_owned());

	if args.len() == 0 {
		constraints.add(Constraint::Output);
		constraints.output = outputs::focused();
		constraints.add(Constraint::Group);
		constraints.add(Constraint::NoGroup);
	}

	let focused_ws = workspaces::focused();

	let neighbor = neighbor::get(focused_ws, constraints.clone(), Direction::Left);

	if let Some(next) = neighbor {
		let mut i3 = I3::connect().unwrap();
		let cmd = format!("workspace {}", next.name);
		i3.run_command(cmd).ok();
	} else {
		match last_action.as_str() {
			ON_LAST_CREATE => {
				crate::commands::adjacent::exec(vec!["left".to_owned()]);
			}
			ON_LAST_LOOP => {
				let first_ws = workspaces::last(constraints);
				let mut i3 = I3::connect().unwrap();
				let cmd = format!("workspace {}", first_ws.name);
				i3.run_command(cmd).ok();
			}
			ON_LAST_STOP => {}
			_ => {}
		}
	}

	polybar::update();
}
