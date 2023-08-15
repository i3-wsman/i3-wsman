use i3_ipc::{Connect, I3};

use crate::common::{
	this_command,
	this_command_abs,
	outputs,
	workspaces,
	polybar,
	constraint::{ Constraints, Constraint }
};

use crate::{
	DEFAULT_CMD, HELP_CMD, WILD_CMD,
	Commands, CommandFn
};
use std::collections::HashMap;

lazy_static! {
	pub static ref CMD: String = "polybar".to_string();

	pub static ref SUBCMDS: Commands = {
		let mut cmds = HashMap::new();
		cmds.insert(DEFAULT_CMD, exec as CommandFn);
		cmds.insert(HELP_CMD, help as CommandFn);
		cmds.insert(WILD_CMD, action as CommandFn);
		cmds
	};
}

pub fn help(_: Vec<String>) {
	println!("{} polybar", this_command());
	println!("    The i3 Workspace Manager Polybar module");
	println!("    To use, add the following to your polybar config.ini:\n\r");
	println!("    [module/i3wsm]");
	println!("    type = custom/ipc");
	println!("    hook-0 = {} polybar", this_command());
	println!("    initial = 1");
}

pub fn action(args: Vec<String>) {
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
	let mut constraints = Constraints::new();

	constraints.add(Constraint::Output);
	constraints.output = outputs::active();

	let workspaces = workspaces::get(constraints, false);

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

		let cmd = this_command_abs() + " polybar " + ws.num.to_string().as_ref();

		let prefix = format!("%{{B#{}}}%{{F#{}}}%{{A1:{}:}}", bgcolor, fgcolor, cmd);
		let suffix = "%{A}%{F-}%{B-}";
		print!("{}  {}  {}", prefix, label, suffix);
	}
}
