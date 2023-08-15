use std::env;

use super::workspaces;

pub fn focused() -> String {
	match env::var("MONITOR") {
		Ok(val) => val as String,
		Err(_e) => workspaces::focused().output,
	}
}
