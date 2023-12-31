use std::collections::HashMap;

use crate::{
	common::{constraint::Constraint, this_command},
	config::global::GotoBehavior,
	groups,
	i3::{self, get_filtered_criteria, get_matching_workspaces},
	polybar, CONFIG,
};

use crate::{CommandFn, Commands, DEFAULT_CMD, HELP_CMD, WILD_CMD};

lazy_static! {
	pub static ref CMD: String = "goto".to_string();
	pub static ref SUBCMDS: Commands = {
		let mut cmds = HashMap::new();
		cmds.insert(WILD_CMD, exec as CommandFn);
		cmds.insert(DEFAULT_CMD, help as CommandFn);
		cmds.insert(HELP_CMD, help as CommandFn);
		cmds
	};
}

pub fn help(_: Vec<String>) {
	println!(
		"{} {} <nth> [{}|{}]",
		this_command(),
		CMD.as_str(),
		GotoBehavior::Create.to_string(),
		GotoBehavior::Stop.to_string(),
	);
	println!(
		"    Focuses on the <nth> workspace, where <nth> is the position of the workspace\n\r"
	);
	println!(
		"    {} {} <nth> {}",
		this_command(),
		CMD.as_str(),
		GotoBehavior::Create.to_string(),
	);
	println!("        If workspace doesn't exist, creates a new workspace.\n\r");
	println!(
		"    {} {} <nth> {}",
		this_command(),
		CMD.as_str(),
		GotoBehavior::Stop.to_string(),
	);
	println!("        If workspace doesn't exist, do nothing.\n\r");
}

pub fn exec(mut args: Vec<String>) {
	let nth_try = args.remove(0).parse::<usize>();
	if nth_try.is_err() {
		help(vec![]);
		return;
	}

	let behavior = GotoBehavior::from_argv(&mut args).unwrap();

	let mut criteria = get_filtered_criteria(true);
	criteria.remove(Constraint::Focused);

	if !CONFIG.navigation.goto.restrict_to_output {
		criteria.remove(Constraint::Output);
	}

	if CONFIG.navigation.goto.include_hidden_groups {
		criteria.remove(Constraint::Group);
		criteria.remove(Constraint::NoGroup);
	} else if groups::show_hidden_enabled() && CONFIG.navigation.goto.honor_show_hidden {
		criteria.remove(Constraint::Group);
		criteria.remove(Constraint::NoGroup);
	}

	let workspaces = get_matching_workspaces(criteria);

	let nth = nth_try.unwrap();
	if nth < 1 {
		if behavior == GotoBehavior::Create {
			let last_ws = workspaces.first().unwrap();
			i3::run_command(format!("workspace {}", last_ws.full_name()));
			crate::commands::actions::adjacent::exec(vec!["left".to_owned()]);
		}
		return;
	}

	if workspaces.len() < nth {
		match behavior {
			GotoBehavior::Create => {
				let last_ws = workspaces.last().unwrap();
				i3::run_command(format!("workspace {}", last_ws.full_name()));
				crate::commands::actions::adjacent::exec(vec!["right".to_owned()]);
			}
			GotoBehavior::Stop => {}
		};
	} else {
		let target_ws = workspaces.get(nth - 1).unwrap();
		i3::run_command(format!("workspace {}", target_ws.full_name()));
	}

	polybar::update();
}
