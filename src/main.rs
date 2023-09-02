#[macro_use]
extern crate lazy_static;

use i3_ipc::{Connect, I3Stream, I3 as I3_api};
use std::collections::HashMap;

mod commands;
mod common;
mod config;
mod groups;
mod i3;
mod polybar;
mod state;

lazy_static! {
	pub static ref CONFIG: config::global::Config = config::global::load_cfg();
	pub static ref POLYBAR_CFG: config::polybar::Config = Default::default();
	pub static ref I3: I3Stream = I3_api::connect().unwrap();
}

pub type CommandFn = fn(Vec<String>);
pub type Commands = HashMap<&'static str, CommandFn>;
pub type CommandMap = HashMap<&'static str, Commands>;

pub static DEFAULT_CMD: &str = "";
pub static HELP_CMD: &str = "help";
pub static WILD_CMD: &str = "*";

fn main() {
	let args: Vec<String> = std::env::args().collect();

	let mut cmds: CommandMap = HashMap::new();

	cmds.insert(
		commands::actions::goto::CMD.as_str(),
		commands::actions::goto::SUBCMDS.clone(),
	);
	cmds.insert(
		commands::actions::next::CMD.as_str(),
		commands::actions::next::SUBCMDS.clone(),
	);
	cmds.insert(
		commands::actions::prev::CMD.as_str(),
		commands::actions::prev::SUBCMDS.clone(),
	);

	cmds.insert(
		commands::actions::adjacent::CMD.as_str(),
		commands::actions::adjacent::SUBCMDS.clone(),
	);
	cmds.insert(
		commands::actions::reorder::CMD.as_str(),
		commands::actions::reorder::SUBCMDS.clone(),
	);
	cmds.insert(
		commands::cli::rename::CMD.as_str(),
		commands::cli::rename::SUBCMDS.clone(),
	);
	// // @TODO: move-container-to

	cmds.insert(
		commands::cli::group::CMD.as_str(),
		commands::cli::group::SUBCMDS.clone(),
	);

	cmds.insert(
		commands::polybar::CMD.as_str(),
		commands::polybar::SUBCMDS.clone(),
	);

	cmds.insert(
		commands::cli::get_workspaces::CMD.as_str(),
		commands::cli::get_workspaces::SUBCMDS.clone(),
	);

	cmds.insert(
		commands::cli::config::CMD.as_str(),
		commands::cli::config::SUBCMDS.clone(),
	);

	cmds.insert(
		commands::cli::maintenance::CMD.as_str(),
		commands::cli::maintenance::SUBCMDS.clone(),
	);

	let help_order: Vec<&str> = vec![
		commands::actions::goto::CMD.as_str(),
		commands::actions::next::CMD.as_str(),
		commands::actions::prev::CMD.as_str(),
		commands::actions::adjacent::CMD.as_str(),
		commands::actions::reorder::CMD.as_str(),
		commands::cli::rename::CMD.as_str(),
		commands::cli::group::CMD.as_str(),
		commands::polybar::CMD.as_str(),
		commands::cli::get_workspaces::CMD.as_str(),
	];

	let command = if args.len() > 1 {
		args[1].as_str()
	} else {
		DEFAULT_CMD
	};

	match cmds.get(command) {
		Some(cmd_map) => {
			let subcmd = if args.len() > 2 {
				args[2].as_str()
			} else {
				DEFAULT_CMD
			};

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
		}
		None if command == "constraints" => {
			println!("Usage: {} <command> <...args>", common::this_command());

			help_constraints(vec![]);
		}
		None if command == "help" => {
			println!("Usage: {} <command> <...args>", common::this_command());

			if args.len() < 3 || args[2].as_str() != "constraints" {
				for cmd_str in help_order {
					if !cmds.contains_key(cmd_str) {
						continue;
					}
					let cmd = cmds.get(cmd_str).unwrap();
					if !cmd.contains_key(HELP_CMD) {
						continue;
					}
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

	state::release_i3_lock().ok();
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
	println!(
		"        {} get-workspaces hidden output\n\r",
		common::this_command()
	);
}
