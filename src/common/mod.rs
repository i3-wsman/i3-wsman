extern crate i3_ipc;
extern crate serde_json;

pub mod constraint;
pub mod moves;
pub mod name;
pub mod neighbor;
pub mod outputs;
pub mod polybar;
pub mod workspaces;

use std::env;
use std::path::PathBuf;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
	Right,
	Left
}

pub fn this_command_abs() -> String {
	env::current_exe().unwrap().to_str().unwrap().to_string()
}

pub fn this_command() -> String {
	let path: PathBuf = env::current_exe().unwrap();
	path.file_name().unwrap().to_str().unwrap().to_string()
}

