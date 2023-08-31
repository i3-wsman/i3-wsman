use crate::{groups, polybar};

pub fn exec(_: Vec<String>) {
	groups::toggle_show_hidden();
	polybar::update();
}
