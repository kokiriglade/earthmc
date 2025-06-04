use earthmc::quarter::Quarter;

#[test]
fn test_quarter() {
    let raw_json = include_str!("inputs/quarter.json");
    let parsed: Vec<Quarter> = serde_json::from_str(raw_json).unwrap();
    insta::assert_debug_snapshot!(parsed);
}
