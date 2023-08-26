use crate::common::{
	constraint,
	constraint::Constraint,
	groups,
	moves,
	outputs,
	polybar,
	this_command,
	workspaces,
};

use crate::{
	DEFAULT_CMD, HELP_CMD, WILD_CMD,
	Commands, CommandFn
};
use std::collections::HashMap;

lazy_static! {
	pub static ref CMD: String = "group".to_string();

	pub static ref SUBCMDS: Commands = {
		let mut cmds = HashMap::new();
		cmds.insert(DEFAULT_CMD, exec as CommandFn);

		cmds.insert("list", list as CommandFn);
		cmds.insert("assign", assign as CommandFn);

		cmds.insert("list-active", list_active as CommandFn);

		cmds.insert("show", show as CommandFn);
		cmds.insert("hide", hide as CommandFn);
		cmds.insert("toggle", toggle as CommandFn);
		cmds.insert("only", only as CommandFn);
		cmds.insert("all", all as CommandFn);

		cmds.insert(WILD_CMD, exec as CommandFn);
		cmds.insert(HELP_CMD, help as CommandFn);
		cmds
	};
}

pub fn help(_: Vec<String>) {
	println!("{} {} <command> [...args]", this_command(), CMD.as_str());
	println!("    A set of commands for managing workspace groups\n\r");
	println!("    Main commands:\n\r");
	println!("      {} {} list", this_command(), CMD.as_str());
	println!("          List all groups\n\r");
	println!("      {} {} assign <workspace-name> <group-name>", this_command(), CMD.as_str());
	println!("          Assign a workspace to the group\n\r");
	println!("    Manage active groups:\n\r");
	println!("      {} {} list-active", this_command(), CMD.as_str());
	println!("          List all active groups\n\r");
	println!("      {} {} show <group-name>", this_command(), CMD.as_str());
	println!("          Add group to list of active groups\n\r");
	println!("      {} {} hide <group-name>", this_command(), CMD.as_str());
	println!("          Remove group to list of active groups\n\r");
	println!("      {} {} toggle <group-name>", this_command(), CMD.as_str());
	println!("          Toggle the group on and off the list of active groups\n\r");
	println!("      {} {} only <group-name>", this_command(), CMD.as_str());
	println!("          Set the group as the exclusive active group\n\r");
	println!("      {} {} all <group-name>", this_command(), CMD.as_str());
	println!("          Activate all groups\n\r");
}

pub fn exec(args: Vec<String>) {
	let mut constraints = constraint::from_vec(args.to_owned());

	if args.len() == 0 {
		constraints.add(Constraint::Output);
		constraints.output = outputs::focused();
	}

	let groups = groups::available(constraints);
	let output = serde_json::to_string_pretty(&groups).unwrap();
	println!("{}", output);
}

pub fn assign(args: Vec<String>) {
	let group_name = if args.len() > 1 {
		args[1].clone()
	} else if args.len() == 1 {
		args[0].clone()
	} else { "".to_owned() };

	let possible_ws = if args.len() > 1 {
		let ws_name = args[0].clone();
		workspaces::by_name(ws_name.clone())
	} else {
		Some(workspaces::focused())
	};

	if let Some(ws) = possible_ws {
		moves::rename(ws, group_name);
	} else {
		println!("No workspace named {}", args[0].clone());
	}

	polybar::update();
}

pub fn list(args: Vec<String>) {
	let mut constraints = constraint::from_vec(args.to_owned());

	if args.len() == 0 {
		constraints.add(Constraint::Output);
		constraints.output = outputs::focused();
	}

	let groups = groups::available(constraints);
	let output = serde_json::to_string_pretty(&groups).unwrap();
	println!("{}", output);
}

pub fn list_active(args: Vec<String>) {
	let output = if args.len() > 0 {
		args[0].clone()
	} else {
		outputs::focused()
	};

	let groups = groups::active(output);
	let output = serde_json::to_string_pretty(&groups).unwrap();
	println!("{}", output);
}

pub fn all(args: Vec<String>) {
	let output = if args.len() > 0 {
		args[0].clone()
	} else {
		outputs::focused()
	};

	let groups = groups::all(output);
	let output = serde_json::to_string_pretty(&groups).unwrap();
	println!("{}", output);
}

pub fn only(mut args: Vec<String>) {
	let group_name = args.remove(0);

	let output = if args.len() > 0 {
		args[0].clone()
	} else {
		outputs::focused()
	};

	let groups = groups::only(group_name, output);
	let output = serde_json::to_string_pretty(&groups).unwrap();
	println!("{}", output);
}

pub fn show(mut args: Vec<String>) {
	let group_name = args.remove(0);

	let output = if args.len() > 0 {
		args[0].clone()
	} else {
		outputs::focused()
	};

	let groups = groups::show(group_name, output);
	let output = serde_json::to_string_pretty(&groups).unwrap();
	println!("{}", output);
}

pub fn hide(mut args: Vec<String>) {
	let group_name = args.remove(0);

	let output = if args.len() > 0 {
		args[0].clone()
	} else {
		outputs::focused()
	};

	let groups = groups::hide(group_name, output);
	let output = serde_json::to_string_pretty(&groups).unwrap();
	println!("{}", output);
}

pub fn toggle(mut args: Vec<String>) {
	let group_name = args.remove(0);

	let output = if args.len() > 0 {
		args[0].clone()
	} else {
		outputs::focused()
	};

	let groups = groups::toggle(group_name, output);
	let output = serde_json::to_string_pretty(&groups).unwrap();
	println!("{}", output);
}
