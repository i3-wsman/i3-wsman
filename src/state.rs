use fs2::FileExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::sync::Mutex;

use crate::i3;
use crate::CONFIG;

lazy_static! {
	static ref I3_LOCK_PATH: PathBuf = get_tmpf("i3.lock");
	static ref I3_LOCK: Mutex<io::Result<std::fs::File>> = Mutex::new(
		OpenOptions::new()
			.read(true)
			.write(true)
			.create(true)
			.open(I3_LOCK_PATH.to_owned())
	);
	static ref I3_LOCK_OBTAINED: Mutex<bool> = Mutex::new(false);
}

type StateGroups = HashMap<String, Vec<String>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct State {
	pub groups: StateGroups,
	#[deprecated]
	pub global_groups: Vec<String>,
	pub show_hidden: bool,
}

fn default() -> State {
	let active_groups = CONFIG.startup.active_workspace_groups.to_owned();

	let mut groups: StateGroups = HashMap::new();
	for o in i3::get_outputs() {
		groups.insert(o.name(), active_groups.clone());
	}
	groups.insert(i3::outputs::XROOT.to_string(), active_groups);

	State {
		groups,
		global_groups: vec![],
		show_hidden: CONFIG.startup.show_hidden_workspaces,
	}
}

fn usid() -> String {
	let session_id = env::var("XDG_SESSION_ID").unwrap_or_default();
	let vtnr = env::var("XDG_VTNR").unwrap_or_default();
	let seat = env::var("XDG_SEAT").unwrap_or_default();

	format!("{}{}{}", session_id, vtnr, seat)
}

fn get_tmpf(filename: &str) -> PathBuf {
	let usid = usid();
	let filename = format!("i3wsm__tmp_{}_{}", usid, filename);

	let temp_dir = if PathBuf::from("/dev/shm").exists() {
		PathBuf::from("/dev/shm")
	} else {
		env::temp_dir()
	};

	temp_dir.join(filename)
}

pub fn obtain_i3_lock() -> Result<(), io::Error> {
	let mut locked = I3_LOCK_OBTAINED.lock().unwrap();
	if *locked == true {
		return Ok(());
	}
	let lockfile = I3_LOCK.lock().unwrap();
	if let Ok(file) = &*lockfile {
		file.lock_exclusive()?;
	} else {
		return Err(io::Error::new(
			io::ErrorKind::Other,
			format!(
				"Failed to initialize lock file: {}",
				I3_LOCK_PATH.as_os_str().to_string_lossy()
			),
		));
	}
	*locked = true;
	Ok(())
}

pub fn release_i3_lock() -> Result<(), io::Error> {
	let locked = I3_LOCK_OBTAINED.lock().unwrap();
	if *locked == false {
		return Ok(());
	}
	drop(locked);

	i3::workspace_maintenance();

	let mut locked = I3_LOCK_OBTAINED.lock().unwrap();
	let lockfile = I3_LOCK.lock().unwrap();
	if let Ok(file) = &*lockfile {
		file.unlock()?;
	} else {
		return Err(io::Error::new(
			io::ErrorKind::Other,
			format!(
				"Failed to initialize lock file: {}",
				I3_LOCK_PATH.as_os_str().to_string_lossy()
			),
		));
	}
	*locked = false;
	Ok(())
}

pub fn set(state: State) {
	let mut state = state.clone();
	state.global_groups = state
		.groups
		.get(&i3::outputs::XROOT.to_string())
		.unwrap()
		.clone();

	let serialized_data = serde_json::to_string(&state).expect("Failed to serialize state");

	let temp_file_path = get_tmpf("state");
	if !temp_file_path.exists() {
		File::create(&temp_file_path).expect("Failed to create temporary file");
		set(default());
	}

	let mut file = OpenOptions::new()
		.write(true)
		.create(true)
		.open(&temp_file_path)
		.expect("Failed to open temporary file");

	file.lock_exclusive().expect("Failed to lock file");
	file.set_len(0).expect("Failed to truncate file");
	file.write_all(serialized_data.as_bytes())
		.expect("Failed to write to temporary file");
	file.unlock().expect("Failed to unlock file");
}

pub fn get() -> State {
	let temp_file_path = get_tmpf("state");
	if !temp_file_path.exists() {
		set(default());
	}

	let mut file = File::open(&temp_file_path).expect("Failed to open temporary file");

	file.lock_shared().expect("Failed to lock file");

	let mut serialized_data = String::new();
	file.read_to_string(&mut serialized_data)
		.expect("Failed to read from temporary file");

	file.unlock().expect("Failed to unlock file");

	if serialized_data.trim().len() == 0 {
		set(default());
		return get();
	}

	serde_json::from_str(&serialized_data).unwrap_or_else(|_| {
		set(default());
		get()
	})
}
