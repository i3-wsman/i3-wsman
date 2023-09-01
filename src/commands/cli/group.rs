use crate::{
	common::this_command,
	groups,
	i3::{self, Workspace},
	polybar,
};

use crate::{CommandFn, Commands, DEFAULT_CMD, HELP_CMD, WILD_CMD};
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
	println!(
		"      {} {} assign [<workspace-name>] <group-name>",
		this_command(),
		CMD.as_str()
	);
	println!("          Assign a workspace to the group\n\r");
	println!("    Manage active groups:\n\r");
	println!("      {} {} list-active", this_command(), CMD.as_str());
	println!("          List all active groups\n\r");
	println!(
		"      {} {} show <group-name>",
		this_command(),
		CMD.as_str()
	);
	println!("          Add group to list of active groups\n\r");
	println!(
		"      {} {} hide <group-name>",
		this_command(),
		CMD.as_str()
	);
	println!("          Remove group to list of active groups\n\r");
	println!(
		"      {} {} toggle <group-name>",
		this_command(),
		CMD.as_str()
	);
	println!("          Toggle the group on and off the list of active groups\n\r");
	println!(
		"      {} {} only <group-name>",
		this_command(),
		CMD.as_str()
	);
	println!("          Set the group as the exclusive active group\n\r");
	println!("      {} {} all <group-name>", this_command(), CMD.as_str());
	println!("          Activate all groups\n\r");
}

pub fn exec(args: Vec<String>) {
	list(args);
}

pub fn assign(args: Vec<String>) {
	let group_name = if args.len() > 1 {
		args[1].clone()
	} else if args.len() == 1 {
		args[0].clone()
	} else {
		"".to_owned()
	};

	let possible_ws = if args.len() > 1 {
		let ws_name = args[0].clone();
		Workspace::by_name(ws_name.as_str())
	} else {
		Some(i3::get_current_workspace())
	};

	if let Some(mut ws) = possible_ws {
		ws.set_group(group_name);
	} else {
		println!("No workspace named {}", args[0].clone());
	}

	polybar::update();
}

pub fn list(_: Vec<String>) {
	let groups = groups::list_for_output(None);
	let output = serde_json::to_string_pretty(&groups).unwrap();
	println!("{}", output);
}

pub fn list_active(_: Vec<String>) {
	let groups = groups::active_for_output(None);
	let output = serde_json::to_string_pretty(&groups).unwrap();
	println!("{}", output);
}

pub fn all(_: Vec<String>) {
	let groups = groups::show_all(None);
	let output = serde_json::to_string_pretty(&groups).unwrap();
	println!("{}", output);
}

pub fn only(mut args: Vec<String>) {
	let group_name = args.remove(0);
	let groups = groups::show_only(group_name, None);
	let output = serde_json::to_string_pretty(&groups).unwrap();
	println!("{}", output);
}

pub fn show(mut args: Vec<String>) {
	let group_name = args.remove(0);
	let groups = groups::show(group_name, None);
	let output = serde_json::to_string_pretty(&groups).unwrap();
	println!("{}", output);
}

pub fn hide(mut args: Vec<String>) {
	let group_name = args.remove(0);
	let groups = groups::hide(group_name, None);
	let output = serde_json::to_string_pretty(&groups).unwrap();
	println!("{}", output);
}

pub fn toggle(mut args: Vec<String>) {
	let group_name = args.remove(0);
	let groups = groups::toggle(group_name, None);
	let output = serde_json::to_string_pretty(&groups).unwrap();
	println!("{}", output);
}
