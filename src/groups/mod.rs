use serde::{Deserialize, Serialize};

use crate::{
	common::{
		constraint::{Constraint, Criteria},
		dedup_vec,
	},
	i3::{self, Output},
	state, CONFIG,
};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub enum GroupSortMethod {
	Alphabetical,
	PreserveOrder,
}

fn get_output(output: Option<Output>) -> Output {
	match output {
		Some(o) => o,
		None => i3::get_current_output(),
	}
}

pub fn list_for_output(output: Option<Output>) -> Vec<String> {
	let output = get_output(output);

	let mut groups = output.groups();

	if CONFIG.groups.sort_method == GroupSortMethod::Alphabetical {
		groups.sort();
	}

	let mut final_groups = vec![];

	if CONFIG.groups.sort_method == GroupSortMethod::PreserveOrder
		&& CONFIG.groups.sort_default_first != true
	{
		final_groups.extend(groups);
		final_groups.extend(CONFIG.groups.default_groups.to_owned());
	} else {
		final_groups.extend(CONFIG.groups.default_groups.to_owned());
		final_groups.extend(groups);
	}

	if CONFIG.groups.sort_method == GroupSortMethod::Alphabetical {
		final_groups.sort();
	}

	dedup_vec(&mut final_groups);

	final_groups
}

pub fn active_for_output(output: Option<Output>) -> Vec<String> {
	let output = get_output(output);

	let mut groups = output.groups();
	groups.extend(CONFIG.groups.always_visible.to_owned());
	groups.sort();
	groups.dedup();

	groups
}

fn get_groups_from_state(output: Output) -> Vec<String> {
	let output_name = output.get_state_name();

	let state = state::get();

	match state.groups.get(&output_name) {
		Some(groups) => groups.to_owned(),
		None => vec![],
	}
}

fn update_groups(output: Output, mut groups: Vec<String>) -> Vec<String> {
	let mut state = state::get();
	let output_name = output.clone().get_state_name();

	groups.sort();
	groups.dedup();

	state.groups.insert(output_name, groups.clone());
	state::set(state);

	if CONFIG.focus.auto_focus_nearest_group {
		let mut criteria = Criteria::new();
		criteria.add(Constraint::Output);
		criteria.output = Some(output);

		let next = i3::get_focused_workspace().get_closest_neighbor(Some(criteria), None);
		if let Some(ws) = next {
			i3::run_command(format!("workspace {}", ws.full_name()));
		}
	}

	groups
}

pub fn show_all(output: Option<Output>) -> Vec<String> {
	update_groups(get_output(output), vec![])
}

pub fn show_only(group_name: String, output: Option<Output>) -> Vec<String> {
	update_groups(get_output(output), vec![group_name])
}

pub fn toggle_only(group_name: String, output: Option<Output>) -> Vec<String> {
	let output = get_output(output);
	let groups = get_groups_from_state(output.clone());

	let groups = if groups.len() == 1 && groups.contains(&group_name) {
		vec![]
	} else {
		vec![group_name]
	};

	update_groups(output, groups)
}

pub fn show(group_name: String, output: Option<Output>) -> Vec<String> {
	let output = get_output(output);
	let mut groups = get_groups_from_state(output.clone());
	groups.push(group_name);
	update_groups(output, groups)
}

pub fn hide(group_name: String, output: Option<Output>) -> Vec<String> {
	let output = get_output(output);
	let mut groups = get_groups_from_state(output.clone());
	groups.retain(|g| g != &group_name);
	update_groups(output, groups)
}

pub fn toggle(group_name: String, output: Option<Output>) -> Vec<String> {
	let output = get_output(output);
	let mut groups = get_groups_from_state(output.clone());
	if groups.contains(&group_name) {
		groups.retain(|g| g != &group_name);
	} else {
		groups.push(group_name);
	}
	update_groups(output, groups)
}