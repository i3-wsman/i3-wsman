use std::collections::HashMap;
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

	pub tokens: HashMap<String, String>,

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

		if let Some(margin) = self.margin.clone() {
			let margin = match margin.parse::<usize>() {
				Ok(i) => " ".repeat(i),
				Err(_) => format!("%{{O{}}}", margin),
			};
			prefix = prefix + margin.as_ref();
			suffix = margin + suffix.as_ref();
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

		if let Some(padding) = self.padding.clone() {
			let padding = match padding.parse::<usize>() {
				Ok(i) => " ".repeat(i),
				Err(_) => format!("%{{O{}}}", padding),
			};
			prefix = prefix + padding.as_ref();
			suffix = padding + suffix.as_ref();
		}

		let mut label = self.label.clone();
		for (token, value) in &self.tokens {
			let token_tag: String = format!("%{}%", token);

			//let token_body = labels
			//	.iter()
			//	.map(|l| l.to_string())
			//	.reduce(|acc, l| acc + l.as_str())
			//	.unwrap_or("".to_owned());

			label = label.replace(token_tag.as_str(), value.as_str());
		}

		write!(f, "{}{}{}", prefix, label, suffix)
	}
}
