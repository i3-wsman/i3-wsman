use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::CONFIG;

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum GotoBehavior {
	Create,
	Stop,
}

impl FromStr for GotoBehavior {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match Self::from_arg(s) {
			Ok(b) => Ok(b),
			Err(_) => {
				eprintln!("Warning: Invalid value '{}' for 'navigation.goto.behavior'. Falling back to 'Stop'.", s);
				Ok(GotoBehavior::Create)
			}
		}
	}
}

impl GotoBehavior {
	pub fn from_arg(arg: &str) -> Result<Self, ()> {
		match arg.to_lowercase().as_ref() {
			"create" => Ok(GotoBehavior::Create),
			"stop" => Ok(GotoBehavior::Stop),
			_ => Err(()),
		}
	}

	pub fn from_argv(argv: &mut Vec<String>) -> Result<Self, ()> {
		match Self::from_arg(argv.get(0).unwrap_or(&"".to_string())) {
			Ok(b) => {
				argv.remove(0);
				Ok(b)
			}
			Err(_) => Ok(CONFIG.navigation.goto.behavior.clone()),
		}
	}

	#[allow(dead_code)]
	pub fn to_string(&self) -> String {
		match self {
			GotoBehavior::Create => "create".to_string(),
			GotoBehavior::Stop => "stop".to_string(),
		}
	}
}
