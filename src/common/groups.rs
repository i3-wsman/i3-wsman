use crate::CONFIG;

use i3_ipc::{ Connect, I3 };

use crate::common::config::GroupSortMethod;
use crate::common::state::State;

use super::{
	constraint::{Constraints, Constraint},
	name,
	state,
	neighbor,
	workspaces,
};

use std::collections::HashSet;

fn dedup_vec<T: std::hash::Hash + std::cmp::Eq + Clone>(vec: &mut Vec<T>) {
	let mut seen = HashSet::new();
	vec.retain(|e| seen.insert(e.clone()));
}

pub fn available(constraints: Constraints) -> Vec<String> {
	let workspaces = workspaces::get(constraints, false);

	let mut groups: Vec<String> = vec![];
	for ws in workspaces {
		let g = name::group(ws.name.as_str());
		if g.len() > 0 {
			groups.push(g);
		}
	}

	if CONFIG.groups.sort_method == GroupSortMethod::Alphabetical {
		groups.sort();
	}

	let mut final_groups = vec![];

	if CONFIG.groups.sort_method == GroupSortMethod::PreserveOrder && CONFIG.groups.sort_default_first != true {
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

pub fn available_output(output: String) -> Vec<String> {
	let mut constraints = Constraints::new();

	if CONFIG.groups.unique_groups_on_outputs == true {
		constraints.add(Constraint::Output);
		constraints.output = output;
	}

	available(constraints)
}

pub fn show_hidden_enabled() -> bool {
	let state = state::get();
	state.show_hidden
}

pub fn toggle_hidden() -> bool {
	let mut state = state::get();

	let new_state = !state.show_hidden;
	state.show_hidden = new_state;

	state::set(state);

	new_state
}

pub fn active(output: String) -> Vec<String> {
	let state = state::get();

	let mut groups = if CONFIG.groups.unique_groups_on_outputs == true {
		if !state.groups.contains_key(&output) {
			return available_output(output);
		}

		let groups = state.groups.get(&output).unwrap().to_owned();
		if groups.len() == 0 {
			return available_output(output);
		}
		groups
	} else {
		state.global_groups.to_owned()
	};

	groups.extend(CONFIG.groups.always_visible.to_owned());
	groups.sort();
	groups.dedup();

	groups
}

fn update_groups(mut state: State, output: String, groups: Vec<String>) -> Vec<String> {
	if CONFIG.groups.unique_groups_on_outputs == true {
		state.groups.insert(output.clone(), groups.clone());
	} else {
		state.global_groups = groups.clone();
	}

	state::set(state);

	if CONFIG.focus.auto_focus_nearest_group == true {
		let focused = workspaces::visible_or_focused(&output);
		let group = name::group(&focused.name);
		if (
			!groups.contains(&group) && group.len() > 0
		) || (
			CONFIG.focus.hide_unassigned_workspaces == true &&
			groups != available_output(output.to_owned())
		) {
			let mut constraints = Constraints::new();
			constraints.add(Constraint::Output);
			constraints.add(Constraint::Group);
			constraints.output = output;
			if let Some(closest) = neighbor::closest_anywhere(focused, constraints) {
				let mut i3 = I3::connect().unwrap();
				i3.run_command(format!("workspace {}", closest.name)).ok();
			}
		}
	}

	groups.to_owned()
}

pub fn all(output: String) -> Vec<String> {
	let groups = vec![];
	update_groups(state::get(), output, groups)
}

pub fn only(group_name: String, output: String) -> Vec<String> {
	let groups = vec![group_name.to_owned()];
	update_groups(state::get(), output, groups)
}

fn groups_from_state(state: &State, output: String) -> Vec<String> {
	if CONFIG.groups.unique_groups_on_outputs != true {
		state.global_groups.to_owned()
	} else {
		if state.groups.contains_key(&output) {
			state.groups.get(&output).unwrap().to_owned()
		} else {
			vec![]
		}
	}
}

pub fn only_toggle(group_name: String, output: String) -> Vec<String> {
	let state = state::get();

	let groups = groups_from_state(&state, output.to_owned());

	let groups = if groups.len() == 1 && groups.contains(&group_name) {
		vec![]
	} else {
		vec![group_name.to_owned()]
	};

	update_groups(state, output, groups)
}

pub fn hide(group_name: String, output: String) -> Vec<String> {
	let state = state::get();

	let mut groups = groups_from_state(&state, output.to_owned());

	groups.retain(|g| g != &group_name);

	groups.sort();
	groups.dedup();

	update_groups(state, output, groups)
}

pub fn show(group_name: String, output: String) -> Vec<String> {
	let state = state::get();

	let mut groups = groups_from_state(&state, output.to_owned());

	if !groups.contains(&group_name) {
		groups.push(group_name.to_owned());
	}

	groups.sort();
	groups.dedup();

	let mut available = available_output(output.to_owned());
	available.sort();
	available.dedup();

	if groups == available {
		groups = vec![];
	}

	update_groups(state, output, groups)
}

pub fn toggle(group_name: String, output: String) -> Vec<String> {
	let state = state::get();

	let mut groups = groups_from_state(&state, output.to_owned());

	if groups.contains(&group_name) {
		groups.retain(|g| g != &group_name);
	} else {
		groups.push(group_name.to_owned());
	}

	groups.sort();
	groups.dedup();

	let mut available = available_output(output.to_owned());
	available.sort();
	available.dedup();

	if groups == available {
		groups = vec![];
	}

	update_groups(state, output, groups)
}
