use crate::{common::this_command_abs, groups, i3, polybar::Actions, POLYBAR_CFG};

static FOCUSED: &str = "focused";
static URGENT: &str = "urgent";
static VISIBLE: &str = "visible";
static UNFOCUSED: &str = "unfocused";

static SECT_WORKSPACES: &str = "workspaces";
static SECT_UNASSIGNED: &str = "workspaces/unassigned";
static SECT_HIDDEN: &str = "workspaces/group-hidden";

pub fn exec(_: Vec<String>) {
	let focused_output = i3::get_current_output();
	let workspaces = i3::get_filtered_workspaces(false);
	let active_groups = groups::active_for_output(Some(focused_output.clone()));
	// let showing_all = focused_output.showing_all();

	let enable_click = POLYBAR_CFG.enable_click();

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
		let section = if ws_group == "" {
			SECT_UNASSIGNED
		} else if active_groups.contains(&ws_group) {
			SECT_WORKSPACES
		} else {
			SECT_HIDDEN
		};

		let ws_state = if ws.focused() {
			FOCUSED
		} else if ws.urgent() {
			URGENT
		} else if ws.visible() {
			VISIBLE
		} else {
			UNFOCUSED
		};

		let mut ws_label_btn = POLYBAR_CFG.get_label(section, Some(ws_state.to_owned()), None);

		let cmd = this_command_abs() + " polybar goto " + ws.num().to_string().as_ref();

		if enable_click {
			ws_label_btn.actions = Some(Actions {
				left_click: Some(cmd),
				middle_click: None,
				right_click: None,
			});
		}

		ws_label_btn.tokens.insert("name".to_owned(), ws.name());
		ws_label_btn.tokens.insert("group".to_owned(), ws.group());
		ws_label_btn
			.tokens
			.insert("full_name".to_owned(), ws.full_name());
		ws_label_btn
			.tokens
			.insert("index".to_owned(), ws.num().to_string());
		ws_label_btn.tokens.insert("output".to_owned(), ws.output());

		state_label.push(ws_label_btn);
	}

	format.labels.insert("state".to_string(), state_label);
	print!("{}", format);
}
