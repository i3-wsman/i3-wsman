use std::collections::HashMap;

use crate::{
	common::{constraint, this_command, Direction},
	config::global::NavigationBehavior,
	i3::{self, get_filtered_criteria, get_focused_workspace, get_matching_workspaces},
	polybar,
};

use crate::{CommandFn, Commands, DEFAULT_CMD, HELP_CMD, WILD_CMD};

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
	println!(
		"{} {} [{}|{}|{}] [...constraints]",
		this_command(),
		CMD.as_str(),
		NavigationBehavior::Create.to_string(),
		NavigationBehavior::Loop.to_string(),
		NavigationBehavior::Stop.to_string(),
	);
	println!("    Focuses on the next workspace\n\r");
	println!(
		"    {} {} {} [...constraints]",
		this_command(),
		CMD.as_str(),
		NavigationBehavior::Create.to_string(),
	);
	println!("        On last workspace, create a new workspace\n\r");
	println!(
		"    {} {} {} [...constraints]  \t",
		this_command(),
		CMD.as_str(),
		NavigationBehavior::Loop.to_string(),
	);
	println!("        On last workspace, loop back to the first workspace\n\r");
	println!(
		"    {} {} {} [...constraints]  \t",
		this_command(),
		CMD.as_str(),
		NavigationBehavior::Stop.to_string(),
	);
	println!("        On last workspace, do nothing\n\r");
	println!(
		"    For constraints, run: {} help constraints\n\r",
		this_command()
	);
}

pub fn exec(mut args: Vec<String>) {
	let behavior = NavigationBehavior::from_argv(&mut args).unwrap();

	let criteria = if args.len() > 0 {
		constraint::from_vec(args.to_owned())
	} else {
		get_filtered_criteria(true)
	};

	let focused = get_focused_workspace();

	let neighbor = focused.get_closest_neighbor(Some(criteria.clone()), Some(Direction::Right));

	if let Some(next) = neighbor {
		i3::run_command(format!("workspace {}", next.full_name()));
	} else {
		match behavior {
			NavigationBehavior::Create => {
				crate::commands::actions::adjacent::exec(vec![Direction::Right.to_string()]);
			}
			NavigationBehavior::Loop => {
				let workspaces = get_matching_workspaces(criteria);
				let first_ws = workspaces.first().unwrap();
				i3::run_command(format!("workspace {}", first_ws.full_name()));
			}
			NavigationBehavior::Stop => {}
		}
	}

	// workspaces::maintenance();
	polybar::update();
}
