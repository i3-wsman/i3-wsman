use i3_ipc::{Connect, I3, reply::Workspace};

use super::{Direction, name, neighbor};

pub fn right(ws: Workspace) {
	let ws_name = ws.name.to_owned();
	let new_num = ws.num + 1;
	let new_name = name::change_prefix(ws_name.as_ref(), new_num);

	let neighbor = neighbor::immediate(ws, Direction::Right);
	if let Some(n) = neighbor {
		right(n);
	}

	let mut i3 = I3::connect().unwrap();
	let cmd = format!("rename workspace {} to {}", ws_name, new_name);
	i3.run_command(cmd).ok();
}

pub fn rename(ws: Workspace, new_name: String) {
	let ws_name = ws.name.to_owned();
	let ws_num = ws.num.to_owned();
	let mut i3 = I3::connect().unwrap();
	let cmd = if new_name == "" {
		format!("rename workspace {} to {}", ws_name, ws_num)
	} else {
		format!("rename workspace {} to {}:{}", ws_name, ws_num, new_name)
	};
	i3.run_command(cmd).ok();
}
