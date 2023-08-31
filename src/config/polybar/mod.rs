use configparser::ini::Ini;
use std::any::Any;
use std::default::Default;
use std::fmt::{self, Debug};
use std::path::PathBuf;

use super::get_path;

mod i3_wsman;
mod styles;

lazy_static! {
	static ref CONFIG_PATH: PathBuf = get_path("polybar", "ini");
}

fn load() -> Ini {
	let mut ini = Ini::new();

	ini.set_comment_symbols(&[';']);

	if !CONFIG_PATH.exists() {
		return ini;
	}

	if let Err(_) = ini.load(CONFIG_PATH.clone()) {
		return ini;
	}

	ini
}

#[derive(Debug)]
pub struct Config {
	root: Ini,
}

impl Default for Config {
	fn default() -> Self {
		Self { root: load() }
	}
}

impl Config {
	pub fn new() -> Self {
		Default::default()
	}

	fn complain<T: Any + Debug>(&self, section: &str, key: &str, default: &T) {
		eprintln!(
			"Polybar Error: Invalid value in {}",
			CONFIG_PATH.to_str().unwrap()
		);

		eprintln!("\t[{}]", section);
		eprintln!("\t...");
		let value = (&self.root).get(section, key).unwrap();
		let clean_val = value.trim();
		eprintln!("\t{} = {}", key, clean_val);

		let arrow = "-".repeat(key.len() + 3) + "^".repeat(clean_val.len()).as_ref();
		eprintln!("\t{}", arrow);

		eprintln!("\n\r\tDefaulting to: {:?}", default as &dyn Any);
	}
}

impl fmt::Display for Config {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}",
			serde_json::to_string_pretty(&self.root.get_map().unwrap()).unwrap()
		)
	}
}
