use std::fmt;

use super::Actions;

//pub enum Alignment {
//	Left,
//	Center,
//	Right
//}

#[derive(Debug, Default, Clone)]
pub struct Label {
	pub label: String,

	pub font: Option<u64>,
	pub actions: Option<Actions>,

	pub foreground: Option<String>,
	pub background: Option<String>,
	pub underline: Option<String>,
	pub overline: Option<String>,
	pub padding: Option<String>,
	pub margin: Option<String>,
	//
	// @TODO: Implement max/min len, ellipsis, alignment
	//pub is_format_container: bool, <= If true, ignore the below
	//pub maxlen: Option<u8>,
	//pub minlen: Option<u8>,
	//pub alignment: Option<Alignment>,
	//pub ellipsis: bool,
}

impl fmt::Display for Label {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut prefix = "".to_owned();
		let mut suffix = "".to_owned();

		if let Some(font) = self.font {
			prefix = prefix + format!("%{{T{}}}", font).as_ref();
			suffix = "%{T-}".to_string() + suffix.as_ref();
		}

		if let Some(padding) = self.padding.clone() {
			prefix = prefix + format!("%{{O{}}}", padding).as_ref();
			suffix = format!("%{{O{}}}", padding) + suffix.as_ref();
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

		if let Some(underline) = self.underline.clone() {
			prefix = prefix + format!("%{{u{}}}%{{+u}}", underline).as_ref();
			suffix = "%{-u}".to_string() + suffix.as_ref();
		}

		if let Some(overline) = self.overline.clone() {
			prefix = prefix + format!("%{{o{}}}%{{+o}}", overline).as_ref();
			suffix = "%{-o}".to_string() + suffix.as_ref();
		}

		if let Some(foreground) = self.foreground.clone() {
			prefix = prefix + format!("%{{F{}}}", foreground).as_ref();
			suffix = "%{F-}".to_string() + suffix.as_ref();
		}

		if let Some(background) = self.background.clone() {
			prefix = prefix + format!("%{{B{}}}", background).as_ref();
			suffix = "%{B-}".to_string() + suffix.as_ref();
		}

		if let Some(margin) = self.margin.clone() {
			prefix = prefix + format!("%{{O{}}}", margin).as_ref();
			suffix = format!("%{{O{}}}", margin) + suffix.as_ref();
		}

		write!(f, "{}{}{}", prefix, self.label, suffix)
	}
}
