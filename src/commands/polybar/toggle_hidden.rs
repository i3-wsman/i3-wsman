use crate::common::{ groups, polybar };

pub fn exec(_: Vec<String>) {
	groups::toggle_hidden();
	polybar::update();
}
