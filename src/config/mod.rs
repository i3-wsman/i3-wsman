pub mod global;
pub mod polybar;

use std::path::PathBuf;
use xdg;

static FILENAME: &str = "i3-wsman";

fn get_path(cfg_type: &str, extension: &str) -> PathBuf {
	let xdg_dirs = xdg::BaseDirectories::with_prefix(cfg_type).unwrap();

	xdg_dirs
		.find_config_file(FILENAME.to_owned() + "." + extension.as_ref())
		.unwrap_or(xdg_dirs.find_config_file(FILENAME).unwrap_or_else(|| {
			let mut path = xdg_dirs.get_config_home();
			path.push(FILENAME);
			path.set_extension(extension);
			path
		}))
}
