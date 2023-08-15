pub fn change_prefix(name: &str, new_prefix: i32) -> String {
	let parts: Vec<&str> = name.split(':').collect();
	if parts.len() > 1 {
		format!("{}:{}", new_prefix, parts[1])
	} else {
		new_prefix.to_string()
	}
}
