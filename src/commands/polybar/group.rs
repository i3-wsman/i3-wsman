use crate::{common::polybar, groups};

pub fn exec(args: Vec<String>) {
	let group_action = args[0].clone();

	if group_action == "toggle" {
		let group_name = args[1].clone();
		groups::toggle(group_name, None);
	} else if group_action == "only" {
		let group_name = args[1].clone();
		groups::toggle_only(group_name, None);
	} else {
		groups::show_all(None);
	}

	let output = serde_json::to_string_pretty(&groups::active_for_output(None)).unwrap();
	println!("{}", output);
	polybar::update();
}
