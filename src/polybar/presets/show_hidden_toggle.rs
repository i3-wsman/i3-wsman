use crate::{common::this_command_abs, POLYBAR_CFG};

use super::{Actions, Format, Label};

#[derive(Clone)]
pub enum ToggleMode {
	On,
	Off,
	Disabled,
}

pub fn get_label(mode: ToggleMode) -> Label {
	match mode {
		ToggleMode::On => Label {
			label: "Turn Off".to_string(),
			foreground: None,
			background: None,
			underline: None,
			overline: None,
			padding: None,
			margin: None,
		},
		ToggleMode::Off => Label {
			label: "Turn On".to_string(),
			foreground: None,
			background: None,
			underline: None,
			overline: None,
			padding: None,
			margin: None,
		},
		ToggleMode::Disabled => Label {
			label: "Disabled".to_string(),
			foreground: None,
			background: None,
			underline: None,
			overline: None,
			padding: None,
			margin: None,
		},
	}
}

pub fn generate(mode: ToggleMode) -> Format {
	let label = get_label(mode.clone());

	let actions = Actions {
		left_click: Some(this_command_abs() + " polybar toggle"),
		middle_click: None,
		right_click: None,
	};

	match mode {
		ToggleMode::On => Format {
			format: POLYBAR_CFG.show_hidden_toggle.format_on.to_owned(),
			label,
			prefix: None,
			suffix: None,

			format_font: None,
			actions: Some(actions),

			foreground: None,
			background: None,
			underline: None,
			overline: None,
			padding: None,
			margin: None,
		},
		ToggleMode::Off => Format {
			format: POLYBAR_CFG.show_hidden_toggle.format_off.to_owned(),
			label,
			prefix: None,
			suffix: None,

			format_font: None,
			actions: Some(actions),

			foreground: None,
			background: None,
			underline: None,
			overline: None,
			padding: None,
			margin: None,
		},
		ToggleMode::Disabled => Format {
			format: POLYBAR_CFG.show_hidden_toggle.format_disabled.to_owned(),
			label,
			prefix: None,
			suffix: None,

			format_font: None,
			actions: Some(actions),

			foreground: None,
			background: None,
			underline: None,
			overline: None,
			padding: None,
			margin: None,
		},
	}
}
