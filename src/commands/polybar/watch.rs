use std::process::Command;
use std::sync::{
	atomic::{AtomicBool, Ordering},
	Mutex,
};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use i3_ipc::{
	event::{Event, Subscribe},
	I3Stream,
};
use once_cell::sync::Lazy;

use crate::{groups, i3, polybar};

static SHOULD_PROCEED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(true));
static PENDING_THREAD: Lazy<Mutex<Option<JoinHandle<()>>>> = Lazy::new(|| Mutex::new(None));

fn update() {
	polybar::update();
}

pub fn update_and_bg() {
	update();

	// Inform any currently sleeping thread that it should not proceed.
	SHOULD_PROCEED.store(false, Ordering::Relaxed);

	let handle = thread::spawn(|| {
		// This is the most recent thread, so it should proceed after sleeping.
		SHOULD_PROCEED.store(true, Ordering::Relaxed);
		thread::sleep(Duration::from_millis(50));

		// Check the flag to determine whether to call update_bg().
		if SHOULD_PROCEED.load(Ordering::Relaxed) {
			update_bg();
		}
	});

	// Update the handle of the current thread.
	let mut pending = PENDING_THREAD.lock().unwrap();
	if let Some(old_handle) = pending.take() {
		let _ = old_handle.join(); // Ensure the old thread completes (but it should just exit without calling update_bg() if flagged).
	}
	*pending = Some(handle);
}

fn update_bg() {
	let config = crate::config::global::load_cfg();

	let bgs = config.groups.backgrounds.clone();
	if bgs.len() == 0 {
		return;
	}

	let Some(cmd_name) = config.groups.background_cmd.clone() else {
		return;
	};

	let mut i = -1;
	let outputs = i3::get_outputs();
	for o in outputs {
		i = i + 1;

		let ws = i3::get_current_workspace_for_output(o.clone());

		let mut group = ws.group();
		if group.is_empty() {
			let mut groups = groups::active_for_output(Some(o));
			if groups.len() != 1 {
				continue;
			}
			group = groups.pop().unwrap();
		}

		if !bgs.contains_key(&group) {
			group = "default".to_string();
		}

		let Some(cfg_argv) = bgs.get(&group) else {
			continue;
		};

		let mut argv = match config.groups.background_screen_arg.clone() {
			Some(arg) => vec![arg.replace("{}", &i.to_string())],
			None => vec![],
		};

		argv.extend(cfg_argv.clone());

		println!("nitrogen {:?}", argv);

		Command::new(cmd_name.clone()).args(argv).spawn().ok();
	}
}

pub fn exec(_: Vec<String>) {
	let mut i3 = I3Stream::conn_sub(&[Subscribe::Window, Subscribe::Workspace]).unwrap();
	for e in i3.listen() {
		match e.unwrap() {
			Event::Workspace(_) => update_and_bg(),
			Event::Window(_) => update(),
			Event::Output(_) => update(),
			Event::Mode(_) => update(),
			Event::BarConfig(_) => update(),
			_ => {}
		}
	}
	exec(vec![]);
}
