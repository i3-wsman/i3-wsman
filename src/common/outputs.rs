use std::env;
use std::process::Command;

use super::workspaces;

pub fn available() -> Vec<String> {
	let raw = Command::new("polybar")
		.arg("--list-monitors")
		.output()
		.unwrap();
	let stdout = String::from_utf8_lossy(&raw.stdout);
	stdout
		.lines()
		.map(|line| line.split(":").next().unwrap().to_string())
		.collect()
}

pub fn focused() -> String {
	match env::var("MONITOR") {
		Ok(val) => val as String,
		Err(_e) => workspaces::focused().output,
	}
}
