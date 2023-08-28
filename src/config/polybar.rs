use configparser::ini::Ini;
use serde::{Deserialize, Deserializer, Serialize};

use super::get_path;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(default, rename_all = "kebab-case")]
pub struct Config {
	pub i3_wsman: I3wsmanConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default, rename_all = "kebab-case")]
pub struct I3wsmanConfig {
	#[serde(deserialize_with = "bool_from_string")]
	pub pin_workspaces: bool,
}

impl Default for I3wsmanConfig {
	fn default() -> Self {
		Self {
			pin_workspaces: true,
		}
	}
}

/// Deserialize bool from String with custom value mapping
/// https://github.com/serde-rs/serde/issues/1344#issuecomment-416042546
fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
	D: Deserializer<'de>,
{
	match String::deserialize(deserializer)?.to_lowercase().as_ref() {
		"true" => Ok(true),
		"false" => Ok(false),
		_ => Ok(false),
	}
}

pub fn load_cfg() -> Config {
	let config_path = get_path("polybar", "ini");

	if !config_path.exists() {
		return Default::default();
	}

	let mut ini = Ini::new();
	if let Err(_) = ini.load(config_path) {
		return Default::default();
	}

	// This is disgusting
	// Should I submit PR to configparser to add serde support?
	serde_json::from_str(&serde_json::to_string_pretty(ini.get_map_ref()).unwrap()).unwrap()
}
