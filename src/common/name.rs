#[allow(dead_code)]
pub fn num(name: &str) -> String {
	name.split(':').collect::<Vec<&str>>()[0].to_owned()
}

#[allow(dead_code)]
pub fn group(name: &str) -> String {
	let parts: Vec<&str> = name.split(':').collect();
	if parts.len() > 1 {
		parts[1].to_owned()
	} else {
		"".to_owned()
	}
}

#[allow(dead_code)]
pub fn name(name: &str) -> String {
	let parts: Vec<&str> = name.split(':').collect();
	if parts.len() > 2 {
		parts[2].to_owned()
	} else {
		"".to_owned()
	}
}

pub fn change_prefix(name: &str, new_prefix: i32) -> String {
	let parts: Vec<&str> = name.split(':').collect();
	if parts.len() > 1 {
		format!("{}:{}", new_prefix, parts[1])
	} else {
		new_prefix.to_string()
	}
}

#[allow(dead_code)]
pub fn set_group(name: &str, group: &str) -> String {
	let parts: Vec<&str> = name.split(':').collect();
	if parts.len() < 3 {
		format!("{}:{}", parts[0], group)
	} else {
		format!("{}:{}:{}", parts[0], group, parts[2..].join(":"))
	}
}

