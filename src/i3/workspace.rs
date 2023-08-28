use i3_ipc::reply;

use crate::{
	common::{
		constraint::{Constraint, Criteria},
		Direction,
	},
	CONFIG,
};

use super::{get_current_output, get_matching_workspaces, get_workspaces_from_i3};

pub struct Workspace {
	root: reply::Workspace,
}

impl Workspace {
	pub fn from_ws(ws: &reply::Workspace) -> Self {
		Self {
			root: ws.to_owned(),
		}
	}

	pub fn by_num(n: i32) -> Option<Self> {
		let workspaces = get_workspaces_from_i3();
		match workspaces.iter().find(|ws| ws.num == n) {
			Some(ws) => Some(Self {
				root: ws.to_owned(),
			}),
			None => None,
		}
	}

	pub fn by_name(name: &str) -> Option<Self> {
		let workspaces = get_workspaces_from_i3();
		match workspaces.iter().find(|ws| ws.name == name) {
			Some(ws) => Some(Self {
				root: ws.to_owned(),
			}),
			None => None,
		}
	}

	pub fn id(&self) -> usize {
		self.root.id
	}

	pub fn num(&self) -> i32 {
		self.root.num
	}

	pub fn full_name(&self) -> String {
		self.root.name
	}

	pub fn name(&self) -> String {
		let parts: Vec<&str> = self.root.name.split(':').collect();
		if parts.len() > 2 {
			parts[2].to_owned()
		} else {
			"".to_owned()
		}
	}

	pub fn group(&self) -> String {
		let parts: Vec<&str> = self.root.name.split(':').collect();
		if parts.len() > 1 {
			parts[1].to_owned()
		} else {
			"".to_owned()
		}
	}

	pub fn visible(&self) -> bool {
		self.root.visible
	}

	pub fn focused(&self) -> bool {
		self.root.focused
	}

	pub fn urgent(&self) -> bool {
		self.root.urgent
	}

	pub fn rect(&self) -> reply::Rect {
		self.root.rect
	}

	pub fn output(&self) -> String {
		self.root.output
	}

	pub fn matches(&self, criteria: Criteria) -> bool {
		if criteria.contains(Constraint::Output) {
			match criteria.output {
				Some(output) => return output.name() == self.output(),
				None => {}
			}
		}

		if criteria.contains(Constraint::AllowUrgent) && self.urgent() {
			return true;
		}

		if criteria.contains(Constraint::Focused) {
			if !self.focused() {
				return false;
			}
		}

		if criteria.contains(Constraint::Visible) {
			if !self.visible() {
				return false;
			}
		} else if criteria.contains(Constraint::Hidden) {
			if self.visible() {
				return false;
			}
		}

		let output = match criteria.output {
			Some(o) => o,
			None => get_current_output(),
		};

		if output.showing_all() {
			return true;
		}

		let ws_group = self.group();
		if ws_group.len() == 0 {
			if criteria.contains(Constraint::NoGroup) || criteria.contains(Constraint::Group) {
				return !CONFIG.focus.hide_unassigned_workspaces;
			}
		}

		if criteria.contains(Constraint::Group) {
			let output = match criteria.output {
				Some(o) => o,
				None => get_current_output(),
			};
			return output.has_active_group(ws_group);
		}

		return true;
	}

	pub fn get_neighbor(
		&self,
		criteria: Criteria,
		direction: Option<Direction>,
	) -> Option<Workspace> {
		let neighbor = self.get_closest_neighbor(criteria, direction);

		match neighbor {
			Some(ws) => {
				if (self.num() - ws.num()).abs() == 1 {
					return Some(ws);
				}
				return None;
			}
			None => None,
		}
	}

	pub fn get_closest_neighbor(
		&self,
		criteria: Criteria,
		direction: Option<Direction>,
	) -> Option<Workspace> {
		let mut workspaces = get_matching_workspaces(criteria);

		match direction {
			Some(d) => match d {
				Direction::Right => workspaces
					.iter()
					.find(|ws| ws.num() > self.num())
					.map(|ws| *ws.to_owned()),
				Direction::Left => {
					workspaces.sort_by(|w1, w2| w2.num().cmp(&w1.num()));
					workspaces
						.iter()
						.find(|ws| ws.num() < self.num())
						.map(|ws| *ws.to_owned())
				}
			},
			None => {
				let mut closest_ws: Option<Workspace> = None;
				let mut closest_distance: i32 = i32::MAX;

				for workspace in workspaces.iter() {
					let distance = (self.num() - workspace.num()).abs();
					if distance < closest_distance {
						closest_distance = distance;
						closest_ws = Some(*workspace.clone());
					}
				}

				closest_ws
			}
		}
	}
}
