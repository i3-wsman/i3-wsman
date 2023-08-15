extern crate i3_ipc;
extern crate serde_json;

pub mod constraint;

// use std::io;
// use std::env;
// use std::fs;
// use std::path::Path;

use std::env;
use std::path::PathBuf;
use i3_ipc::{Connect, I3, reply::Workspace};
use constraint::{ Constraints, Constraint };

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
	Right,
	Left
}

pub fn polybar_update() {
	use std::process::Command;

	let _output = Command::new("polybar-msg")
		.arg("action")
		.arg("#i3wsm.hook.0")
		.output()
		.ok();
}

pub fn this_command_abs() -> String {
	env::current_exe().unwrap().to_str().unwrap().to_string()
}

pub fn this_command() -> String {
	let path: PathBuf = env::current_exe().unwrap();
	path.file_name().unwrap().to_str().unwrap().to_string()
}

pub fn get_output() -> String {
	match env::var("MONITOR") {
		Ok(val) => val as String,
		Err(_e) => get_active_workspace().output,
	}
}

pub fn get_constraints(nouns: Vec<String>) -> Constraints {
	let mut constraints = Constraints::new();

	for noun in &nouns {
		if noun.contains(&"output=") {
			constraints.add(Constraint::Output);
			constraints.output = noun
				.split("=")
				.nth(1)
				.unwrap_or("none")
				.to_string();
			continue;
		}

		match noun.as_ref() {
			"focused" => constraints.add(Constraint::Focused),
			"visible" => constraints.add(Constraint::Visible),
			"hidden" => constraints.add(Constraint::Hidden),
			"group" => constraints.add(Constraint::Group),
			"output" => {
				if !constraints.contains(Constraint::Output) {
					constraints.add(Constraint::Output);
					constraints.output = get_output();
				}
			},
			_ => {},
		}
	}

	constraints
}

pub fn get_workspaces(constraints: Constraints, reverse: bool) -> Vec<Workspace> {
	let mut i3 = I3::connect().unwrap();
	let mut workspaces = i3.get_workspaces().unwrap();

	if reverse {
		workspaces.sort_by(
			|w1, w2| w2.num.cmp(&w1.num)
		);
	} else {
		workspaces.sort_by(
			|w1, w2| w1.num.cmp(&w2.num)
		);
	}

	if constraints.contains(Constraint::None) {
		return workspaces
	}

	workspaces
		.iter()
		.filter(|ws| {
			if constraints.contains(Constraint::Visible) {
				if !ws.visible { return false }
			} else if constraints.contains(Constraint::Hidden) {
				if ws.visible { return false }
				if ws.focused { return false }
			}

			if constraints.contains(Constraint::Focused) {
				if !ws.focused { return false }
			}

			if constraints.contains(Constraint::Output) && constraints.output != "none" {
				if constraints.output != ws.output { return false }
			}

			if constraints.contains(Constraint::Group) {
				todo!();
			}

			return true
		})
		.cloned()
		.collect::<Vec<Workspace>>()
}

pub fn get_first_workspace(constraints: Constraints) -> Workspace {
	let workspaces = get_workspaces(constraints, false);
	workspaces.first().unwrap().to_owned()
}

pub fn get_active_workspace() -> Workspace {
	let mut i3 = I3::connect().unwrap();
	let workspaces = i3.get_workspaces().unwrap();

	workspaces
		.iter()
		.find(|ws| ws.focused)
		.unwrap()
		.to_owned()
}

pub fn get_workspace_by_num(n: i32) -> Option<Workspace> {
	let mut i3 = I3::connect().unwrap();
	let workspaces = i3.get_workspaces().unwrap();

	let ws = workspaces
		.iter()
		.find(|ws| ws.num == n);

	match ws {
		Some(w) => Some(w.to_owned()),
		None => None,
	}
}

pub fn get_neighbor(ws: Workspace, constraints: Constraints, direction: Direction) -> Option<Workspace> {
	let workspaces = get_workspaces(constraints, direction == Direction::Left);

	let neighbor = if direction == Direction::Right {
		workspaces
			.iter()
			.find(|nws| nws.num > ws.num)
	} else {
		workspaces
			.iter()
			.find(|nws| nws.num < ws.num)
	};

	match neighbor {
		Some(n) => Some(n.to_owned()),
		None => None
	}
}

pub fn get_immediate_neighbor(ws: Workspace, direction: Direction) -> Option<Workspace> {
	let constraints = Constraints::new();

	let ws_num = ws.num;

	let neighbor = get_neighbor(ws, constraints, direction);

	if let Some(n) = neighbor {
		match direction {
			Direction::Right => {
				if n.num == ws_num + 1 {
					return Some(n)
				}
				return None
			},
			Direction::Left => {
				if n.num == ws_num - 1 {
					return Some(n)
				}
				return None
			}
		}
	}

	None
}

pub fn change_prefix(name: &str, new_prefix: i32) -> String {
	let parts: Vec<&str> = name.split(':').collect();
	if parts.len() > 1 {
		format!("{}:{}", new_prefix, parts[1])
	} else {
		new_prefix.to_string()
	}
}

pub fn move_workspace_right(ws: Workspace) {
	let ws_name = ws.name.to_owned();
	let new_num = ws.num + 1;
	let new_name = change_prefix(ws_name.as_ref(), new_num);

	let neighbor = get_immediate_neighbor(ws, Direction::Right);
	if let Some(n) = neighbor {
		move_workspace_right(n);
	}

	let mut i3 = I3::connect().unwrap();
	let cmd = format!("rename workspace {} to {}", ws_name, new_name);
	i3.run_command(cmd).ok();
}

// pub fn get_active_workspace(workspaces: &str) -> Option<serde_json::Value> {
// 	let parsed_workspaces: serde_json::Value = serde_json::from_str(workspaces).unwrap();
// 	parsed_workspaces
// 		.as_array()?
// 		.iter()
// 		.find(|ws| ws["focused"] == true)
// 		.cloned()
// }
// 
// pub fn get_neighbor(workspaces: &str, current_num: i64, direction: &str) -> Option<serde_json::Value> {
// 	let parsed_workspaces: serde_json::Value = serde_json::from_str(workspaces).unwrap();
// 	let output_workspaces: Vec<serde_json::Value> = parsed_workspaces
// 		.as_array()?
// 		.iter()
// 		.filter(|ws| ws["output"] == parsed_workspaces[current_num as usize]["output"])
// 		.cloned()
// 		.collect();
// 
// 	let adjacent_num = match direction {
// 		"left" => current_num - 1,
// 		"right" => current_num + 1,
// 		_ => return None,
// 	};
// 
// 	output_workspaces
// 		.iter()
// 		.find(|ws| ws["num"].as_i64() == Some(adjacent_num))
// 		.cloned()
// }
// 
// // ... other functions ...
// 
// pub fn test_common() {
// 	// Equivalent to TEST_i3wsm::common in the original script
// 	// You can implement this function to test the common functionalities
// }

