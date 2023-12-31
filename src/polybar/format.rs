use std::collections::HashMap;
use std::fmt;

use super::Label;

#[derive(Debug)]
pub struct Format {
	pub container: Label,
	pub labels: HashMap<String, Vec<Label>>,
	pub prefix: Option<Label>,
	pub suffix: Option<Label>,
}

pub static WILDCARD: &str = "*";

impl fmt::Display for Format {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut label = self.container.label.clone();

		for (state, labels) in &self.labels {
			let label_tag: String = if state == WILDCARD {
				"<label>".to_owned()
			} else {
				"<label-".to_string() + state.as_ref() + ">"
			};

			let label_body = labels
				.iter()
				.map(|l| l.to_string())
				.reduce(|acc, l| acc + l.as_str())
				.unwrap_or("".to_owned());

			label = label.replace(label_tag.as_str(), label_body.as_str());
		}

		let before_label = match self.prefix.clone() {
			Some(l) => format!("{}", l),
			None => "".to_string(),
		};

		let after_label = match self.suffix.clone() {
			Some(l) => format!("{}", l),
			None => "".to_string(),
		};

		let mut container = self.container.clone();
		container.label = format!("{}{}{}", before_label, label, after_label,);

		write!(f, "{}", container)
	}
}
