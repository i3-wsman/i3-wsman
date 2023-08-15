use super::outputs;

#[derive(Debug, Clone, Copy)]
pub enum Constraint {
	None = 0,
	Focused = 1 << 0,
	Output = 1 << 1,
	Visible = 1 << 2,
	Hidden = 1 << 3,
	Group = 1 << 4,
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

pub fn parse(nouns: Vec<String>) -> Constraints {
	let mut constraints = Constraints::new();

	for noun in &nouns {
		if noun.contains(&"output=") {
			constraints.add(Constraint::Output);
			constraints.output = noun
				.split("=")
				.nth(1)
				.unwrap_or("none")
				.to_string();
			continue;
		}

		match noun.as_ref() {
			"focused" => constraints.add(Constraint::Focused),
			"visible" => constraints.add(Constraint::Visible),
			"hidden" => constraints.add(Constraint::Hidden),
			"group" => constraints.add(Constraint::Group),
			"output" => {
				if !constraints.contains(Constraint::Output) {
					constraints.add(Constraint::Output);
					constraints.output = outputs::focused();
				}
			},
			_ => {},
		}
	}

	constraints
}
