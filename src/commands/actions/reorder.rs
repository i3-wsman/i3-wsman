use std::collections::HashMap;

use crate::{
	common::{constraint::Constraint, this_command, Direction},
	i3::{self, get_filtered_criteria, get_focused_workspace, workspace::assemble_name},
	polybar,
};

use crate::{CommandFn, Commands, DEFAULT_CMD, HELP_CMD, WILD_CMD};

lazy_static! {
	pub static ref CMD: String = "reorder".to_string();
	pub static ref SUBCMDS: Commands = {
		let mut cmds = HashMap::new();
		cmds.insert(DEFAULT_CMD, exec as CommandFn);
		cmds.insert(WILD_CMD, exec as CommandFn);
		cmds.insert(HELP_CMD, help as CommandFn);
		cmds
	};
}

pub fn help(_: Vec<String>) {
	println!("{} {} <right|left>", this_command(), CMD.as_str());
	println!("    Reorders the current workspaces to the left or right\n\r");
	println!("    {} {} right", this_command(), CMD.as_str());
	println!("        Adjusts the order of the workspace to the right\n\r");
	println!("    {} {} left", this_command(), CMD.as_str());
	println!("        Adjusts the order of the workspace to the left\n\r");
}

pub fn exec(args: Vec<String>) {
	let dir: Direction = args[0].parse().unwrap();

	let mut criteria = get_filtered_criteria(true);
	criteria.remove(Constraint::AllowUrgent);

	let focused = get_focused_workspace();

	if let Some(neighbor) = focused.get_closest_neighbor(Some(criteria), Some(dir)) {
		let focused_name = focused.full_name();
		let neighbor_name = neighbor.full_name();

		let new_focused_name = assemble_name(neighbor.num(), focused.group(), focused.name());
		let new_neighbor_name = assemble_name(focused.num(), neighbor.group(), neighbor.name());

		i3::run_command(format!(
			"rename workspace {} to reorderinprogress:{}",
			focused_name, focused_name
		));
		i3::run_command(format!(
			"rename workspace {} to {}",
			neighbor_name, new_neighbor_name
		));
		i3::run_command(format!(
			"rename workspace reorderinprogress:{} to {}",
			focused_name, new_focused_name
		));
	}

	polybar::update();
}
