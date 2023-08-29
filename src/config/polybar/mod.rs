use configparser::ini::Ini;
use std::any::Any;
use std::default::Default;
use std::fmt::{self, Debug};
use std::path::PathBuf;

use super::get_path;

use crate::polybar::{Format, Label};

mod i3_wsman;

lazy_static! {
	static ref CONFIG_PATH: PathBuf = get_path("polybar", "ini");
}

fn load() -> Ini {
	let mut ini = Ini::new();

	ini.set_comment_symbols(&[';']);

	if !CONFIG_PATH.exists() {
		return ini;
	}

	if let Err(_) = ini.load(CONFIG_PATH.clone()) {
		return ini;
	}

	ini
}

#[derive(Debug)]
pub struct Config {
	root: Ini,
}

impl Default for Config {
	fn default() -> Self {
		Self { root: load() }
	}
}

#[derive(Debug)]
struct Styles {
	pub format: String,
	pub label: String,
	pub font: Option<u64>,
	pub foreground: Option<String>,
	pub background: Option<String>,
	pub underline: Option<String>,
	pub overline: Option<String>,
	pub padding: Option<String>,
	pub margin: Option<String>,
}

impl Config {
	pub fn new() -> Self {
		Default::default()
	}

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
			Some(v) => Some(v),
			None => match fallback {
				Some(v) => self.get_optional(section, v, None),
				None => None,
			},
		}
	}

	fn get_key(&self, section: &str, key: &str, fallback: Option<&str>) -> String {
		match (&self.root).get(section, key) {
			Some(v) => v,
			None => match fallback {
				Some(v) => self.get_key(section, v, None),
				None => "".to_string(),
			},
		}
	}

	fn get_sub_styles(&self, section: &str, key_type: &str, key_state: &str) -> Styles {
		let target = |subkey: &str| format!("{}-{}-{}", key_type, key_state, subkey);
		let fallback = |subkey: &str| format!("{}-{}", key_type, subkey);

		Styles {
			label: self.get_key(section, &target("label"), Some(&"label")),
			format: self.get_key(section, &target("format"), Some(&"format")),
			font: self.get_optional_int(section, &target("font"), Some(&fallback("font"))),
			foreground: self.get_optional(
				section,
				&target("foreground"),
				Some(&fallback("foreground")),
			),
			background: self.get_optional(
				section,
				&target("background"),
				Some(&fallback("background")),
			),
			underline: self.get_optional(
				section,
				&target("underline"),
				Some(&fallback("underline")),
			),
			overline: self.get_optional(section, &target("overline"), Some(&fallback("overline"))),
			padding: self.get_optional(section, &target("padding"), Some(&fallback("padding"))),
			margin: self.get_optional(section, &target("margin"), Some(&fallback("margin"))),
		}
	}

	fn get_styles(&self, section: &str, key_type: &str) -> Styles {
		let target = |subkey: &str| format!("{}-{}", key_type, subkey);

		Styles {
			label: self.get_key(section, "label", None),
			format: self.get_key(section, "format", None),
			font: self.get_optional_int(section, &target("font"), None),
			foreground: self.get_optional(section, &target("foreground"), None),
			background: self.get_optional(section, &target("background"), None),
			underline: self.get_optional(section, &target("underline"), None),
			overline: self.get_optional(section, &target("overline"), None),
			padding: self.get_optional(section, &target("padding"), None),
			margin: self.get_optional(section, &target("margin"), None),
		}
	}

	pub fn get_label(&self, section: &str, key_state: Option<&str>) -> Label {
		let styles = match key_state {
			Some(ks) => self.get_sub_styles(section, "label", ks),
			None => self.get_styles(section, "label"),
		};

		Label {
			label: styles.label,
			foreground: styles.foreground,
			background: styles.background,
			underline: styles.underline,
			overline: styles.overline,
			padding: styles.padding,
			margin: styles.margin,
		}
	}

	pub fn get_format(&self, section: &str, key_state: Option<&str>, label: Label) -> Format {
		let styles = match key_state {
			Some(ks) => self.get_sub_styles(section, "format", ks),
			None => self.get_styles(section, "format"),
		};

		Format {
			label,

			prefix: None,
			suffix: None,
			actions: None,

			format: styles.format,
			font: styles.font,
			foreground: styles.foreground,
			background: styles.background,
			underline: styles.underline,
			overline: styles.overline,
			padding: styles.padding,
			margin: styles.margin,
		}
	}

	fn complain<T: Any + Debug>(&self, section: &str, key: &str, default: &T) {
		eprintln!(
			"Polybar Error: Invalid value in {}",
			CONFIG_PATH.to_str().unwrap()
		);

		eprintln!("\t[{}]", section);
		eprintln!("\t...");
		let value = (&self.root).get(section, key).unwrap();
		let clean_val = value.trim();
		eprintln!("\t{} = {}", key, clean_val);

		let arrow = "-".repeat(key.len() + 3) + "^".repeat(clean_val.len()).as_ref();
		eprintln!("\t{}", arrow);

		eprintln!("\n\r\tDefaulting to: {:?}", default as &dyn Any);
	}
}

impl fmt::Display for Config {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}",
			serde_json::to_string_pretty(&self.root.get_map().unwrap()).unwrap()
		)
	}
}
