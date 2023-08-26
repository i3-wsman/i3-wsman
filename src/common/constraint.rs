use std::str::FromStr;

use super::outputs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Constraint {
	None = 0,
	Focused = 1 << 0,
	Output = 1 << 1,
	Visible = 1 << 2,
	Hidden = 1 << 3,
	Group = 1 << 4,
	NoGroup = 1 << 5,
	AllowUrgent = 1 << 6,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseConstraintError;

impl FromStr for Constraint {
	type Err = ParseConstraintError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let s = s.to_lowercase();
		if s.contains(&"output=") {
			return Ok(Constraint::Output);
		}

		match s.as_ref() {
			"focused" => Ok(Constraint::Focused),
			"visible" => Ok(Constraint::Visible),
			"hidden" => Ok(Constraint::Hidden),
			"group" => Ok(Constraint::Group),
			"nogroup" => Ok(Constraint::NoGroup),
			"no-group" => Ok(Constraint::NoGroup),
			"allowurgent" => Ok(Constraint::AllowUrgent),
			"allow-urgent" => Ok(Constraint::AllowUrgent),
			"output" => Ok(Constraint::Output),
			_ => Err(ParseConstraintError),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Constraints {
	bit: u32,
	pub output: String,
}

impl Constraints {
	pub fn new() -> Self {
		Constraints {
			bit: 0,
			output: "none".to_string(),
		}
	}

	pub fn add(&mut self, constraint: Constraint) {
		self.bit |= constraint as u32;
	}

	#[allow(dead_code)]
	pub fn remove(&mut self, constraint: Constraint) {
		self.bit &= !(constraint as u32);
	}

	pub fn contains(&self, constraint: Constraint) -> bool {
		self.bit != Constraint::None as u32 && (self.bit & constraint as u32) != 0
	}
}

pub fn from_vec(nouns: Vec<String>) -> Constraints {
	let mut constraints = Constraints::new();

	for noun in nouns {
		if let Ok(c) = noun.parse::<Constraint>() {
			if noun.contains(&"output=") {
				constraints.output = noun
					.split("=")
					.nth(1)
					.unwrap_or("none")
					.to_string();
			} else if c == Constraint::Output && !constraints.contains(c) {
				constraints.output = outputs::focused();
			}
			constraints.add(c);
		}
	}

	constraints
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseConstraintsError;

impl FromStr for Constraints {
	type Err = ParseConstraintsError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let nouns = s
			.split_whitespace()
			.map(|s| s.to_owned())
			.collect::<Vec<_>>();

		Ok(from_vec(nouns))
	}
}
