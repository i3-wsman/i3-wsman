use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use toml;
use xdg;

use super::Config;

fn config_file() -> PathBuf {
	let xdg_dirs = xdg::BaseDirectories::with_prefix("i3").unwrap();

	xdg_dirs.find_config_file("i3-wsman.toml").unwrap_or(
		xdg_dirs.find_config_file("i3-wsman").unwrap_or_else(|| {
			let mut path = xdg_dirs.get_config_home();
			path.push("i3-wsman");
			path.set_extension("toml");
			path
		}),
	)
}

pub fn load_cfg() -> Config {
	let config_path = config_file();

	let mut config: Config = Default::default();

	if !config_path.exists() {
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
