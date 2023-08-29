use serde::{Deserialize, Serialize};

mod goto_behavior;
mod navigation_behavior;

pub use goto_behavior::GotoBehavior;
pub use navigation_behavior::NavigationBehavior;

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
