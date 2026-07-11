use std::process::{Child, Command};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use i3_ipc::{
	event::{Event, Subscribe},
	I3Stream,
};
use once_cell::sync::Lazy;

use crate::{groups, i3, polybar};

const WATCH_EVENTS: [Subscribe; 5] = [
	Subscribe::Window,
	Subscribe::Workspace,
	Subscribe::Output,
	Subscribe::Mode,
	Subscribe::BarConfigUpdate,
];

enum BgRequest {
	Update,
}

#[derive(Clone, Copy)]
enum BgRunMode {
	Async,
	Blocking,
}

struct BgWorker {
	sender: mpsc::Sender<BgRequest>,
}

impl BgWorker {
	fn start() -> Self {
		let (sender, receiver) = mpsc::channel();
		thread::spawn(move || worker_loop(receiver));
		Self { sender }
	}

	fn request_update(&self) {
		let _ = self.sender.send(BgRequest::Update);
	}
}

static BG_WORKER: Lazy<BgWorker> = Lazy::new(BgWorker::start);

fn update() {
	polybar::update();
}

pub fn update_and_bg() {
	update();
	BG_WORKER.request_update();
}

pub fn update_and_bg_blocking() {
	update();
	update_bg_with_mode(BgRunMode::Blocking);
}

fn worker_loop(receiver: mpsc::Receiver<BgRequest>) {
	while receiver.recv().is_ok() {
		while receiver.recv_timeout(Duration::from_millis(50)).is_ok() {}
		update_bg_with_mode(BgRunMode::Async);
	}
}

fn update_bg_with_mode(mode: BgRunMode) {
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

		run_background_command(cmd_name.as_str(), argv, mode);
	}
}

fn run_background_command(cmd_name: &str, argv: Vec<String>, mode: BgRunMode) {
	match mode {
		BgRunMode::Async => {
			if let Ok(child) = Command::new(cmd_name).args(argv).spawn() {
				reap_child(child);
			}
		}
		BgRunMode::Blocking => {
			let _ = Command::new(cmd_name).args(argv).status();
		}
	}
}

fn reap_child(mut child: Child) {
	thread::spawn(move || {
		let _ = child.wait();
	});
}

pub fn exec(_: Vec<String>) {
	loop {
		let Some(mut i3) = connect_event_stream() else {
			return;
		};

		for e in i3.listen() {
			match e {
				Ok(Event::Workspace(_)) => update_and_bg(),
				Ok(Event::Window(_)) => update(),
				Ok(Event::Output(_)) => update(),
				Ok(Event::Mode(_)) => update(),
				Ok(Event::BarConfig(_)) => update(),
				Ok(_) => {}
				Err(err) => {
					eprintln!("i3-wsman: lost i3 event stream: {}. Reconnecting.", err);
					thread::sleep(Duration::from_secs(1));
					break;
				}
			}
		}
	}
}

fn connect_event_stream() -> Option<I3Stream> {
	match i3::conn_sub(&WATCH_EVENTS) {
		Ok(i3) => Some(i3),
		Err(err) => {
			eprintln!("i3-wsman: unable to subscribe to i3 events: {}", err);
			eprintln!("i3-wsman: start i3 or set I3SOCK to the active i3 IPC socket.");
			None
		}
	}
}
