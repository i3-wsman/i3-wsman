use i3_ipc::reply::Workspace;

use super::{
	constraint::Constraints,
	Direction, workspaces
};

pub fn get(ws: Workspace, constraints: Constraints, direction: Direction) -> Option<Workspace> {
	let workspaces = workspaces::get(constraints, direction == Direction::Left);

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

pub fn closest_anywhere(ws: Workspace, constraints: Constraints) -> Option<Workspace> {
	let workspaces = workspaces::get(constraints, false);

	let mut closest_ws: Option<Workspace> = None;
	let mut closest_distance: i32 = i32::MAX;

	for workspace in workspaces.iter() {
		let distance = (ws.num - workspace.num).abs();
		if distance < closest_distance {
			closest_distance = distance;
			closest_ws = Some(workspace.clone());
		}
	}

	closest_ws
}

pub fn closest(ws: Workspace, direction: Direction) -> Option<Workspace> {
	let constraints = Constraints::new();
	get(ws, constraints, direction)
}

#[allow(dead_code)]
pub fn immediate(ws: Workspace, direction: Direction) -> Option<Workspace> {
	let constraints = Constraints::new();

	let ws_num = ws.num;

	let neighbor = get(ws, constraints, direction);

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

