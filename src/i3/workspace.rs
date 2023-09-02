use i3_ipc::reply;
use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
	common::{
		constraint::{Constraint, Criteria},
		Direction,
	},
	i3, POLYBAR_CFG,
};

use super::{
	get_current_output, get_matching_workspaces, get_outputs, get_workspaces,
	get_workspaces_from_i3,
};

#[derive(Debug, Clone)]
pub struct Workspace {
	root: reply::Workspace,
}

pub fn parse_name(name: String) -> (String, String, String) {
	let parts: Vec<&str> = name.split(':').collect();
	(
		parts.get(0).unwrap_or(&"").to_string(),
		parts.get(1).unwrap_or(&"").to_string(),
		parts.get(2).unwrap_or(&"").to_string(),
	)
}

pub fn assemble_name(num: i32, group: String, name: String) -> String {
	let (group, name) = match (group.as_str(), name.as_str()) {
		("", "") => ("".to_string(), "".to_string()),
		(g, "") => (":".to_string() + g, "".to_string()),
		("", n) => (":".to_string(), ":".to_string() + n),
		(g, n) => (":".to_string() + g, ":".to_string() + n),
	};

	format!("{}{}{}", num, group, name)
}

fn get_workspace_from_i3_by_num(num: i32) -> Option<reply::Workspace> {
	let workspaces = get_workspaces_from_i3();
	match workspaces.iter().find(|ws| ws.num == num) {
		Some(ws) => Some(ws.to_owned()),
		None => None,
	}
}

fn get_workspace_from_i3_by_name(name: &str) -> Option<reply::Workspace> {
	let workspaces = get_workspaces_from_i3();
	match workspaces.iter().find(|ws| ws.name == name) {
		Some(ws) => Some(ws.to_owned()),
		None => None,
	}
}

impl Workspace {
	pub fn from_ws(ws: &reply::Workspace) -> Self {
		Self {
			root: ws.to_owned(),
		}
	}

	pub fn by_num(num: i32) -> Option<Self> {
		match get_workspace_from_i3_by_num(num) {
			Some(ws) => Some(Self {
				root: ws.to_owned(),
			}),
			None => None,
		}
	}

	#[allow(dead_code)]
	pub fn by_name(name: &str) -> Option<Self> {
		match get_workspace_from_i3_by_name(name) {
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
		self.root.name.clone()
	}

	pub fn name(&self) -> String {
		let (_, _, name) = parse_name(self.root.name.clone());
		name
	}

	pub fn group(&self) -> String {
		let (_, group, _) = parse_name(self.root.name.clone());
		group
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
		self.root.rect.clone()
	}

	pub fn output(&self) -> String {
		self.root.output.clone()
	}

	pub fn matches(&self, criteria: Criteria) -> bool {
		let always_show_urgent = POLYBAR_CFG.show_urgent();
		if always_show_urgent && criteria.contains(Constraint::AllowUrgent) && self.urgent() {
			return true;
		}

		if criteria.contains(Constraint::Output) {
			match criteria.output.clone() {
				Some(output) => {
					if output.name() != self.output() {
						return false;
					}
				}
				None => {}
			}
		}

		if criteria.contains(Constraint::Focused) && self.focused() {
			return true;
		}

		if !always_show_urgent && criteria.contains(Constraint::AllowUrgent) && self.urgent() {
			return true;
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

		//let output = match criteria.output.clone() {
		//	Some(o) => o,
		//	None => get_current_output(),
		//};

		// if output.showing_all() == true || groups::show_hidden_enabled() {
		// 	return true;
		// }

		if criteria.contains(Constraint::NoGroup) && self.group().len() == 0 {
			// || criteria.contains(Constraint::Group) {
			return true; // !CONFIG.focus.hide_unassigned_workspaces;
		}

		if criteria.contains(Constraint::Group) {
			let output = match criteria.output.clone() {
				Some(o) => o,
				None => get_current_output(),
			};
			return output.has_active_group(self.group());
		}

		return true;
	}

	pub fn get_neighbor(
		&self,
		criteria: Option<Criteria>,
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
		criteria: Option<Criteria>,
		direction: Option<Direction>,
	) -> Option<Workspace> {
		let mut workspaces = match criteria {
			Some(c) => get_matching_workspaces(c),
			None => get_workspaces(),
		};

		match direction {
			Some(d) => match d {
				Direction::Right => {
					workspaces.sort_by(|w1, w2| w1.num().cmp(&w2.num()));
					workspaces.iter().find(|ws| ws.num() > self.num()).cloned()
				}
				Direction::Left => {
					workspaces.sort_by(|w1, w2| w2.num().cmp(&w1.num()));
					workspaces.iter().find(|ws| ws.num() < self.num()).cloned()
				}
			},
			None => {
				let mut closest_ws: Option<Workspace> = None;
				let mut closest_distance: i32 = i32::MAX;

				for workspace in workspaces.iter() {
					let distance = (self.num() - workspace.num()).abs();
					if distance < closest_distance {
						closest_distance = distance;
						closest_ws = Some(workspace.clone());
					}
				}

				closest_ws
			}
		}
	}

	pub fn set_name(&mut self, new_name: String) {
		let cur_name = self.full_name();
		let new_name = assemble_name(self.num(), self.group(), new_name);
		i3::run_command(format!("rename workspace {} to {}", cur_name, new_name));
		self.root = get_workspace_from_i3_by_name(new_name.as_str()).unwrap();
	}

	pub fn set_group(&mut self, new_group: String) {
		let cur_name = self.full_name();
		let new_name = assemble_name(self.num(), new_group, self.name());
		i3::run_command(format!("rename workspace {} to {}", cur_name, new_name));
		self.root = get_workspace_from_i3_by_name(new_name.as_str()).unwrap();
	}

	pub fn reorder(&mut self, new_pos: i32) {
		if new_pos == self.num() {
			return;
		}

		let cur_name = self.full_name();
		let new_name = assemble_name(new_pos, self.group(), self.name());
		i3::run_command(format!("rename workspace {} to {}", cur_name, new_name));
		self.root = get_workspace_from_i3_by_name(new_name.as_str()).unwrap();
	}

	pub fn scoot(&mut self) {
		let mut outputs = get_outputs();
		let mut workspaces = get_workspaces();

		outputs.reverse();
		workspaces.reverse();

		let mut i = TryInto::<i32>::try_into(workspaces.len()).unwrap() + 1;
		for o in outputs {
			let workspaces = workspaces.clone();
			for mut ws in workspaces {
				if o.name() != ws.output() {
					continue;
				}

				ws.reorder(i);

				if ws.full_name() == self.full_name() {
					i = i - 1;
				}
				i = i - 1;
			}
		}
	}
}

impl Serialize for Workspace {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut state = serializer.serialize_struct("Workspace", 10)?;
		state.serialize_field("id", &self.id())?;
		state.serialize_field("num", &self.num())?;
		state.serialize_field("full_name", &self.full_name())?;
		state.serialize_field("name", &self.name())?;
		state.serialize_field("group", &self.group())?;
		state.serialize_field("visible", &self.visible())?;
		state.serialize_field("focused", &self.focused())?;
		state.serialize_field("urgent", &self.urgent())?;
		state.serialize_field("rect", &self.rect())?;
		state.serialize_field("output", &self.output())?;
		state.end()
	}
}
