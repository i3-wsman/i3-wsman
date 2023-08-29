use std::path::PathBuf;
use xdg;

use crate::POLYBAR_CFG;

use super::{this_command, this_command_abs, CMD};

fn get_polybar_cfg() -> PathBuf {
	let xdg_dirs = xdg::BaseDirectories::with_prefix("polybar").unwrap();

	xdg_dirs.find_config_file("config.ini").unwrap_or(
		xdg_dirs.find_config_file("config").unwrap_or_else(|| {
			let mut path = xdg_dirs.get_config_home();
			path.push("config.ini");
			path.set_extension("ini");
			path
		}),
	)
}

pub fn exec(_: Vec<String>) {
	println!("{} {}", this_command(), CMD.as_str());
	println!("    The i3 Workspace Manager Polybar module\n\r");
	//println!("    {} {} start [bar-name]", this_command(), CMD.as_str());
	//println!("        Starts polybar for all monitors and watches i3 for changes. ");
	//println!("        Note: Do not include a [bar-name] if you have configured them in your i3-wsman config\n\r");
	println!("    {} {} watch", this_command(), CMD.as_str());
	println!(
		"        Watches i3 for changes (not needed when using `{} start`)\n\r",
		CMD.as_str()
	);
	println!("    To setup polybar, add the following to your polybar config:\n\r");
	println!("      [module/i3wsm]");
	println!("      type = custom/ipc");
	println!("      hook-0 = {} {}", this_command_abs(), CMD.as_str());
	println!("      initial = 1\n\r");
	println!("    Or, use modules individually:\n\r");
	println!("      [module/i3wsm-groups]");
	println!("      type = custom/ipc");
	println!(
		"      hook-0 = {} {} module-groups",
		this_command_abs(),
		CMD.as_str()
	);
	println!("      initial = 1");
	println!("      format = <label>");
	println!("      format-font = 3\n\r");
	println!("      [module/i3wsm-toggle-hidden]");
	println!("      type = custom/ipc");
	println!(
		"      hook-0 = {} {} module-toggle-hidden",
		this_command_abs(),
		CMD.as_str()
	);
	println!("      initial = 1");
	println!("      format = <label>");
	println!("      format-font = 3\n\r");
	println!("      [module/i3wsm-workspaces]");
	println!("      type = custom/ipc");
	println!(
		"      hook-0 = {} {} module-workspaces",
		this_command_abs(),
		CMD.as_str()
	);
	println!("      initial = 1");
	println!("      format = <label>");
	println!("      format-font = 3\n\r");

	let cfg_path = get_polybar_cfg();
	println!("    Your polybar config file should be:");
	println!(
		"        {}\n\r",
		cfg_path.into_os_string().into_string().unwrap()
	);

	println!("{}", *POLYBAR_CFG);
}
