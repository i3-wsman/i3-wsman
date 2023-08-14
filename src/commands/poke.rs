use crate::common::{
	this_command,
	polybar_update,
};

pub fn exec(args: Vec<String>) {
	if args.contains(&"help".to_string()) {
		println!("{} poke-poly", this_command());
		println!("    Pokes polybar to update");
		return;
	}

	polybar_update();
}
