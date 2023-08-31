use crate::polybar;

pub fn exec(_: Vec<String>) {
	polybar::update();
}
