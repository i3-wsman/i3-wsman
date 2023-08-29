use crate::{
	common::polybar,
	i3::{self, workspace::Workspace},
};

use super::help;

pub fn exec(args: Vec<String>) {
	let wsarg = args[0].parse::<i32>();
	match wsarg {
		Ok(ws_num) => {
			if let Some(ws) = Workspace::by_num(ws_num) {
				i3::run_command(format!("workspace {}", ws.full_name()));
			} else {
				println!("No workspace numbered {}, or it no longer exists", ws_num);
			}
		}
		Err(_) => {
			help::exec(vec![]);
		}
	}
	polybar::update();
}
