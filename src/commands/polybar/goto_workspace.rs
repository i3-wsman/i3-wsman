use i3_ipc::{Connect, I3};

use crate::common::{polybar, workspaces};

pub fn exec(args: Vec<String>) {
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
