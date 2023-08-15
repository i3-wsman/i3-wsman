use std::process::Command;

pub fn update() {
	Command::new("polybar-msg")
		.arg("action")
		.arg("#i3wsm.hook.0")
		.output()
		.ok();
}
