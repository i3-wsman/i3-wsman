use std::fmt;

use super::{Actions, Label};

#[derive(Debug)]
pub struct Format {
	pub format: String,
	pub label: Label,
	pub prefix: Option<Label>,
	pub suffix: Option<Label>,

	pub font: Option<u64>,
	pub actions: Option<Actions>,

	pub foreground: Option<String>,
	pub background: Option<String>,
	pub underline: Option<String>,
	pub overline: Option<String>,
	pub padding: Option<String>,
	pub margin: Option<String>,
}

impl fmt::Display for Format {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let label = self.format.clone();

		let mut prefix = "".to_owned();
		let mut suffix = "".to_owned();

		if let Some(font) = self.font {
			prefix = prefix + format!("%{{T{}}}", font).as_ref();
			suffix = "%{T-}".to_string() + suffix.as_ref();
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

		let before_label = match self.prefix.clone() {
			Some(l) => format!("{}", l),
			None => "".to_string(),
		};

		let after_label = match self.suffix.clone() {
			Some(l) => format!("{}", l),
			None => "".to_string(),
		};

		let label = format!(
			"{}{}{}{}{}",
			before_label,
			prefix,
			label.replace("<label>", self.label.to_string().as_ref()),
			suffix,
			after_label,
		);

		let container = Label {
			label,
			foreground: self.foreground.clone(),
			background: self.background.clone(),
			underline: self.underline.clone(),
			overline: self.overline.clone(),
			padding: self.padding.clone(),
			margin: self.margin.clone(),
		};

		write!(f, "{}", container)
	}
}
