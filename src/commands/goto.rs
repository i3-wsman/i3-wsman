use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

use crate::{
	common::{
		constraint::{Constraint, Criteria},
		polybar, this_command,
	},
	i3::{self, get_current_output, get_filtered_workspaces},
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

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum GotoBehavior {
	Create,
	Stop,
}

impl FromStr for GotoBehavior {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.len() == 0 {
			return Ok(GotoBehavior::Stop);
		}

		match s.to_lowercase().as_ref() {
			"create" => Ok(GotoBehavior::Create),
			"stop" => Ok(GotoBehavior::Stop),
			_ => {
				eprintln!("Warning: Invalid value '{}' for 'navigation.goto.behavior'. Falling back to 'Stop'.", s);
				Ok(GotoBehavior::Stop)
			}
		}
	}
}

impl GotoBehavior {
	#[allow(dead_code)]
	pub fn to_string(&self) -> String {
		match self {
			GotoBehavior::Create => "create".to_string(),
			GotoBehavior::Stop => "stop".to_string(),
		}
	}
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

	let behavior: GotoBehavior = args
		.get(0)
		.unwrap_or(&"".to_string())
		.parse()
		.unwrap_or(GotoBehavior::Stop);

	let workspaces = get_filtered_workspaces(false);

	let nth = nth_try.unwrap();
	if nth < 1 {
		if behavior == GotoBehavior::Create {
			let last_ws = workspaces.first().unwrap();
			i3::run_command(format!("workspace {}", last_ws.full_name()));
			crate::commands::adjacent::exec(vec!["left".to_owned()]);
		}
		return;
	}

	if workspaces.len() < nth {
		match behavior {
			GotoBehavior::Create => {
				let last_ws = workspaces.last().unwrap();
				i3::run_command(format!("workspace {}", last_ws.full_name()));
				crate::commands::adjacent::exec(vec!["right".to_owned()]);
			}
			GotoBehavior::Stop => {}
		};
	} else {
		let target_ws = workspaces.get(nth - 1).unwrap();
		i3::run_command(format!("workspace {}", target_ws.full_name()));
	}

	polybar::update();
}
