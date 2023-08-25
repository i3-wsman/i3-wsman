use crate::common::{
	this_command_abs,
	groups,
	outputs,
	polybar,
	constraint::{ Constraints, Constraint }
};

pub fn exec(_: Vec<String>) {
	let focused_output = outputs::focused();
	let active_groups = groups::active(focused_output.clone());

	let mut avail_constraints = Constraints::new();
	avail_constraints.add(Constraint::Output);
	avail_constraints.output = focused_output.clone();
	let groups = groups::available(avail_constraints);

	let show_hidden = groups::show_hidden_enabled();
	let showing_all = active_groups.len() == 0 || groups == active_groups;

	let mut toggle_hidden = polybar::Label::new(
		polybar::defaults::TOGGLE_HIDDEN_LABEL,
		1,
		0
	);

	if showing_all {
		toggle_hidden.fg_color = Some(polybar::defaults::TOGGLE_HIDDEN_ALL_FG.to_owned());
	} else if show_hidden {
		toggle_hidden.fg_color = Some(polybar::defaults::TOGGLE_HIDDEN_ON_FG.to_owned());
	} else {
		toggle_hidden.fg_color = Some(polybar::defaults::TOGGLE_HIDDEN_OFF_FG.to_owned());
	}

	let cmd = this_command_abs() + " polybar toggle-show-hidden";
	toggle_hidden.set_actions(Some(cmd), None, None);

	print!("{}", toggle_hidden);
}
