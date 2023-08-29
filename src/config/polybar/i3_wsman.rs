use super::Config;

impl Config {
	pub fn pin_workspaces(&self) -> bool {
		let default = true;

		let (section, key) = ("i3-wsman", "pin-workspaces");
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
}
