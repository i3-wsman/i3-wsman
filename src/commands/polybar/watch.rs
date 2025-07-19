use i3_ipc::{
	event::{Event, Subscribe},
	I3Stream,
};

use crate::polybar;

fn update() {
	polybar::update();
}

pub fn exec(_: Vec<String>) {
	loop {
		let mut i3 = match I3Stream::conn_sub(&[Subscribe::Window, Subscribe::Workspace]) {
			Ok(stream) => stream,
			Err(err) => {
				eprintln!("Failed to connect to i3 IPC: {}", err);
				continue;
			}
		};

		for e in i3.listen() {
			match e {
				Ok(Event::Workspace(_))
				| Ok(Event::Window(_))
				| Ok(Event::Output(_))
				| Ok(Event::Mode(_))
				| Ok(Event::BarConfig(_)) => update(),
				Ok(_) => {}
				Err(err) => {
					eprintln!("i3 IPC error: {}", err);
					break;
				}
			}
		}
	}
}
