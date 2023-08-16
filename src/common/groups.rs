use super::{
	constraint::{Constraints, Constraint},
	name,
	state,
	workspaces,
};

pub fn available(constraints: Constraints) -> Vec<String> {
	let workspaces = workspaces::get(constraints, false);

	let mut groups: Vec<String> = vec![];
	for ws in workspaces {
		let g = name::group(ws.name.as_str());
		if g.len() > 0 {
			groups.push(g);
		}
	}

	groups.sort();
	groups.dedup();

	groups
}

pub fn available_output(output: String) -> Vec<String> {
	let mut constraints = Constraints::new();

	constraints.add(Constraint::Output);
	constraints.output = output;

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

	if !state.groups.contains_key(&output) {
		return available_output(output);
	}

	let mut groups = state.groups.get(&output).unwrap().to_owned();
	if groups.len() == 0 {
		return available_output(output);
	}
	groups.sort();
	groups.dedup();
	groups
}

pub fn all(output: String) -> Vec<String> {
	let mut state = state::get();

	let groups = vec![];
	state.groups.insert(output, groups.clone());

	state::set(state);

	groups.to_owned()
}

pub fn only(group_name: String, output: String) -> Vec<String> {
	let mut state = state::get();

	let groups = vec![group_name.to_owned()];
	state.groups.insert(output, groups.clone());

	state::set(state);

	groups.to_owned()
}

pub fn only_toggle(group_name: String, output: String) -> Vec<String> {
	let mut state = state::get();

	let groups = if state.groups.contains_key(&output) {
		let groups = state.groups.get(&output).unwrap().to_owned();
		if groups.len() == 1 && groups.contains(&group_name) {
			vec![]
		} else {
			vec![group_name.to_owned()]
		}
	} else {
		vec![group_name.to_owned()]
	};

	state.groups.insert(output, groups.clone());

	state::set(state);

	groups.to_owned()
}

pub fn hide(group_name: String, output: String) -> Vec<String> {
	let mut state = state::get();

	let mut groups = if state.groups.contains_key(&output) {
		let mut groups = state.groups.get(&output).unwrap().to_owned();
		groups.retain(|g| g != &group_name);
		groups
	} else {
		vec![group_name.to_owned()]
	};

	groups.sort();
	groups.dedup();

	state.groups.insert(output, groups.clone());

	state::set(state);

	groups
}

pub fn show(group_name: String, output: String) -> Vec<String> {
	let mut state = state::get();

	let mut groups = if state.groups.contains_key(&output) {
		let mut groups = state.groups.get(&output).unwrap().to_owned();
		if !groups.contains(&group_name) {
			groups.push(group_name.to_owned());
		}
		groups
	} else {
		vec![group_name.to_owned()]
	};

	groups.sort();
	groups.dedup();

	if groups == available_output(output.to_owned()) {
		groups = vec![];
	}

	state.groups.insert(output, groups.clone());

	state::set(state);

	groups
}

pub fn toggle(group_name: String, output: String) -> Vec<String> {
	let mut state = state::get();

	let mut groups = if state.groups.contains_key(&output) {
		let mut groups = state.groups.get(&output).unwrap().to_owned();
		if groups.contains(&group_name) {
			groups.retain(|g| g != &group_name);
		} else {
			groups.push(group_name.to_owned());
		}
		groups
	} else {
		vec![group_name.to_owned()]
	};

	groups.sort();
	groups.dedup();

	if groups == available_output(output.to_owned()) {
		groups = vec![];
	}

	state.groups.insert(output, groups.clone());
	state::set(state);

	groups
}
