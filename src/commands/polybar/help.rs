use super::{ this_command, this_command_abs, CMD };

pub fn exec(_: Vec<String>) {
	println!("{} {}", this_command(), CMD.as_str());
	println!("    The i3 Workspace Manager Polybar module");
	println!("    To use, add the following to your polybar config.ini:\n\r");
	println!("      [module/i3wsm]");
	println!("      type = custom/ipc");
	println!("      hook-0 = {} polybar", this_command_abs());
	println!("      initial = 1\n\r");
	println!("    Or, use modules individually:\n\r");
	println!("      [module/i3wsm-groups]");
	println!("      type = custom/ipc");
	println!("      hook-0 = {} polybar module-groups", this_command_abs());
	println!("      initial = 1");
	println!("      format = <label>");
	println!("      format-font = 3\n\r");
	println!("      [module/i3wsm-toggle-hidden]");
	println!("      type = custom/ipc");
	println!("      hook-0 = {} polybar module-show-hidden", this_command_abs());
	println!("      initial = 1");
	println!("      format = <label>");
	println!("      format-font = 3\n\r");
	println!("      [module/i3wsm-workspaces]");
	println!("      type = custom/ipc");
	println!("      hook-0 = {} polybar module-workspaces", this_command_abs());
	println!("      initial = 1");
	println!("      format = <label>");
	println!("      format-font = 3\n\r");
}
