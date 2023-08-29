use serde::{Deserialize, Serialize};

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
