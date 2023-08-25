mod help;
mod toggle_hidden;
mod module_groups;
mod module_toggle_hidden;
mod module_workspaces;
mod goto_workspace;
mod set_group;

use crate::common::{
	this_command,
	this_command_abs,
};

use crate::{
	DEFAULT_CMD, HELP_CMD,
	Commands, CommandFn
};
use std::collections::HashMap;

lazy_static! {
	pub static ref CMD: String = "polybar".to_string();

	pub static ref SUBCMDS: Commands = {
		let mut cmds = HashMap::new();
		cmds.insert(HELP_CMD, help::exec as CommandFn);

		cmds.insert(DEFAULT_CMD, exec as CommandFn);

		cmds.insert("module-groups", module_groups::exec as CommandFn);
		cmds.insert("module-toggle-hidden", module_toggle_hidden::exec as CommandFn);
		cmds.insert("module-workspaces", module_workspaces::exec as CommandFn);

		cmds.insert("goto-workspace", goto_workspace::exec as CommandFn);
		cmds.insert("set-group", set_group::exec as CommandFn);
		cmds.insert("toggle-show-hidden", toggle_hidden::exec as CommandFn);

		cmds
	};
}

pub fn exec(_: Vec<String>) {
	module_groups::exec(vec![]);
	module_toggle_hidden::exec(vec![]);
	module_workspaces::exec(vec![]);
}
