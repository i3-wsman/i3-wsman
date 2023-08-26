mod common;
mod commands;

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

pub type CommandFn = fn(Vec<String>);
pub type Commands = HashMap<&'static str, CommandFn>;
pub type CommandMap = HashMap<&'static str, Commands>;

pub static DEFAULT_CMD: &str = "";
pub static HELP_CMD: &str = "help";
pub static WILD_CMD: &str = "*";

fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 1 {
		println!("Please provide a command.");
		return;
	}

	let mut cmds: CommandMap = HashMap::new();

	cmds.insert(commands::goto::CMD.as_str(), commands::goto::SUBCMDS.clone());
	cmds.insert(commands::next::CMD.as_str(), commands::next::SUBCMDS.clone());
	cmds.insert(commands::prev::CMD.as_str(), commands::prev::SUBCMDS.clone());

	cmds.insert(commands::adjacent::CMD.as_str(), commands::adjacent::SUBCMDS.clone());
	cmds.insert(commands::reorder::CMD.as_str(), commands::reorder::SUBCMDS.clone());
	// @TODO: move-container-to

	cmds.insert(commands::group::CMD.as_str(), commands::group::SUBCMDS.clone());

	cmds.insert(commands::polybar::CMD.as_str(), commands::polybar::SUBCMDS.clone());
	cmds.insert(commands::poke::CMD.as_str(), commands::poke::SUBCMDS.clone());

	cmds.insert(commands::get_workspaces::CMD.as_str(), commands::get_workspaces::SUBCMDS.clone());

	let help_order = vec![
		commands::goto::CMD.as_str(),
		commands::next::CMD.as_str(),
		commands::prev::CMD.as_str(),

		commands::adjacent::CMD.as_str(),
		commands::reorder::CMD.as_str(),

		commands::group::CMD.as_str(),

		commands::polybar::CMD.as_str(),
		commands::poke::CMD.as_str(),

		commands::get_workspaces::CMD.as_str(),
	];

	let command = if args.len() > 1 { args[1].as_str() } else { DEFAULT_CMD };

	match cmds.get(command) {
		Some(cmd_map) => {
			let subcmd = if args.len() > 2 { args[2].as_str() } else { DEFAULT_CMD };
			match cmd_map.get(subcmd) {
				Some(func) if subcmd == DEFAULT_CMD => {
					func(args[2..].to_vec());
				}
				Some(func) => {
					func(args[3..].to_vec());
				}
				None => {
					let func = if cmd_map.contains_key(WILD_CMD) {
						cmd_map.get(WILD_CMD).unwrap()
					} else {
						cmd_map.get(HELP_CMD).unwrap()
					};

					func(args[2..].to_vec());
				}
			}

			return;
		}
		None if command == "constraints" => {
			println!("Usage: {} <command> <...args>", common::this_command());

			help_constraints(vec![]);
		}
		None if command == "help" => {
			println!("Usage: {} <command> <...args>", common::this_command());

			if args.len() < 3 || args[2].as_str() != "constraints" {
				for cmd_str in help_order {
					if !cmds.contains_key(cmd_str) { continue; }
					let cmd = cmds.get(cmd_str).unwrap();
					if !cmd.contains_key(HELP_CMD) { continue; }
					let help_fn = cmd.get("help").unwrap();
					help_fn(vec![]);
					println!("");
				}
			}

			help_constraints(vec![]);
		}
		_ => {
			println!("Usage: {} <command> <...args>", common::this_command());

			println!("\n\rFor detailed usage, run:");
			println!("    {} <command> help", common::this_command());
			println!("\r\nAvailable commands:");
			for &cmd in cmds.keys() {
				println!("    {} {}", common::this_command(), cmd);
			}
			println!("    {} help", common::this_command());
		}
	};
}

fn help_constraints(_: Vec<String>) {
	println!("\n\rConstraints:");
	println!("    focused\t\tFocused Workspace");
	println!("    visible\t\tVisible Workspace");
	println!("    hidden\t\tNon-visible Workspaces");
	println!("    group\t\tWorkspace is part of an active group");
	println!("    nogroup\t\tWorkspace has no group");
	println!("    no-group\t\tWorkspace has no group");
	println!("    allowurgent\t\tWorkspace is Urgent");
	println!("    allow-urgent\tWorkspace is Urgent");
	println!("    output\t\tWorkspace is on focused display");
	println!("    output=xyz\t\tWorkspace is on display 'xyz'\n\r");
	println!("    For instance, to get all hidden workspaces on the current monitor:");
	println!("        {} get-workspaces hidden output\n\r", common::this_command());
}

