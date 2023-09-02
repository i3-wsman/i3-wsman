use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::CONFIG;

pub enum NavigationDirection {
	Prev,
	Next,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum NavigationBehavior {
	Create,
	Loop,
	Stop,
}

impl FromStr for NavigationBehavior {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match Self::from_arg(s) {
			Ok(b) => Ok(b),
			Err(_) => {
				eprintln!("Warning: Invalid value '{}' for 'navigation.*.behavior' or 'navigation.prev_behavior'. Falling back to 'Create'.", s);
				Ok(NavigationBehavior::Create)
			}
		}
	}
}

impl NavigationBehavior {
	pub fn from_arg(arg: &str) -> Result<Self, ()> {
		match arg.to_lowercase().as_ref() {
			"create" => Ok(NavigationBehavior::Create),
			"loop" => Ok(NavigationBehavior::Loop),
			"stop" => Ok(NavigationBehavior::Stop),
			_ => Err(()),
		}
	}

	pub fn from_argv(
		argv: &mut Vec<String>,
		nav_type: Option<NavigationDirection>,
	) -> Result<Self, ()> {
		match Self::from_arg(argv.get(0).unwrap_or(&"".to_string())) {
			Ok(b) => {
				argv.remove(0);
				Ok(b)
			}
			Err(_) => match nav_type {
				Some(n) => match n {
					NavigationDirection::Prev => Ok(CONFIG.navigation.prev.behavior.clone()),
					NavigationDirection::Next => Ok(CONFIG.navigation.next.behavior.clone()),
				},
				None => Ok(Self::Create),
			},
		}
	}

	#[allow(dead_code)]
	pub fn to_string(&self) -> String {
		match self {
			NavigationBehavior::Create => "create".to_string(),
			NavigationBehavior::Loop => "loop".to_string(),
			NavigationBehavior::Stop => "stop".to_string(),
		}
	}
}
