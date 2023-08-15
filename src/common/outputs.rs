use std::env;

use super::workspaces;

pub fn active() -> String {
	match env::var("MONITOR") {
		Ok(val) => val as String,
		Err(_e) => workspaces::active().output,
	}
}
