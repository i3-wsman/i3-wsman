use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use toml;

use super::get_path;
use crate::commands::goto::GotoBehavior;
use crate::groups::GroupSortMethod;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(default)]
pub struct Config {
	pub startup: Startup,
	pub groups: Groups,
	pub focus: Focus,
	pub create: Create,
	pub navigation: Navigation,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Startup {
	pub initial_workspace_group: String,
	pub active_workspace_groups: Vec<String>,
	pub show_hidden_workspaces: bool,
}

impl Default for Startup {
	fn default() -> Self {
		Self {
			initial_workspace_group: "".to_string(),
			active_workspace_groups: vec![],
			show_hidden_workspaces: false,
		}
	}
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Groups {
	pub default_groups: Vec<String>,
	pub always_visible: Vec<String>,
	pub sort_method: GroupSortMethod,
	pub sort_default_first: bool,
	pub unique_groups_on_outputs: bool,
}

impl Default for Groups {
	fn default() -> Self {
		Self {
			default_groups: vec![],
			always_visible: vec![],
			sort_method: GroupSortMethod::Alphabetical,
			sort_default_first: true,
			unique_groups_on_outputs: true,
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

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Focus {
	pub hide_unassigned_workspaces: bool,
	pub auto_focus_nearest_group: bool,
}

impl Default for Focus {
	fn default() -> Self {
		Self {
			hide_unassigned_workspaces: false,
			auto_focus_nearest_group: false,
		}
	}
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Create {
	pub inherit_focused_group: bool,
	pub inherit_nearest_active_group: bool,
	pub default_group: String,
}

impl Default for Create {
	fn default() -> Self {
		Self {
			inherit_focused_group: true,
			inherit_nearest_active_group: true,
			default_group: "".to_string(),
		}
	}
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Navigation {
	pub allow_urgent: bool,
	pub next: NavigationNext,
	pub prev: NavigationPrev,
	pub goto: Goto,
}

impl Default for Navigation {
	fn default() -> Self {
		Self {
			allow_urgent: true,
			next: NavigationNext::default(),
			prev: NavigationPrev::default(),
			goto: Goto::default(),
		}
	}
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct NavigationPrev {
	pub behavior: NavigationBehavior,
}

impl Default for NavigationPrev {
	fn default() -> Self {
		Self {
			behavior: NavigationBehavior::Create,
		}
	}
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct NavigationNext {
	pub behavior: NavigationBehavior,
}

impl Default for NavigationNext {
	fn default() -> Self {
		Self {
			behavior: NavigationBehavior::Create,
		}
	}
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum NavigationBehavior {
	Create,
	Loop,
	Stop,
}

impl FromStr for NavigationBehavior {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_ref() {
			"create" => Ok(NavigationBehavior::Create),
			"loop" => Ok(NavigationBehavior::Loop),
			"stop" => Ok(NavigationBehavior::Stop),
			_ => {
				eprintln!("Warning: Invalid value '{}' for 'navigation.*.behavior' or 'navigation.prev_behavior'. Falling back to 'Create'.", s);
				Ok(NavigationBehavior::Create)
			}
		}
	}
}

impl NavigationBehavior {
	#[allow(dead_code)]
	pub fn to_string(&self) -> String {
		match self {
			NavigationBehavior::Create => "create".to_string(),
			NavigationBehavior::Loop => "loop".to_string(),
			NavigationBehavior::Stop => "stop".to_string(),
		}
	}
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Goto {
	pub behavior: GotoBehavior,
	pub ignore_focus: bool,
	pub restrict_to_output: bool,
}

impl Default for Goto {
	fn default() -> Self {
		Self {
			behavior: GotoBehavior::Stop,
			ignore_focus: false,
			restrict_to_output: true,
		}
	}
}

pub fn load_cfg() -> Config {
	let config_path = get_path("i3", "toml");

	let mut config: Config = Default::default();

	if !config_path.exists() {
		eprintln!("Customize i3-wsman in {}", config_path.to_str().unwrap());
		return config;
	}

	let mut contents = String::new();
	if let Ok(mut file) = File::open(config_path.clone()) {
		if let Err(_) = file.read_to_string(&mut contents) {
			eprintln!("Customize i3-wsman in {}", config_path.to_str().unwrap());
			return config;
		}
	} else {
		eprintln!("Customize i3-wsman in {}", config_path.to_str().unwrap());
		return config;
	}

	let toml_cfg = toml::from_str::<Config>(&contents);
	if let Ok(real_config) = toml_cfg {
		config = real_config;
	} else {
		eprintln!(
			"Error reading config file: {}\n\r{}",
			config_path.to_str().unwrap(),
			toml_cfg.unwrap_err()
		);
	}

	config
}
