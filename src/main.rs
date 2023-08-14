mod common;
mod commands;

use std::collections::HashMap;

type CommandFn = fn(Vec<String>);

fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 1 {
		println!("Please provide a command.");
		return;
	}

	// Define the commands
	let mut commands: HashMap<&str, CommandFn> = HashMap::new();
	commands.insert("get-workspaces", commands::get_workspaces::exec as CommandFn);
	commands.insert("next", commands::next::exec as CommandFn);
	commands.insert("adjacent", commands::adjacent::exec as CommandFn);
	commands.insert("polybar", commands::polybar::exec as CommandFn);

	let command = if args.len() > 1 { args[1].as_str() } else { "help" };
	match commands.get(command) {
		Some(&func) => {
			func(args[2..].to_vec());
		}
		_ => {
			println!("Usage: {} <command> <...args>", common::this_command());

			for &func in commands.values() {
				println!("");
				func(vec!["help".to_string()]);
			}
		}
	}

}

