use serde::{Deserialize, Serialize};

mod goto_behavior;
mod navigation_behavior;

pub use goto_behavior::GotoBehavior;
pub use navigation_behavior::{NavigationBehavior, NavigationDirection};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Navigation {
	pub allow_urgent: bool,
	pub next: NavigationCfg,
	pub prev: NavigationCfg,
	pub goto: Goto,
}

impl Default for Navigation {
	fn default() -> Self {
		Self {
			allow_urgent: true,
			next: NavigationCfg::default(),
			prev: NavigationCfg::default(),
			goto: Goto::default(),
		}
	}
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct NavigationCfg {
	pub behavior: NavigationBehavior,
}

impl Default for NavigationCfg {
	fn default() -> Self {
		Self {
			behavior: NavigationBehavior::Create,
		}
	}
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Goto {
	pub behavior: GotoBehavior,
	pub restrict_to_output: bool,
	pub include_hidden_groups: bool,
	pub honor_show_hidden: bool,
}

impl Default for Goto {
	fn default() -> Self {
		Self {
			behavior: GotoBehavior::Stop,
			restrict_to_output: true,
			include_hidden_groups: false,
			honor_show_hidden: false,
		}
	}
}
