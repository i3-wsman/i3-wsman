use i3_ipc::{
	event::{Event, Subscribe},
	I3Stream,
};

use crate::polybar;

fn update() {
	polybar::update();
}

pub fn exec(_: Vec<String>) {
	let mut i3 = I3Stream::conn_sub(&[Subscribe::Window, Subscribe::Workspace]).unwrap();
	for e in i3.listen() {
		match e.unwrap() {
			Event::Workspace(_) => update(),
			Event::Window(_) => update(),
			Event::Output(_) => update(),
			Event::Mode(_) => update(),
			Event::BarConfig(_) => update(),
			_ => {}
		}
	}
	exec(vec![]);
}
