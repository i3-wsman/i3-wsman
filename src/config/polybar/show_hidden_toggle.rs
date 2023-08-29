use serde::{Deserialize, Serialize};

pub struct FormatConfig {
	pub format: String,
	pub format_font: String,
	pub format_foreground: String,
	pub format_background: String,
	pub format_underline: String,
	pub format_overline: String,
	pub format_padding: String,
	pub format_margin: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default, rename_all = "kebab-case")]
pub struct ShowHiddenToggle {
	pub format: String,
	// pub format_font = 3
	// pub format_foreground =
	// pub format_background =
	// pub format_underline =
	// pub format_overline =
	// pub format_padding =
	// pub format_margin =
	#[serde(alias = "format_on", alias = "format")]
	pub format_on: String,
	// pub format_on_font =
	// pub format_on_foreground =
	// pub format_on_background =
	// pub format_on_underline =
	// pub format_on_overline =
	// pub format_on_padding =
	// pub format_on_margin =
	#[serde(alias = "format_off", alias = "format")]
	pub format_off: String,
	// pub format_off_font =
	// pub format_off_foreground =
	// pub format_off_background =
	// pub format_off_underline =
	// pub format_off_overline =
	// pub format_off_padding =
	// pub format_off_margin =
	#[serde(alias = "format_disabled", alias = "format")]
	pub format_disabled: String,
	// pub format_disabled_font = 3
	// pub format_disabled_foreground =
	// pub format_disabled_background =
	// pub format_disabled_underline =
	// pub format_disabled_overline =
	// pub format_disabled_padding =
	// pub format_disabled_margin =

	// pub label_on =
	// pub label_on_foreground =
	// pub label_on_background =
	// pub label_on_underline =
	// pub label_on_overline =
	// pub label_on_padding =
	// pub label_on_margin =

	// pub label_off =
	// pub label_off_foreground =
	// pub label_off_background =
	// pub label_off_underline =
	// pub label_off_overline =
	// pub label_off_padding =
	// pub label_off_margin =

	// pub label_disabled =
	// pub label_disabled_foreground =
	// pub label_disabled_background =
	// pub label_disabled_underline =
	// pub label_disabled_overline =
	// pub label_disabled_padding =
	// pub label_disabled_margin =
}

impl Default for ShowHiddenToggle {
	fn default() -> Self {
		let unset_value = "%%UNSET%%";

		Self {
			format: "<label>".to_string(),
			format_on: unset_value.to_string(),
			format_off: unset_value.to_string(),
			format_disabled: unset_value.to_string(),
		}
	}
}
