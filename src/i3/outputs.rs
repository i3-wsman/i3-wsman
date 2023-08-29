use i3_ipc::reply;

use super::{get_matching_workspaces, get_outputs_from_i3};
use crate::common::constraint::{Constraint, Criteria};
use crate::state;
use crate::CONFIG;

pub static XROOT: &str = "xroot-0";

#[derive(Debug, Clone)]
pub struct Output {
	root: reply::Output,
}

#[allow(dead_code)]
impl Output {
	pub fn from_output(o: &reply::Output) -> Self {
		Self { root: o.to_owned() }
	}

	pub fn by_name(name: &str) -> Option<Self> {
		match get_outputs_from_i3().iter().find(|o| o.name == name) {
			Some(o) => Some(Self { root: o.to_owned() }),
			None => None,
		}
	}

	pub fn get_state_name(&self) -> String {
		match CONFIG.groups.unique_groups_on_outputs {
			true => self.root.name.to_owned(),
			false => XROOT.to_owned(),
		}
	}

	pub fn name(&self) -> String {
		self.root.name.to_owned()
	}

	pub fn active(&self) -> bool {
		self.root.active
	}

	pub fn primary(&self) -> bool {
		self.root.primary
	}

	pub fn current_workspace(&self) -> Option<String> {
		self.root.current_workspace.to_owned()
	}

	pub fn rect(&self) -> reply::Rect {
		self.root.rect.to_owned()
	}

	pub fn showing_all(&self) -> bool {
		let state = state::get();
		match state.groups.get(&self.get_state_name()) {
			Some(groups) => groups.len() == 0,
			None => true,
		}
	}

	pub fn groups(&self) -> Vec<String> {
		let mut constraint = Criteria::new();

		if CONFIG.groups.unique_groups_on_outputs {
			constraint.add(Constraint::Output);
			constraint.output = Some(self.clone());
		}

		let mut groups: Vec<String> = get_matching_workspaces(constraint)
			.iter()
			.map(|ws| ws.group())
			.collect();

		groups.sort();
		groups.dedup();
		groups
	}

	pub fn active_groups(&self) -> Vec<String> {
		let state = state::get();
		match state.groups.get(&self.get_state_name()) {
			Some(/* mut */ groups) => {
				// Is this needed? I guess we'll see!
				// groups.extend(CONFIG.groups.always_visible.to_owned());
				// groups.sort();
				// groups.dedup();
				groups.to_owned()
			}
			None => vec![],
		}
	}

	pub fn has_active_group(&self, group: String) -> bool {
		match self.showing_all() {
			true => true,
			false => {
				let active_groups = self.active_groups();
				active_groups.contains(&group)
			}
		}
	}
}
