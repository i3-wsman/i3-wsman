use i3_ipc::{Connect, I3, reply::Workspace};

use super::{name, groups, outputs};
use super::constraint::{ Constraints, Constraint };

fn get_ws() -> Vec<Workspace> {
	let mut i3 = I3::connect().unwrap();
	i3.get_workspaces().unwrap()
}

pub fn get(constraints: Constraints, reverse: bool) -> Vec<Workspace> {
	let mut workspaces = get_ws();

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
			if constraints.contains(Constraint::AllowUrgent) && ws.urgent {
				return true
			}

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

			let ws_group = name::group(ws.name.as_ref());
			if ws_group.len() == 0 && constraints.contains(Constraint::NoGroup) {
				return true
			}

			if constraints.contains(Constraint::Group) {
				let output = if constraints.output != "none" {
					constraints.output.clone()
				} else {
					outputs::focused()
				};
				let active_groups = groups::active(output);
				if active_groups.len() > 0 {
					return active_groups.contains(&ws_group);
				}
			}

			return true
		})
		.cloned()
		.collect::<Vec<Workspace>>()
}

pub fn first(constraints: Constraints) -> Workspace {
	get(constraints, false).first().unwrap().to_owned()
}

pub fn last(constraints: Constraints) -> Workspace {
	get(constraints, true).first().unwrap().to_owned()
}

pub fn visible(output: &str) -> Workspace {
	let workspaces = get_ws();

	workspaces
		.iter()
		.find(|ws| ws.output == output && (ws.focused || ws.visible))
		.unwrap()
		.to_owned()
}

pub fn focused() -> Workspace {
	let workspaces = get_ws();

	workspaces
		.iter()
		.find(|ws| ws.focused)
		.unwrap()
		.to_owned()
}

pub fn by_name(name: String) -> Option<Workspace> {
	let workspaces = get_ws();

	let ws = workspaces
		.iter()
		.find(|ws| ws.name == name);

	match ws {
		Some(w) => Some(w.to_owned()),
		None => None,
	}
}

pub fn by_num(n: i32) -> Option<Workspace> {
	let workspaces = get_ws();

	let ws = workspaces
		.iter()
		.find(|ws| ws.num == n);

	match ws {
		Some(w) => Some(w.to_owned()),
		None => None,
	}
}
