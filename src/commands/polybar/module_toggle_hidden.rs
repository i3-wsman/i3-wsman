use crate::{
	common::this_command_abs, groups, i3::get_current_output, polybar::Actions, POLYBAR_CFG,
};

pub fn exec(_: Vec<String>) {
	let output = get_current_output();
	let show_hidden = groups::show_hidden_enabled();
	let showing_all = output.showing_all();

	let key_state = if showing_all {
		"disabled"
	} else if show_hidden {
		"on"
	} else {
		"off"
	};

	let actions = Actions {
		left_click: Some(this_command_abs() + " polybar toggle"),
		middle_click: None,
		right_click: None,
	};

	let label = POLYBAR_CFG.get_label("show-hidden-toggle", Some(key_state));
	let mut format = POLYBAR_CFG.get_format("show-hidden-toggle", Some(key_state), label);
	format.actions = Some(actions);

	print!("{}", format);
}
