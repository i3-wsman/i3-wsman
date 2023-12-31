use i3_ipc::{reply, Connect, I3};
use std::env;

use crate::{
	common::constraint::{Constraint, Criteria},
	state, CONFIG, POLYBAR_CFG,
};

pub mod outputs;
pub mod workspace;

pub use outputs::Output;
pub use workspace::Workspace;

pub fn run_command(payload: String) {
	let mut i3 = I3::connect().unwrap();
	state::obtain_i3_lock().ok();
	i3.run_command(payload).ok();
}

// Criteria
pub fn get_filtered_criteria(force_output: bool) -> Criteria {
	let mut criteria = Criteria::new();
	criteria.add(Constraint::Focused);
	criteria.add(Constraint::Group);

	if !CONFIG.focus.hide_unassigned_workspaces {
		criteria.add(Constraint::NoGroup);
	}

	if POLYBAR_CFG.show_hidden_urgent() {
		criteria.add(Constraint::AllowUrgent);
	}

	if force_output || POLYBAR_CFG.pin_workspaces() == true {
		criteria.add(Constraint::Output);
		criteria.output = Some(get_current_output());
	}

	criteria
}

// Workspaces
fn get_workspaces_from_i3() -> Vec<reply::Workspace> {
	let mut i3 = I3::connect().unwrap();
	let workspaces = i3.get_workspaces().unwrap();
	// workspaces.sort_by(|w1, w2| w1.num.cmp(&w2.num));
	workspaces
}

pub fn get_workspaces() -> Vec<Workspace> {
	get_workspaces_from_i3()
		.iter()
		.map(|ws| Workspace::from_ws(ws))
		.collect()
}

pub fn get_focused_workspace() -> Workspace {
	get_workspaces_from_i3()
		.iter()
		.find(|ws| ws.focused)
		.map(|ws| Workspace::from_ws(ws))
		.unwrap()
}

pub fn get_current_workspace_for_output(output: Output) -> Workspace {
	get_workspaces_from_i3()
		.iter()
		.find(|ws| ws.visible && ws.output == output.name())
		.map(|ws| Workspace::from_ws(ws))
		.unwrap()
}

pub fn get_current_workspace() -> Workspace {
	let current_output = get_current_output();
	get_workspaces_from_i3()
		.iter()
		.find(|ws| ws.visible && ws.output == current_output.name())
		.map(|ws| Workspace::from_ws(ws))
		.unwrap()
}

pub fn get_matching_workspaces(criteria: Criteria) -> Vec<Workspace> {
	if criteria.contains(Constraint::None) {
		return get_workspaces();
	}

	get_workspaces_from_i3()
		.iter()
		.map(|ws| Workspace::from_ws(ws))
		.filter(|ws| ws.matches(criteria.clone()))
		.collect()
}

pub fn get_filtered_workspaces(force_output: bool) -> Vec<Workspace> {
	get_matching_workspaces(get_filtered_criteria(force_output))
}

pub fn workspace_maintenance() {
	let mut criteria = Criteria::new();
	criteria.add(Constraint::Output);

	let mut i = 1;
	let mut workspaces = get_workspaces();
	workspaces.sort_by(|w1, w2| w1.num().cmp(&w2.num()));
	for mut ws in workspaces {
		ws.reorder(i);
		i = i + 1;
	}
}

// Outputs
fn get_outputs_from_i3() -> Vec<reply::Output> {
	let mut i3 = I3::connect().unwrap();
	i3.get_outputs()
		.unwrap()
		.iter()
		.filter(|o| o.current_workspace.is_some())
		.map(|o| o.to_owned())
		.collect()
}

pub fn get_outputs() -> Vec<Output> {
	let mut i3 = I3::connect().unwrap();
	i3.get_outputs()
		.unwrap()
		.iter()
		.filter(|o| o.current_workspace.is_some())
		.map(|o| Output::from_output(o))
		.collect()
}

pub fn get_current_output() -> Output {
	let output_name = match env::var("MONITOR") {
		Ok(val) => val as String,
		Err(_) => get_focused_workspace().output(),
	};
	Output::by_name(&output_name).expect(&format!(
		"Invalid value for MONITOR env var. Output not found: {}",
		output_name
	))
}
