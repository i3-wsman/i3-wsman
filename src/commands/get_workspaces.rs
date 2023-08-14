use crate::common::{
	this_command,
	get_constraints,
	get_workspaces,
};

pub fn exec(args: Vec<String>) {
	if args.contains(&"help".to_string()) {
		println!("{} get-workspaces [...constraints]", this_command());
		println!("    Returns workspaces matching the constraints.");
		println!("    Constraints are optional. If none are provided, all workspaces are returned.\n\r");
		println!("    Constraints:");
		println!("      focused: Focused Workspace");
		println!("      visible: Visible Workspaces");
		println!("      hidden: Hidden Workspaces");
		println!("      group: Workspaces apart of the active Group");
		println!("      output: Workspaces on the output ");
		println!("      output=xyz: Workspaces on the output xyz");
		println!("");
		println!("    For instance, to get all hidden workspaces on the current monitor:");
		println!("        {} get-workspaces hidden output", this_command());
		return;
	}

	let constraints = get_constraints(args.to_owned());

	let workspaces = get_workspaces(constraints, false);
	let output = serde_json::to_string_pretty(&workspaces).unwrap();
	println!("{}", output);
}
