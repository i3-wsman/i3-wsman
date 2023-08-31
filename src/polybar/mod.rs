use std::process::Command;

mod format;
mod label;
// pub mod presets;

pub use format::Format;
pub use label::Label;

#[derive(Debug, Default, Clone)]
pub struct Actions {
	pub left_click: Option<String>,
	pub middle_click: Option<String>,
	pub right_click: Option<String>,
}

pub fn update() {
	Command::new("polybar-msg")
		.arg("action")
		.arg("#i3wsm.hook.0")
		.output()
		.ok();
	Command::new("polybar-msg")
		.arg("action")
		.arg("#i3wsm-groups.hook.0")
		.output()
		.ok();
	Command::new("polybar-msg")
		.arg("action")
		.arg("#i3wsm-workspaces.hook.0")
		.output()
		.ok();
	Command::new("polybar-msg")
		.arg("action")
		.arg("#i3wsm-toggle-hidden.hook.0")
		.output()
		.ok();
}
