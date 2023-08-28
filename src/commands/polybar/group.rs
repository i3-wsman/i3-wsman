use crate::common::{groups, outputs, polybar};

pub fn exec(args: Vec<String>) {
	let group_action = args[0].clone();

	let groups = if group_action == "toggle" {
		let group_name = args[1].clone();
		groups::toggle(group_name, outputs::focused())
	} else if group_action == "only" {
		let group_name = args[1].clone();
		groups::only_toggle(group_name, outputs::focused())
	} else {
		// "all"
		groups::all(outputs::focused())
	};

	let output = serde_json::to_string_pretty(&groups).unwrap();
	println!("{}", output);
	polybar::update();
}
