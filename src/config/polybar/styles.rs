use crate::polybar::{Format, Label};

use super::Config;

lazy_static! {
	static ref QUOTES: [char; 2] = ['"', '\''];
}

#[derive(Debug)]
struct Styles {
	pub body: String,
	pub font: Option<u64>,
	pub foreground: Option<String>,
	pub background: Option<String>,
	pub underline: Option<String>,
	pub overline: Option<String>,
	pub padding: Option<String>,
	pub margin: Option<String>,
}

impl Config {
	fn get_optional_int(&self, section: &str, key: &str, fallback: Option<&str>) -> Option<u64> {
		match (&self.root).getuint(section, key) {
			Ok(v) => match v {
				Some(v) => Some(v),
				None => match fallback {
					Some(v) => self.get_optional_int(section, v, None),
					None => None,
				},
			},
			Err(_) => {
				self.complain(section, key, &1);
				None
			}
		}
	}

	fn get_optional(&self, section: &str, key: &str, fallback: Option<&str>) -> Option<String> {
		match (&self.root).get(section, key) {
			Some(v) => Some(v.trim_matches(&QUOTES[..]).to_owned()),
			None => match fallback {
				Some(v) => self.get_optional(section, v, None),
				None => None,
			},
		}
	}

	fn get_key(&self, section: &str, key: &str, fallback: Option<&str>) -> String {
		match (&self.root).get(section, key) {
			Some(v) => v.trim_matches(&QUOTES[..]).to_owned(),
			None => match fallback {
				Some(v) => self.get_key(section, v, None),
				None => "".to_string(),
			},
		}
	}

	fn get_sub_styles(&self, section: &str, key_type: &str, key_state: &str) -> Styles {
		let target = |subkey: &str| format!("{}-{}-{}", key_type, key_state, subkey);
		let target_body = format!("{}-{}", key_type, key_state);

		Styles {
			body: self.get_key(section, &target_body, Some(key_type)),
			font: self.get_optional_int(section, &target("font"), None),
			foreground: self.get_optional(section, &target("foreground"), None),
			background: self.get_optional(section, &target("background"), None),
			underline: self.get_optional(section, &target("underline"), None),
			overline: self.get_optional(section, &target("overline"), None),
			padding: self.get_optional(section, &target("padding"), None),
			margin: self.get_optional(section, &target("margin"), None),
		}
	}

	fn get_styles(&self, section: &str, key_type: &str) -> Styles {
		let target = |subkey: &str| format!("{}-{}", key_type, subkey);

		Styles {
			body: self.get_key(section, key_type, None),
			font: self.get_optional_int(section, &target("font"), None),
			foreground: self.get_optional(section, &target("foreground"), None),
			background: self.get_optional(section, &target("background"), None),
			underline: self.get_optional(section, &target("underline"), None),
			overline: self.get_optional(section, &target("overline"), None),
			padding: self.get_optional(section, &target("padding"), None),
			margin: self.get_optional(section, &target("margin"), None),
		}
	}

	pub fn get_label(
		&self,
		section: &str,
		key_state: Option<String>,
		key_type: Option<String>,
	) -> Label {
		let key_type = match key_type {
			Some(key_type) => key_type,
			None => "label".to_string(),
		};

		let styles = match key_state {
			Some(key_state) => self.get_sub_styles(section, key_type.as_str(), key_state.as_str()),
			None => self.get_styles(section, key_type.as_str()),
		};

		Label {
			label: styles.body,
			actions: None,
			tokens: Default::default(),
			font: styles.font,
			foreground: styles.foreground,
			background: styles.background,
			underline: styles.underline,
			overline: styles.overline,
			padding: styles.padding,
			margin: styles.margin,
		}
	}

	pub fn get_format(&self, section: &str, key_state: Option<String>) -> Format {
		let container = self.get_label(section, key_state, Some("format".to_string()));
		// let fix_state = |key_state: Option<String>, fix: &str| match key_state {
		// 	Some(s) => Some(s.to_owned() + "-" + fix),
		// 	None => Some(fix.to_owned()),
		// };
		// @TODO: Implement "optional" labels
		// let prefix = self.get_label(section, fix_state(key_state, "prefix"), true);
		// let suffix = self.get_label(section, fix_state(key_state, "suffix"), true);

		Format {
			container,
			labels: Default::default(),
			prefix: None, // Some(prefix),
			suffix: None, // Some(suffix),
		}
	}
}
