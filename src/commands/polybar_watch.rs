use crate::common::{
	this_command,
	constraint,
	workspaces,
};

use crate::{DEFAULT_CMD, HELP_CMD, Commands, CommandFn};
use std::collections::HashMap;

lazy_static! {
	pub static ref CMD: String = "polybar-watch".to_string();

	pub static ref SUBCMDS: Commands = {
		let mut cmds = HashMap::new();
		cmds.insert(DEFAULT_CMD, exec as CommandFn);
		cmds.insert(HELP_CMD, help as CommandFn);
		cmds
	};
}

pub fn help(_: Vec<String>) {
	println!("{} {} [...constraints]", this_command(), CMD.as_str());
	println!("    Returns workspaces matching the constraints.");
	println!("    Constraints are optional. If none are provided, all workspaces are returned.\n\r");
	println!("    Constraints:");
	println!("      focused: Focused Workspace");
	println!("      visible: Visible Workspaces");
	println!("      hidden: Hidden Workspaces");
	println!("      group: Workspaces apart of the focused Group");
	println!("      output: Workspaces on the output ");
	println!("      output=xyz: Workspaces on the output xyz");
	println!("");
	println!("    For instance, to get all hidden workspaces on the current monitor:");
	println!("        {} get-workspaces hidden output", this_command());
}

pub fn exec(args: Vec<String>) {
	let constraints = constraint::parse(args.to_owned());

	let workspaces = workspaces::get(constraints, false);
	let output = serde_json::to_string_pretty(&workspaces).unwrap();
	println!("{}", output);
}
