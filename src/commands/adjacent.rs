extern crate i3_ipc;

use i3_ipc::{Connect, I3};

use crate::common::{
	this_command,
	polybar_update,
	get_active_workspace,
	move_workspace_right,
	get_workspace_by_num,
	change_prefix,
};

pub fn exec(args: Vec<String>) {
	if args.contains(&"help".to_string()) {
		println!("{} adjacent <direction>", this_command());
		println!("    Creates a new workspace next to the current workspace\n\r");
		println!("    Directions: right, left");
		return;
	}

	let active_ws = get_active_workspace();
	let active_ws_name = active_ws.name.to_owned();
	let active_ws_num = active_ws.num;

	let new_ws_num = if args[0] == "left" {
		move_workspace_right(active_ws);
		active_ws_num
	} else {
		let nn = active_ws_num + 1;
		let ws_to_move = get_workspace_by_num(nn);
		if let Some(moveit) = ws_to_move {
			move_workspace_right(moveit);
		}
		nn
	};

	let new_ws_name = change_prefix(
		&active_ws_name,
		new_ws_num
	);

	let mut i3 = I3::connect().unwrap();
	let cmd = format!("workspace {}", new_ws_name);
	i3.run_command(cmd).ok();

	polybar_update();
}
