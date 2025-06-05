use earthmc::location::LocationInfo;

#[test]
fn test_location() {
    let raw_json = include_str!("inputs/location.json");
    let parsed: Vec<LocationInfo> = serde_json::from_str(raw_json).unwrap();
    insta::assert_debug_snapshot!(parsed);
}
