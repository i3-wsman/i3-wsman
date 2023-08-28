use crate::common::{constraint, this_command, workspaces};

use crate::{CommandFn, Commands, DEFAULT_CMD, HELP_CMD, WILD_CMD};
use std::collections::HashMap;

lazy_static! {
	pub static ref CMD: String = "get-workspaces".to_string();
	pub static ref SUBCMDS: Commands = {
		let mut cmds = HashMap::new();
		cmds.insert(DEFAULT_CMD, exec as CommandFn);
		cmds.insert(WILD_CMD, exec as CommandFn);
		cmds.insert(HELP_CMD, help as CommandFn);
		cmds
	};
}

pub fn help(_: Vec<String>) {
	println!("{} {} [...constraints]", this_command(), CMD.as_str());
	println!("    Returns workspaces matching the constraints.\n\r");
	println!(
		"    For constraints, run: {} help constraints",
		this_command()
	);
}

pub fn exec(args: Vec<String>) {
	let constraints = constraint::from_vec(args.to_owned());

	let workspaces = workspaces::get(constraints, false);
	let output = serde_json::to_string_pretty(&workspaces).unwrap();
	println!("{}", output);
}
