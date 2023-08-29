use crate::common::{constraint, this_command};
use crate::i3::get_matching_workspaces;

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
	println!("{} {} [...criteria]", this_command(), CMD.as_str());
	println!("    Returns workspaces matching the criteria.\n\r");
	println!("    For criteria, run: {} help criteria", this_command());
}

pub fn exec(args: Vec<String>) {
	let criteria = constraint::from_vec(args.to_owned());

	let workspaces = get_matching_workspaces(criteria);
	let output = serde_json::to_string_pretty(&workspaces).unwrap();
	println!("{}", output);
}
