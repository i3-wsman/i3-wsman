use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use toml;

use super::get_path;

mod create;
mod focus;
mod groups;
mod navigation;
mod startup;

pub use create::Create;
pub use focus::Focus;
pub use groups::Groups;
pub use navigation::{GotoBehavior, Navigation, NavigationBehavior, NavigationDirection};
pub use startup::Startup;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(default)]
pub struct Config {
	pub startup: Startup,
	pub groups: Groups,
	pub focus: Focus,
	pub create: Create,
	pub navigation: Navigation,
}

pub fn load_cfg() -> Config {
	let config_path = get_path("i3", "toml");

	let mut config: Config = Default::default();

	if !config_path.exists() {
		eprintln!("Customize i3-wsman in {}", config_path.to_str().unwrap());
		return config;
	}

	let mut contents = String::new();
	if let Ok(mut file) = File::open(config_path.clone()) {
		if let Err(_) = file.read_to_string(&mut contents) {
			eprintln!("Customize i3-wsman in {}", config_path.to_str().unwrap());
			return config;
		}
	} else {
		eprintln!("Customize i3-wsman in {}", config_path.to_str().unwrap());
		return config;
	}

	let toml_cfg = toml::from_str::<Config>(&contents);
	if let Ok(real_config) = toml_cfg {
		config = real_config;
	} else {
		eprintln!(
			"Error reading config file: {}\n\r{}",
			config_path.to_str().unwrap(),
			toml_cfg.unwrap_err()
		);
	}

	config
}
