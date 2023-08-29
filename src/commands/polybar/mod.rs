mod goto_workspace;
mod group;
mod help;
mod module_groups;
mod module_toggle_hidden;
mod module_workspaces;
mod poke;
mod toggle_hidden;
mod watch;

use crate::common::{this_command, this_command_abs};

use crate::{CommandFn, Commands, DEFAULT_CMD, HELP_CMD};
use std::collections::HashMap;

lazy_static! {
	pub static ref CMD: String = "polybar".to_string();
	pub static ref SUBCMDS: Commands = {
		let mut cmds = HashMap::new();
		cmds.insert(HELP_CMD, help::exec as CommandFn);

		cmds.insert(DEFAULT_CMD, exec as CommandFn);

		cmds.insert("module-groups", module_groups::exec as CommandFn);
		cmds.insert(
			"module-toggle-hidden",
			module_toggle_hidden::exec as CommandFn,
		);
		cmds.insert("module-workspaces", module_workspaces::exec as CommandFn);

		cmds.insert("goto", goto_workspace::exec as CommandFn);
		cmds.insert("group", group::exec as CommandFn);
		cmds.insert("toggle", toggle_hidden::exec as CommandFn);

		cmds.insert("watch", watch::exec as CommandFn);

		cmds.insert("poke", poke::exec as CommandFn);

		cmds
	};
}

pub fn exec(_: Vec<String>) {
	module_groups::exec(vec![]);
	module_toggle_hidden::exec(vec![]);
	module_workspaces::exec(vec![]);
}
