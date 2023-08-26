use i3_ipc::{
	event::{Event, Subscribe},
	I3Stream,
};

use crate::common::polybar;

pub fn exec(_: Vec<String>) {
	let mut i3 = I3Stream::conn_sub(&[Subscribe::Window, Subscribe::Workspace]).unwrap();
	for e in i3.listen() {
		match e.unwrap() {
			Event::Workspace(_) => polybar::update(),
			Event::Window(_) => polybar::update(),
			Event::Output(_) => polybar::update(),
			Event::Mode(_) => polybar::update(),
			Event::BarConfig(_) => polybar::update(),
			_ => {}
		}
	}
	exec(vec![]);
}
