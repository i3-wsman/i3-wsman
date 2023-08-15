use i3_ipc::reply::Workspace;

use super::{ constraint::Constraints, Direction, workspaces };

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

