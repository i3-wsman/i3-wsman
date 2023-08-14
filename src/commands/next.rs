extern crate i3_ipc;

use i3_ipc::{Connect, I3};

use crate::common::{
	Direction,
	this_command,
	polybar_update,
	get_active_workspace,
	get_first_workspace,
	get_output,
	get_neighbor,
	get_constraints,
	constraint::Constraint,
};

pub fn exec(mut args: Vec<String>) {
	if args.contains(&"help".to_string()) {
		println!("{} next [create|loop] [...constraints]", this_command());
		println!("    Focuses on the next workspace\n\r");
		println!("    {} next create [...constraints]\tOn last workspace, creates a new workspace", this_command());
		println!("    {} next loop [...constraints]  \tOn last workspace, loops back to the first", this_command());
		println!("");
		println!("    For constraints, run: {} get-workspaces help", this_command());
		return;
	}

	let create = args.len() > 1 && args[0] == "create";

	if create || args.len() > 1 && args[0] == "loop" {
		args.remove(0);
	}

	let mut constraints = get_constraints(args.to_owned());

	if args.len() == 0 {
		constraints.add(Constraint::Output);
		constraints.output = get_output();
	}

	let active_ws = get_active_workspace();

	let neighbor = get_neighbor(active_ws, constraints.clone(), Direction::Right);

	if let Some(next) = neighbor {
		let mut i3 = I3::connect().unwrap();
		let cmd = format!("workspace {}", next.name);
		i3.run_command(cmd).ok();
	} else if create {
		crate::commands::adjacent::exec(vec!["right".to_owned()]);
	} else {
		let first_ws = get_first_workspace(constraints);
		let mut i3 = I3::connect().unwrap();
		let cmd = format!("workspace {}", first_ws.name);
		i3.run_command(cmd).ok();
	}

	polybar_update();
}
