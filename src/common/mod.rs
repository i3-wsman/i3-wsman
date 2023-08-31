use std::collections::HashSet;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;

pub mod constraint;
// pub mod moves;
// pub mod name;
// pub mod neighbor;
// pub mod polybar;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
	Right,
	Left,
}

impl FromStr for Direction {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_ref() {
			"left" => Ok(Direction::Left),
			"right" => Ok(Direction::Right),
			_ => {
				eprintln!(
					"Warning: Invalid direction '{}'. Falling back to 'Right'.",
					s
				);
				Ok(Direction::Right)
			}
		}
	}
}

impl Direction {
	#[allow(dead_code)]
	pub fn to_string(&self) -> String {
		match self {
			Direction::Left => "left".to_string(),
			Direction::Right => "right".to_string(),
		}
	}
}

pub fn this_command_abs() -> String {
	env::current_exe().unwrap().to_str().unwrap().to_string()
}

pub fn this_command() -> String {
	let path: PathBuf = env::current_exe().unwrap();
	path.file_name().unwrap().to_str().unwrap().to_string()
}

pub fn dedup_vec<T: std::hash::Hash + std::cmp::Eq + Clone>(vec: &mut Vec<T>) {
	let mut seen = HashSet::new();
	vec.retain(|e| seen.insert(e.clone()));
}
