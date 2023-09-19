use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

use crate::groups::GroupSortMethod;

pub type GroupBackgrounds = HashMap<String, Vec<String>>;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Groups {
	pub default_groups: Vec<String>,
	pub always_visible: Vec<String>,
	pub sort_method: GroupSortMethod,
	pub sort_default_first: bool,
	pub unique_groups_on_outputs: bool,
	pub background_cmd: Option<String>,
	pub background_screen_arg: Option<String>,
	pub backgrounds: GroupBackgrounds,
}

impl Default for Groups {
	fn default() -> Self {
		Self {
			default_groups: vec![],
			always_visible: vec![],
			sort_method: GroupSortMethod::Alphabetical,
			sort_default_first: true,
			unique_groups_on_outputs: true,
			background_cmd: None,
			background_screen_arg: None,
			backgrounds: Default::default(),
		}
	}
}

impl FromStr for GroupSortMethod {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_ref() {
			"alphabetical" => Ok(GroupSortMethod::Alphabetical),
			"preserveorder" => Ok(GroupSortMethod::PreserveOrder),
			_ => {
				eprintln!("Warning: Invalid value '{}' for 'groups.sort_method'. Falling back to 'Alphabetical'.", s);
				Ok(GroupSortMethod::Alphabetical)
			}
		}
	}
}

impl GroupSortMethod {
	#[allow(dead_code)]
	pub fn to_string(&self) -> String {
		match self {
			GroupSortMethod::Alphabetical => "alphabetical".to_string(),
			GroupSortMethod::PreserveOrder => "preserveorder".to_string(),
		}
	}
}
