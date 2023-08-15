pub mod defaults;

use std::fmt;
use std::process::Command;

pub fn update() {
	Command::new("polybar-msg").arg("action").arg("#i3wsm.hook.0").output().ok();
	Command::new("polybar-msg").arg("action").arg("#i3wsm-groups.hook.0").output().ok();
	Command::new("polybar-msg").arg("action").arg("#i3wsm-workspaces.hook.0").output().ok();
	Command::new("polybar-msg").arg("action").arg("#i3wsm-toggle-hidden.hook.0").output().ok();
}

pub const LEFT_CLICK: &str = "left_click";
pub const MIDDLE_CLICK: &str = "middle_click";
pub const RIGHT_CLICK: &str = "right_click";

#[derive(Debug, Clone)]
pub struct Actions {
	pub left_click: Option<String>,
	pub middle_click: Option<String>,
	pub right_click: Option<String>,
}

#[derive(Debug)]
pub struct Label {
	pub label: String,
	pub padding: u8,
	pub margin: u8,
	pub font: Option<u8>,
	pub fg_color: Option<String>,
	pub bg_color: Option<String>,
	pub actions: Option<Actions>,
}

impl Label {
	pub fn new(label: &str, padding: u8, margin: u8) -> Self {
		Label {
			label: label.to_owned(),
			padding,
			margin,
			font: None,
			fg_color: None,
			bg_color: None,
			actions: None,
		}
	}

	pub fn set_colors(&mut self, fg_color: &str, bg_color: &str) {
		self.fg_color = Some(fg_color.to_owned());
		self.bg_color = Some(bg_color.to_owned());
	}

	pub fn set_action(&mut self, click_type: &str, cmd: &str) {
		let cur_actions = self.actions.clone();

		let mut actions = if let Some(a) = cur_actions { a } else {
			Actions {
				left_click: None,
				middle_click: None,
				right_click: None,
			}
		};

		match click_type {
			LEFT_CLICK => { actions.left_click = Some(cmd.to_owned()) },
			MIDDLE_CLICK => { actions.middle_click = Some(cmd.to_owned()) },
			RIGHT_CLICK => { actions.right_click = Some(cmd.to_owned()) },
			_ => {}
		};

		self.actions = Some(actions);
	}

	pub fn set_actions(&mut self, l_action: Option<String>, r_action: Option<String>, m_action: Option<String>) {
		self.actions = Some(Actions {
			left_click: l_action,
			middle_click: m_action,
			right_click: r_action,
		});
	}
}

impl fmt::Display for Label {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let space = " ";
		let mut prefix = "".to_owned();
		let mut suffix = "".to_owned();

		if let Some(font) = self.font {
			prefix = prefix + format!("%{{T{}}}", font).as_ref();
			suffix = "%{T-}".to_string() + suffix.as_ref();
		}

		prefix = prefix + space.repeat(self.margin.into()).as_ref();
		suffix = space.repeat(self.margin.into()) + suffix.as_ref();

		if let Some(fg_color) = self.fg_color.clone() {
			prefix = prefix + format!("%{{F#{}}}", fg_color).as_ref();
			suffix = "%{F-}".to_string() + suffix.as_ref();
		}

		if let Some(bg_color) = self.bg_color.clone() {
			prefix = prefix + format!("%{{B#{}}}", bg_color).as_ref();
			suffix = "%{B-}".to_string() + suffix.as_ref();
		}

		if let Some(actions) = self.actions.clone() {
			if let Some(cmd) = actions.left_click {
				prefix = prefix + format!("%{{A1:{}:}}", cmd).as_ref();
				suffix = "%{A}".to_string() + suffix.as_ref();
			}
			if let Some(cmd) = actions.middle_click {
				prefix = prefix + format!("%{{A2:{}:}}", cmd).as_ref();
				suffix = "%{A}".to_string() + suffix.as_ref();
			}
			if let Some(cmd) = actions.right_click {
				prefix = prefix + format!("%{{A3:{}:}}", cmd).as_ref();
				suffix = "%{A}".to_string() + suffix.as_ref();
			}
		}

		prefix = prefix + space.repeat(self.padding.into()).as_ref();
		suffix = space.repeat(self.padding.into()) + suffix.as_ref();

		write!(f, "{}{}{}", prefix, self.label, suffix)
	}
}
