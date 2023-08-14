#[allow(dead_code)]
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

#[allow(dead_code)]
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

	pub fn remove(&mut self, constraint: Constraint) {
		self.bit &= !(constraint as u32);
	}

	pub fn contains(&self, constraint: Constraint) -> bool {
		self.bit != Constraint::None as u32 && (self.bit & constraint as u32) != 0
	}
}

/*
fn main() {
	let mut constraints = Constraints::new();
	constraints.add(Constraint::Visible);
	constraints.add(Constraint::Group);

	if constraints.contains(Constraint::Visible) {
		println!("Constraint is visible!");
	}

	if constraints.contains(Constraint::Group) {
		println!("Constraint is grouped!");
	}
}
*/
