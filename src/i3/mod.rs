use i3_ipc::reply;
use std::env;

use crate::common::constraint::{Constraint, Criteria};
use crate::I3;

pub mod outputs;
pub mod workspace;

pub use outputs::Output;
pub use workspace::Workspace;

lazy_static! {
	static ref CURRENT_OUTPUT: Output = init_current_output();
}

// Workspaces
fn get_workspaces_from_i3() -> Vec<reply::Workspace> {
	I3.get_workspaces().unwrap()
}

pub fn get_workspaces() -> Vec<Workspace> {
	get_workspaces_from_i3()
		.iter()
		.map(|ws| Workspace::from_ws(ws))
		.collect()
}

pub fn get_focused_workspace() -> Workspace {
	let mut constraint = Criteria::new();
	constraint.add(Constraint::Focused);
	*get_matching_workspaces(constraint).first().unwrap()
}

pub fn get_matching_workspaces(criteria: Criteria) -> Vec<Workspace> {
	if criteria.contains(Constraint::None) {
		return get_workspaces();
	}

	get_workspaces_from_i3()
		.iter()
		.map(|ws| Workspace::from_ws(ws))
		.filter(|ws| !ws.matches(criteria.clone()))
		.collect()
}

// Outputs
fn get_outputs_from_i3() -> Vec<reply::Output> {
	I3.get_outputs()
		.unwrap()
		.iter()
		.filter(|o| o.current_workspace.is_some())
		.map(|o| o.to_owned())
		.collect()
}

fn init_current_output() -> Output {
	let output_name = match env::var("MONITOR") {
		Ok(val) => val as String,
		Err(_) => get_focused_workspace().output(),
	};
	Output::by_name(&output_name).expect(&format!(
		"Invalid value for MONITOR env var. Output not found: {}",
		output_name
	))
}

pub fn get_outputs() -> Vec<Output> {
	I3.get_outputs()
		.unwrap()
		.iter()
		.filter(|o| o.current_workspace.is_some())
		.map(|o| Output::from_output(o))
		.collect()
}

pub fn get_current_output() -> Output {
	CURRENT_OUTPUT.clone()
}
