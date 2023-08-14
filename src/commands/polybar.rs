use crate::common::{
	this_command,
	get_output,
	get_workspaces,
	constraint::{ Constraints, Constraint }
};

pub fn exec(args: Vec<String>) {
	if args.contains(&"help".to_string()) {
		println!("{} polybar", this_command());
		println!("    The i3 Workspace Manager Polybar module");
		println!("    To use, add the following to your polybar config.ini:\n\r");
		println!("    [module/i3wsm]");
		println!("    type = custom/ipc");
		println!("    hook-0 = {} polybar", this_command());
		println!("    initial = 1");
		return;
	}

	let mut constraints = Constraints::new();

	constraints.add(Constraint::Output);
	constraints.output = get_output();

	let workspaces = get_workspaces(constraints, false);

	for ws in workspaces {
		let mut fgcolor = "33fdfefe";
		let mut bgcolor = "77000000";

		let mut label = "";

		if ws.urgent {
			fgcolor = "ffc2bd60";
			label = "";
		} else if ws.focused {
			fgcolor = "ff8080f0";
			bgcolor = "b9010202";
			label = "";
		} else if ws.visible {
			fgcolor = "ccfdfefe";
			bgcolor = "99010202";
			label = "";
		}

		let prefix = format!("%{{B#{}}}%{{F#{}}}", bgcolor, fgcolor);
		let suffix = "%{F-}%{B-}";
		print!("{}  {}  {}", prefix, label, suffix);
	}
}
