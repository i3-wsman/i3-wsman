use crate::common::{
	this_command_abs,
	groups,
	name,
	outputs,
	polybar,
	workspaces,
	constraint::{ Constraints, Constraint },
};

pub fn exec(mut args: Vec<String>) {
	let hide_all_button = args.len() > 0 && args.remove(0) == "no-all";

	let show_hidden = groups::show_hidden_enabled();
	let focused_output = outputs::focused();

	let mut show_constraints = Constraints::new();
	show_constraints.add(Constraint::Output);
	if !show_hidden {
		show_constraints.add(Constraint::Group);
	}
	show_constraints.output = focused_output.clone();

	let groups = groups::available_output(focused_output.clone());
	let mut active_groups = groups::active(focused_output.clone());
	let showing_all = active_groups.len() == 0 || groups == active_groups;

	if !hide_all_button {
		let mut all_button = polybar::Label::new(polybar::defaults::GROUP_ALL_LABEL, 1, 0);
		let cmd = this_command_abs() + " polybar group all";
		all_button.set_action(polybar::LEFT_CLICK, &cmd);
		all_button.font = Some(1);

		if showing_all {
			all_button.set_colors(polybar::defaults::FOCUSED_FG, polybar::defaults::FOCUSED_BG);
		} else {
			all_button.set_colors(polybar::defaults::FG, polybar::defaults::BG);
		}

		print!("{}", all_button);
	}

	if showing_all {
		active_groups = vec![];
	}

	let focused_ws = workspaces::visible_or_focused(&focused_output);
	let focused_group = name::group(focused_ws.name.as_str());
	for g in groups {
		let mut group_btn = polybar::Label::new(&g, 1, 0);
		group_btn.font = Some(1);

		let left_click = this_command_abs() + " polybar group only " + g.as_ref();
		let secondary_click = this_command_abs() + " polybar group toggle " + g.as_ref();

		group_btn.set_actions(
			Some(left_click),
			Some(secondary_click.clone()),
			Some(secondary_click)
		);

		if showing_all {
			if &g == &focused_group {
				group_btn.set_colors(
					polybar::defaults::GROUP_FOCUSED_FG,
					polybar::defaults::GROUP_FOCUSED_BG
				);
			} else {
				group_btn.set_colors(
					polybar::defaults::GROUP_FG,
					polybar::defaults::GROUP_BG
				);
			}
		} else if active_groups.contains(&g) {
			group_btn.set_colors(
				polybar::defaults::GROUP_ACTIVE_FG,
				polybar::defaults::GROUP_ACTIVE_BG
			);
		} else { // Hidden
			if &g == &focused_group {
				group_btn.set_colors(
					polybar::defaults::GROUP_HIDDEN_FOCUSED_FG,
					polybar::defaults::GROUP_HIDDEN_FOCUSED_BG
				);
			} else {
				group_btn.set_colors(
					polybar::defaults::GROUP_HIDDEN_FG,
					polybar::defaults::GROUP_HIDDEN_BG
				);
			}
		}

		print!("{}", group_btn);
	}
}
