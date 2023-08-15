use i3_ipc::{Connect, I3};

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

		cmds.insert("get", get as CommandFn);
		cmds.insert("set", set as CommandFn);

		cmds.insert("active", active as CommandFn);

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
	println!("{} {}", this_command(), CMD.as_str());
	println!("    Pokes polybar to update");
}

pub fn exec(args: Vec<String>) {
	let mut constraints = constraint::parse(args.to_owned());

	if args.len() == 0 {
		constraints.add(Constraint::Output);
		constraints.output = outputs::focused();
	}

	let groups = groups::available(constraints);
	let output = serde_json::to_string_pretty(&groups).unwrap();
	println!("{}", output);
}

pub fn set(args: Vec<String>) {
	let group_name = if args.len() > 1 { args[1].clone() } else { args[0].clone() };
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

pub fn get(args: Vec<String>) {
	let mut constraints = constraint::parse(args.to_owned());

	if args.len() == 0 {
		constraints.add(Constraint::Output);
		constraints.output = outputs::focused();
	}

	let groups = groups::available(constraints);
	let output = serde_json::to_string_pretty(&groups).unwrap();
	println!("{}", output);
}

pub fn active(args: Vec<String>) {
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
