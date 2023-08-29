use serde::{Deserialize, Serialize};

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
