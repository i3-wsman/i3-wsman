use i3_ipc::{Connect, I3, reply::Workspace};

use super::{Direction, name, neighbor, workspaces};

pub fn scoot(ws: Workspace) -> Workspace {
	let prev_neighbor = neighbor::closest(ws.clone(), Direction::Left);
	let next_neighbor = neighbor::closest(ws.clone(), Direction::Right);

	let new_pos = if let Some(prev_n) = prev_neighbor {
		prev_n.num + 2
	} else {
		2
	};

	let new_name = if let Some(next_n) = next_neighbor {
		if next_n.num == new_pos {
			scoot(next_n);
			reorder(ws, new_pos)
		} else {
			let n = reorder(ws, new_pos);
			scoot(next_n);
			n
		}
	} else {
		reorder(ws, new_pos)
	};

	workspaces::by_name(new_name).unwrap()
}

pub fn reorder(ws: Workspace, new_pos: i32) -> String {
	let ws_name = ws.name.to_owned();
	let group = name::group(&ws_name);

	let new_name = if group == "" {
		new_pos.to_string()
	} else {
		format!("{}:{}", new_pos, group)
	};

	let mut i3 = I3::connect().unwrap();
	i3.run_command(format!("rename workspace {} to {}", ws_name, new_name)).ok();

	new_name
}

pub fn rename(ws: Workspace, new_name: String) -> String {
	let ws_name = ws.name;
	let ws_num = ws.num.to_owned();

	let new_name = format!("{}:{}", ws_num, new_name);

	let mut i3 = I3::connect().unwrap();
	i3.run_command(format!("rename workspace \"{}\" to \"{}\"", ws_name, new_name)).ok();

	new_name
}

