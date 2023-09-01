use super::Config;

impl Config {
	fn get_bool_or_default(&self, section: &str, key: &str, default: bool) -> bool {
		match (&self.root).getboolcoerce(section, key) {
			Ok(v) => match v {
				Some(v) => v,
				None => default,
			},
			Err(_) => {
				self.complain(section, key, &default);
				default
			}
		}
	}

	pub fn pin_workspaces(&self) -> bool {
		self.get_bool_or_default("i3-wsman", "pin-workspaces", true)
	}

	pub fn show_urgent(&self) -> bool {
		self.get_bool_or_default("i3-wsman", "show-urgent", false)
	}

	pub fn show_hidden_urgent(&self) -> bool {
		self.get_bool_or_default("i3-wsman", "show-hidden-urgent", true)
	}

	pub fn enable_click(&self) -> bool {
		self.get_bool_or_default("i3-wsman", "enable-click", true)
	}

	pub fn enable_scroll(&self) -> bool {
		self.get_bool_or_default("i3-wsman", "enable-scroll", true)
	}

	pub fn wrapping_scroll(&self) -> bool {
		self.get_bool_or_default("i3-wsman", "wrapping-scroll", true)
	}

	pub fn reverse_scroll(&self) -> bool {
		self.get_bool_or_default("i3-wsman", "reverse-scroll", true)
	}
}
