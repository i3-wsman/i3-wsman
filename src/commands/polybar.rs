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
		cmds.insert(HELP_CMD, help as CommandFn);

		cmds.insert(DEFAULT_CMD, exec as CommandFn);
		cmds.insert("module-groups", module_groups as CommandFn);
		cmds.insert("module-show-hidden", module_show_hidden as CommandFn);
		cmds.insert("module-workspaces", module_workspaces as CommandFn);

		cmds.insert("set-workspace", workspace as CommandFn);
		cmds.insert("set-group", group as CommandFn);
		cmds.insert("toggle-show-hidden", toggle_hidden as CommandFn);
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

pub fn toggle_hidden(_: Vec<String>) {
	groups::toggle_hidden();
	polybar::update();
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

pub fn module_groups(_: Vec<String>) {
	let show_hidden = groups::show_hidden_enabled();
	let focused_output = outputs::focused();

	let mut show_constraints = Constraints::new();
	show_constraints.add(Constraint::Output);
	if !show_hidden {
		show_constraints.add(Constraint::Group);
	}
	show_constraints.output = focused_output.clone();

	let mut avail_constraints = Constraints::new();
	avail_constraints.add(Constraint::Output);
	avail_constraints.output = focused_output.clone();

	let groups = groups::available(avail_constraints);
	let mut active_groups = groups::active(focused_output.clone());
	let showing_all = active_groups.len() == 0 || groups == active_groups;

	let mut all_button = polybar::Label::new("all", 1, 0);
	let cmd = this_command_abs() + " polybar set-group all";
	all_button.set_action(polybar::LEFT_CLICK, &cmd);
	all_button.font = Some(1);

	if showing_all {
		all_button.set_colors(polybar::defaults::FOCUSED_FG, polybar::defaults::FOCUSED_BG);
		active_groups = vec![];
	} else {
		all_button.set_colors(polybar::defaults::FG, polybar::defaults::BG);
	}

	print!("{}", all_button);

	let focused_ws = workspaces::focused();
	let focused_group = name::group(focused_ws.name.as_str());
	for g in groups {
		let mut group_btn = polybar::Label::new(&g, 1, 0);

		let left_click = this_command_abs() + " polybar set-group only " + g.as_ref();
		let secondary_click = this_command_abs() + " polybar set-group toggle " + g.as_ref();

		group_btn.set_actions(
			Some(left_click),
			Some(secondary_click.clone()),
			Some(secondary_click)
		);

		if active_groups.contains(&g) {
			group_btn.set_colors(
				"ff8080f0",
				"b9010202"
			);
		} else {
			let bg_color =if &g == &focused_group {
				polybar::defaults::VISIBLE_BG
			} else {
				"82010202"
			};

			group_btn.set_colors(
				polybar::defaults::VISIBLE_FG,
				bg_color
			);
		}

		print!("{}", group_btn);
	}
}

pub fn module_show_hidden(_: Vec<String>) {
	let focused_output = outputs::focused();
	let active_groups = groups::active(focused_output.clone());

	let mut avail_constraints = Constraints::new();
	avail_constraints.add(Constraint::Output);
	avail_constraints.output = focused_output.clone();
	let groups = groups::available(avail_constraints);

	let show_hidden = groups::show_hidden_enabled();
	let showing_all = active_groups.len() == 0 || groups == active_groups;

	let mut toggle_hidden = polybar::Label::new(
		polybar::defaults::TOGGLE_HIDDEN_LABEL,
		1,
		1
	);

	if showing_all {
		toggle_hidden.fg_color = Some(polybar::defaults::TOGGLE_HIDDEN_ALL_FG.to_owned());
	} else if show_hidden {
		toggle_hidden.fg_color = Some(polybar::defaults::TOGGLE_HIDDEN_ON_FG.to_owned());
	} else {
		toggle_hidden.fg_color = Some(polybar::defaults::TOGGLE_HIDDEN_OFF_FG.to_owned());
	}

	let cmd = this_command_abs() + " polybar toggle-show-hidden";
	toggle_hidden.set_actions(Some(cmd), None, None);

	print!("{}", toggle_hidden);
}

pub fn module_workspaces(_: Vec<String>) {
	let show_hidden = groups::show_hidden_enabled();
	let focused_output = outputs::focused();

	let mut show_constraints = Constraints::new();
	show_constraints.add(Constraint::Output);
	if !show_hidden {
		show_constraints.add(Constraint::Group);
	}
	show_constraints.output = focused_output.clone();

	let focused_ws = workspaces::focused();
	let mut workspaces = workspaces::get(show_constraints, false);

	if !workspaces.contains(&focused_ws) && focused_ws.output == focused_output {
		workspaces.push(focused_ws);
	}

	workspaces.sort_by(
		|w1, w2| w1.num.cmp(&w2.num)
	);

	let active_groups = groups::active(focused_output.clone());
	for ws in workspaces {
		let mut ws_label_btn = polybar::Label::new(
			polybar::defaults::WS_LABEL, 2, 0
		);
		ws_label_btn.font = Some(3);

		let cmd = this_command_abs() + " polybar set-workspace " + ws.num.to_string().as_ref();
		ws_label_btn.set_action(polybar::LEFT_CLICK, &cmd);

		let ws_group = name::group(ws.name.as_str());
		if active_groups.len() == 0 || active_groups.contains(&ws_group) {
			if ws.focused {
				ws_label_btn.set_colors(
					polybar::defaults::FOCUSED_FG,
					polybar::defaults::FOCUSED_BG
				);
				ws_label_btn.label = polybar::defaults::FOCUSED_WS_LABEL.to_string();
			} else if ws.urgent {
				ws_label_btn.set_colors(
					polybar::defaults::URGENT_FG,
					polybar::defaults::URGENT_BG
				);
				ws_label_btn.label = polybar::defaults::URGENT_WS_LABEL.to_string();
			} else if ws.visible {
				ws_label_btn.set_colors(
					polybar::defaults::VISIBLE_FG,
					polybar::defaults::VISIBLE_BG
				);
				ws_label_btn.label = polybar::defaults::VISIBLE_WS_LABEL.to_string();
			}
		} else {
			if ws.focused {
				ws_label_btn.set_colors(
					polybar::defaults::HIDDEN_FOCUSED_FG,
					polybar::defaults::HIDDEN_FOCUSED_BG
				);
				ws_label_btn.label = polybar::defaults::HIDDEN_FOCUSED_WS_LABEL.to_string();
			} else if ws.urgent {
				ws_label_btn.set_colors(
					polybar::defaults::HIDDEN_URGENT_FG,
					polybar::defaults::HIDDEN_URGENT_BG
				);
				ws_label_btn.label = polybar::defaults::HIDDEN_URGENT_WS_LABEL.to_string();
			} else if ws.visible {
				ws_label_btn.set_colors(
					polybar::defaults::HIDDEN_VISIBLE_FG,
					polybar::defaults::HIDDEN_VISIBLE_BG
				);
				ws_label_btn.label = polybar::defaults::HIDDEN_VISIBLE_WS_LABEL.to_string();
			} else {
				ws_label_btn.set_colors(
					polybar::defaults::HIDDEN_FG,
					polybar::defaults::HIDDEN_BG
				);
				ws_label_btn.label = polybar::defaults::HIDDEN_WS_LABEL.to_string();
			}
		}

		print!("{}", ws_label_btn);
	}
}

pub fn exec(_: Vec<String>) {
	module_groups(vec![]);
	module_show_hidden(vec![]);
	module_workspaces(vec![]);
}
