use std::str::FromStr;

use crate::i3::{get_current_output, outputs::Output};

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
pub struct Criteria {
	bit: u32,
	pub output: Option<Output>,
}

impl Criteria {
	pub fn new() -> Self {
		Criteria {
			bit: 0,
			output: None,
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
		constraint == Constraint::None && self.bit == Constraint::None as u32
			|| (self.bit & constraint as u32) != 0
	}
}

pub fn from_vec(nouns: Vec<String>) -> Criteria {
	let mut criteria = Criteria::new();

	for noun in nouns {
		if let Ok(c) = noun.parse::<Constraint>() {
			if noun.contains(&"output=") {
				criteria.output = match noun.split("=").nth(1) {
					Some(output_name) => Output::by_name(output_name),
					None => None,
				};
			} else if c == Constraint::Output && !criteria.contains(c) {
				criteria.output = Some(get_current_output());
			}
			criteria.add(c);
		}
	}

	criteria
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCriteriaError;

impl FromStr for Criteria {
	type Err = ParseCriteriaError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let nouns = s
			.split_whitespace()
			.map(|s| s.to_owned())
			.collect::<Vec<_>>();

		Ok(from_vec(nouns))
	}
}
