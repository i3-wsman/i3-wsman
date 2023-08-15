use i3_ipc::{Connect, I3};

use crate::common::{
	this_command,
	this_command_abs,
	groups,
	name,
	outputs,
	polybar,
	workspaces,
	constraint::{ Constraints, Constraint }
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
		cmds.insert(DEFAULT_CMD, exec as CommandFn);
		cmds.insert(HELP_CMD, help as CommandFn);
		cmds.insert("workspace", workspace as CommandFn);
		cmds.insert("group", group as CommandFn);
		cmds
	};
}

pub fn help(_: Vec<String>) {
	println!("{} {}", this_command(), CMD.as_str());
	println!("    The i3 Workspace Manager Polybar module");
	println!("    To use, add the following to your polybar config.ini:\n\r");
	println!("    [module/i3wsm]");
	println!("    type = custom/ipc");
	println!("    hook-0 = {} polybar", this_command());
	println!("    initial = 1");
}

pub fn group(args: Vec<String>) {
	let group_action = args[0].clone();

	let groups = if group_action == "toggle" {
		let group_name = args[1].clone();
		groups::toggle(group_name, outputs::focused())
	} else if group_action == "only" {
		let group_name = args[1].clone();
		groups::only(group_name, outputs::focused())
	} else { // "all"
		groups::all( outputs::focused())
	};

	let output = serde_json::to_string_pretty(&groups).unwrap();
	println!("{}", output);
	polybar::update();
}

pub fn workspace(args: Vec<String>) {
	let wsarg = args[0].parse::<i32>();
	match wsarg {
		Ok(ws_num) => {
			if let Some(ws) = workspaces::by_num(ws_num) {
				let mut i3 = I3::connect().unwrap();
				let cmd = format!("workspace {}", ws.name);
				i3.run_command(cmd).ok();
			} else {
				println!("No workspace numbered {}, or it no longer exists", ws_num);
			}
		}
		Err(_) => {
			exec(vec!["help".to_string()]);
		}
	}
	polybar::update();
}

pub fn exec(_: Vec<String>) {
	let mut show_constraints = Constraints::new();
	show_constraints.add(Constraint::Output);
	show_constraints.add(Constraint::Group);
	show_constraints.output = outputs::focused();

	let mut avail_constraints = Constraints::new();
	avail_constraints.add(Constraint::Output);
	avail_constraints.output = outputs::focused();

	let groups = groups::available(avail_constraints);
	let mut active_groups = groups::active(outputs::focused());

	let mut fgcolor = "ccfdfefe";
	let mut bgcolor = "82010202";
	if active_groups.len() == 0 || groups == active_groups {
		fgcolor = "ff8080f0";
		bgcolor = "b9010202";
		active_groups = vec![];
	}

	let cmd = this_command_abs() + " polybar group all";
	let prefix = format!("%{{T1}}%{{B#{}}}%{{F#{}}}%{{A1:{}:}}", bgcolor, fgcolor, cmd);
	let suffix = "%{A}%{F-}%{B-}%{T-}";
	print!("{} all {}", prefix, suffix);

	let focused_ws = workspaces::focused();
	let focused_group = name::group(focused_ws.name.as_str());
	for g in groups {
		let mut fgcolor = "ccfdfefe";
		let mut bgcolor = "82010202";

		if &g == &focused_group {
			bgcolor = "99010202";
		}

		if active_groups.contains(&g) {
			fgcolor = "ff8080f0";
			bgcolor = "b9010202";
		}

		let cmd1 = this_command_abs() + " polybar group only " + g.as_ref();
		let cmd2 = this_command_abs() + " polybar group toggle " + g.as_ref();
		let prefix = format!("%{{B#{}}}%{{F#{}}}%{{A1:{}:}}%{{A2:{}:}}%{{A3:{}:}}", bgcolor, fgcolor, cmd1, cmd2, cmd2);
		let suffix = "%{A}%{A}%{A}%{F-}%{B-}";
		print!("{} {} {}", prefix, g, suffix);
	}

	print!("  %{{T3}}");

	let workspaces = workspaces::get(show_constraints, false);

	for ws in workspaces {
		let mut fgcolor = "33fdfefe";
		let mut bgcolor = "77000000";

		let mut label = "";

		if ws.focused {
			fgcolor = "ff8080f0";
			bgcolor = "b9010202";
			label = "";
		} else if ws.urgent {
			fgcolor = "ffc2bd60";
			label = "";
		} else if ws.visible {
			fgcolor = "ccfdfefe";
			bgcolor = "99010202";
			label = "";
		}

		let cmd = this_command_abs() + " polybar workspace " + ws.num.to_string().as_ref();

		let prefix = format!("%{{B#{}}}%{{F#{}}}%{{A1:{}:}}", bgcolor, fgcolor, cmd);
		let suffix = "%{A}%{F-}%{B-}";
		print!("{}  {}  {}", prefix, label, suffix);
	}
	print!("%{{T-}}");
}
