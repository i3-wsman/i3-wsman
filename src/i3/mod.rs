use i3_ipc::{event::Subscribe, reply, Connect, I3Stream, I3};
use std::{env, io, path::Path};

use crate::{
	common::constraint::{Constraint, Criteria},
	state, CONFIG, POLYBAR_CFG,
};

pub mod outputs;
pub mod workspace;

pub use outputs::Output;
pub use workspace::Workspace;

pub fn connect() -> I3Stream {
	connect_result().unwrap()
}

pub fn conn_sub(events: &[Subscribe]) -> io::Result<I3Stream> {
	normalize_socket_env();
	match I3Stream::conn_sub(events) {
		Ok(i3) => Ok(i3),
		Err(err) => retry_without_i3sock(err, || I3Stream::conn_sub(events)),
	}
}

fn normalize_socket_env() {
	let Ok(i3sock) = env::var("I3SOCK") else {
		return;
	};

	if Path::new(&i3sock).exists() {
		return;
	}

	clear_i3sock(i3sock);
}

fn connect_result() -> io::Result<I3Stream> {
	normalize_socket_env();
	match I3::connect() {
		Ok(i3) => Ok(i3),
		Err(err) => retry_without_i3sock(err, I3::connect),
	}
}

fn retry_without_i3sock<T, F>(err: io::Error, retry: F) -> io::Result<T>
where
	F: FnOnce() -> io::Result<T>,
{
	let Ok(i3sock) = env::var("I3SOCK") else {
		return Err(err);
	};

	match err.kind() {
		io::ErrorKind::NotFound | io::ErrorKind::ConnectionRefused | io::ErrorKind::BrokenPipe => {
			clear_i3sock(i3sock);
			retry()
		}
		_ => Err(err),
	}
}

fn clear_i3sock(i3sock: String) {
	eprintln!("i3-wsman: ignoring stale I3SOCK ({})", i3sock);
	env::remove_var("I3SOCK");
}

pub fn run_command(payload: String) {
	let mut i3 = connect();
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
	let mut i3 = connect();
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
	let mut i3 = connect();
	i3.get_outputs()
		.unwrap()
		.iter()
		.filter(|o| o.current_workspace.is_some())
		.map(|o| o.to_owned())
		.collect()
}

pub fn get_outputs() -> Vec<Output> {
	let mut i3 = connect();
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
