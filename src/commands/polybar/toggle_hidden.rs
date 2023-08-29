use crate::{common::polybar, groups};

pub fn exec(_: Vec<String>) {
	groups::toggle_show_hidden();
	polybar::update();
}
