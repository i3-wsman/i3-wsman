use crate::{common::this_command_abs, groups, i3, polybar::Actions, POLYBAR_CFG};

pub fn exec(_: Vec<String>) {
	let focused_output = i3::get_current_output();
	let workspaces = i3::get_filtered_workspaces(false);
	let active_groups = groups::active_for_output(Some(focused_output.clone()));
	let showing_all = focused_output.showing_all();

	let mut format = POLYBAR_CFG.get_format("workspaces", None);
	// let separator = POLYBAR_CFG.get_label("workspaces", None, Some("separator".to_owned()));
	// let output_separator = POLYBAR_CFG.get_label("workspaces", None, Some("output-separator".to_owned()));

	let mut cur_output = "".to_string();
	let mut state_label = vec![];
	for ws in workspaces {
		if cur_output.len() > 0 && cur_output != ws.output() {
			// state_label.push(output_separator.clone());
			println!("  ");
		} else {
			// state_label.push(separator.clone());
		}
		cur_output = ws.output();

		let ws_group = ws.group();
		let (section, ws_state) = if ws_group == "" || active_groups.contains(&ws_group) {
			if ws_group == "" && !showing_all {
				if ws.focused() {
					("workspaces/unassigned", "unassigned-focused")
				} else if ws.urgent() {
					("workspaces/unassigned", "unassigned-urgent")
				} else if ws.visible() {
					("workspaces/unassigned", "unassigned-visible")
				} else {
					("workspaces/unassigned", "unassigned-unfocused")
				}
			} else {
				if ws.focused() {
					("workspaces", "focused")
				} else if ws.urgent() {
					("workspaces", "urgent")
				} else if ws.visible() {
					("workspaces", "visible")
				} else {
					("workspaces", "unfocused")
				}
			}
		} else {
			if ws.focused() {
				("workspaces/group-hidden", "focused")
			} else if ws.urgent() {
				("workspaces/group-hidden", "urgent")
			} else if ws.visible() {
				("workspaces/group-hidden", "visible")
			} else {
				("workspaces/group-hidden", "unfocused")
			}
		};

		let mut ws_label_btn = POLYBAR_CFG.get_label(section, Some(ws_state.to_owned()), None);
		let cmd = this_command_abs() + " polybar goto " + ws.num().to_string().as_ref();

		ws_label_btn.actions = Some(Actions {
			left_click: Some(cmd),
			middle_click: None,
			right_click: None,
		});

		state_label.push(ws_label_btn);
	}

	format.labels.insert("state".to_string(), state_label);
	print!("{}", format);
}
