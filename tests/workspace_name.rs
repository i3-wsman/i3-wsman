use i3_wsman::i3::workspace::{parse_name, assemble_name};
use i3_wsman::config::global::GotoBehavior;
use std::str::FromStr;

#[test]
fn parse_name_splits_workspace() {
    let (num, group, name) = parse_name("1:dev:editor".to_string());
    assert_eq!(num, "1");
    assert_eq!(group, "dev");
    assert_eq!(name, "editor");
}

#[test]
fn assemble_name_reassembles_parts() {
    let name = assemble_name(2, "ops".to_string(), "logs".to_string());
    assert_eq!(name, "2:ops:logs");
}

#[test]
fn parse_and_assemble_roundtrip() {
    let original = "3:grp:name";
    let parts = parse_name(original.to_string());
    let rebuilt = assemble_name(parts.0.parse::<i32>().unwrap(), parts.1, parts.2);
    assert_eq!(rebuilt, original);
}

#[test]
fn goto_behavior_from_str_invalid_returns_create() {
    let b = GotoBehavior::from_str("invalid").unwrap();
    assert_eq!(b, GotoBehavior::Create);
}
