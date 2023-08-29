use serde::{Deserialize, Serialize};

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
