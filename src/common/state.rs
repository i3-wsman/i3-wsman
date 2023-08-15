use std::env;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use fs2::FileExt;

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
	pub groups: HashMap<String, Vec<String>>,
	pub show_hidden: bool,
}

fn usid() -> String {
	let session_id = env::var("XDG_SESSION_ID").unwrap_or_default();
	let vtnr = env::var("XDG_VTNR").unwrap_or_default();
	let seat = env::var("XDG_SEAT").unwrap_or_default();

	format!("{}{}{}", session_id, vtnr, seat)
}

fn get_tmpf(filename: &str) -> PathBuf {
	let usid = usid(); // Assuming usid function is implemented
	let filename = format!("i3wsm__tmp_{}_{}", usid, filename);

	let temp_dir = if PathBuf::from("/dev/shm").exists() {
		PathBuf::from("/dev/shm")
	} else {
		env::temp_dir()
	};

	let temp_file_path = temp_dir.join(filename);

	// Create the file only if it doesn't exist
	if !temp_file_path.exists() {
		File::create(&temp_file_path).expect("Failed to create temporary file");
		set(State {
			groups: HashMap::new(),
			show_hidden: false,
		});
	}

	temp_file_path
}

pub fn set(state: State) {
	let serialized_data = serde_json::to_string(&state).expect("Failed to serialize state");

	let temp_file_path = get_tmpf("state");
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
	let mut file = File::open(&temp_file_path).expect("Failed to open temporary file");

	file.lock_shared().expect("Failed to lock file");

	let mut serialized_data = String::new();
	file.read_to_string(&mut serialized_data)
		.expect("Failed to read from temporary file");

	file.unlock().expect("Failed to unlock file");

	serde_json::from_str(&serialized_data).expect("Failed to deserialize state")
}
