use crate::{
	common::this_command_abs,
	groups,
	i3::{self, Workspace},
	polybar::Actions,
	POLYBAR_CFG,
};

pub fn exec(mut args: Vec<String>) {
	let hide_all_button = args.len() > 0 && args.remove(0) == "no-all";

	let focused_output = i3::get_current_output();

	let groups = groups::list_for_output(Some(focused_output.clone()));
	let active_groups = groups::active_for_output(Some(focused_output.clone()));
	let showing_all = focused_output.showing_all();

	let mut format = POLYBAR_CFG.get_format("groups", None);

	if !hide_all_button {
		let all_state = match showing_all {
			true => "all-activated".to_string(),
			false => "all".to_string(),
		};

		let mut all_button = POLYBAR_CFG.get_label("groups", Some(all_state), None);

		all_button.actions = Some(Actions {
			left_click: Some(this_command_abs() + " polybar group all"),
			middle_click: None,
			right_click: None,
		});

		format.labels.insert("all".to_owned(), vec![all_button]);
	} else {
		format.labels.insert("all".to_owned(), vec![]);
	}

	let focused_ws = i3::get_current_workspace();
	let focused_group = focused_ws.group();
	let mut state_label = vec![];
	for g in groups {
		let left_click = this_command_abs() + " polybar group only " + g.as_ref();
		let secondary_click = this_command_abs() + " polybar group toggle " + g.as_ref();

		let group_actions = Actions {
			left_click: Some(left_click),
			middle_click: Some(secondary_click.clone()),
			right_click: Some(secondary_click),
		};

		let group_state = if showing_all {
			if &g == &focused_group {
				"focused"
			} else {
				"unfocused"
			}
		} else if active_groups.contains(&g) {
			"activated"
		} else {
			if &g == &focused_group {
				"hidden-focused"
			} else {
				"hidden-unfocused"
			}
		};

		let mut group_btn = POLYBAR_CFG.get_label("groups", Some(group_state.to_owned()), None);

		group_btn.tokens.insert("name".to_owned(), g.to_owned());

		let count = i3::get_workspaces()
			.iter()
			.filter(|w| w.group() == g)
			.collect::<Vec<&Workspace>>()
			.len();

		group_btn
			.tokens
			.insert("count".to_owned(), count.to_string());

		group_btn.actions = Some(group_actions);

		state_label.push(group_btn);
	}
	format.labels.insert("state".to_owned(), state_label);

	print!("{}", format);
}
