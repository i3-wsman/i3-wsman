use super::*;

// Constraint::FromStr
#[test]
fn test_constraint_from_str_simple() {
	let focused = "focused";
	assert_eq!(focused.parse::<Constraint>(), Ok(Constraint::Focused));
	let visible = "visible";
	assert_eq!(visible.parse::<Constraint>(), Ok(Constraint::Visible));
	let hidden = "hidden";
	assert_eq!(hidden.parse::<Constraint>(), Ok(Constraint::Hidden));
	let group = "group";
	assert_eq!(group.parse::<Constraint>(), Ok(Constraint::Group));
	let nogroup = "nogroup";
	assert_eq!(nogroup.parse::<Constraint>(), Ok(Constraint::NoGroup));
	let nogroup = "no-group";
	assert_eq!(nogroup.parse::<Constraint>(), Ok(Constraint::NoGroup));
	let allowurgent = "allowurgent";
	assert_eq!(
		allowurgent.parse::<Constraint>(),
		Ok(Constraint::AllowUrgent)
	);
	let allowurgent = "allow-urgent";
	assert_eq!(
		allowurgent.parse::<Constraint>(),
		Ok(Constraint::AllowUrgent)
	);
	let output = "output";
	assert_eq!(output.parse::<Constraint>(), Ok(Constraint::Output));
	let output = "output=foobar";
	assert_eq!(output.parse::<Constraint>(), Ok(Constraint::Output));
}

#[test]
fn test_constraint_from_str_bad() {
	let foobar = "foobar";
	assert_eq!(foobar.parse::<Constraint>(), Err(ParseConstraintError));
}

// Criteria.add()
#[test]
fn test_criteria_add() {
	let mut criteria = Criteria::new();
	assert_eq!(criteria.get_bits(), 0);

	criteria.add(Constraint::Hidden);
	assert_eq!(criteria.get_bits(), Constraint::Hidden as u32);
}

#[test]
fn test_criteria_add_multiple() {
	let mut criteria = Criteria::new();
	assert_eq!(criteria.get_bits(), 0);

	criteria.add(Constraint::Visible);
	assert_eq!(criteria.get_bits(), Constraint::Visible as u32);

	criteria.add(Constraint::Group);
	assert_eq!(
		criteria.get_bits(),
		Constraint::Visible as u32 | Constraint::Group as u32
	);
}

#[test]
fn test_criteria_add_twice() {
	let mut criteria = Criteria::new();
	assert_eq!(criteria.get_bits(), 0);

	criteria.add(Constraint::Hidden);
	assert_eq!(criteria.get_bits(), Constraint::Hidden as u32);

	// We're not doing addition here.
	criteria.add(Constraint::Hidden);
	assert_eq!(criteria.get_bits(), Constraint::Hidden as u32);
}

// Criteria.contains()
#[test]
fn test_criteria_contains() {
	let mut criteria = Criteria::new();
	assert_eq!(criteria.get_bits(), 0);

	criteria.add(Constraint::Hidden);
	assert!(criteria.contains(Constraint::Hidden));
	assert!(!criteria.contains(Constraint::Visible));
}

// Criteria.remove()
#[test]
fn test_criteria_remove() {
	let mut criteria = Criteria::new();
	assert_eq!(criteria.get_bits(), 0);

	criteria.add(Constraint::Hidden);
	criteria.remove(Constraint::Hidden);
	assert_eq!(criteria.get_bits(), 0);
}

#[test]
fn test_criteria_remove_multiple() {
	let mut criteria = Criteria::new();
	assert_eq!(criteria.get_bits(), 0);

	criteria.add(Constraint::Visible);
	criteria.add(Constraint::Group);
	criteria.remove(Constraint::Group);
	assert_eq!(criteria.get_bits(), Constraint::Visible as u32);
}

#[test]
fn test_criteria_remove_twice() {
	let mut criteria = Criteria::new();
	assert_eq!(criteria.get_bits(), 0);

	// We're not doing removeition here.
	criteria.add(Constraint::Hidden);
	criteria.remove(Constraint::Hidden);
	criteria.remove(Constraint::Hidden);
	assert_eq!(criteria.get_bits(), 0);
}

// from_vec()
#[test]
fn test_criteria_from_str() {
	let constraints = "focused     hidden";

	match constraints.parse::<Criteria>() {
		Ok(criteria) => {
			assert!(criteria.contains(Constraint::Focused));
			assert!(criteria.contains(Constraint::Hidden));
			assert!(!criteria.contains(Constraint::Visible));
		}
		Err(_) => assert!(false),
	}
}

// from_vec()
#[test]
fn test_from_vec() {
	let constraints = vec!["focused".to_owned(), "hidden".to_owned()];

	let criteria = from_vec(constraints);
	assert!(criteria.contains(Constraint::Focused));
	assert!(criteria.contains(Constraint::Hidden));
	assert!(!criteria.contains(Constraint::Visible));
}

// #[derive(Debug, Clone)]
// pub struct Output {
// 	pub expected: String,
// }
//
// impl Output {
// 	pub fn by_name(output_name: &str) -> Option<Output> {
// 		Some(Output {
// 			expected: output_name.to_owned(),
// 		})
// 	}
// }
//
// pub fn get_current_output() -> Output {
// 	Output {
// 		expected: "Some Output".to_owned(),
// 	}
// }
