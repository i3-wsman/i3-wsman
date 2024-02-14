use super::*;

// dedup_vec()
#[test]
fn test_dedup_vec() {
	let mut actual = vec!["foo", "bar", "baz", "baz", "foo"];
	let expected = vec!["foo", "bar", "baz"];

	dedup_vec::<&str>(&mut actual);

	assert!(actual.iter().eq(expected.iter()));
}

// Direction::FromStr()
#[test]
fn test_direction_from_str() {
	let left = "left";
	assert_eq!(left.parse::<Direction>(), Ok(Direction::Left));

	let right = "right";
	assert_eq!(right.parse::<Direction>(), Ok(Direction::Right));
}

#[test]
fn test_direction_from_str_fallback_to_right() {
	let foobar = "baz";
	assert_eq!(foobar.parse::<Direction>(), Ok(Direction::Right));
}

// Direction.to_string()
#[test]
fn test_direction_to_string() {
	assert_eq!(Direction::Left.to_string(), "left");
	assert_eq!(Direction::Right.to_string(), "right");
}
