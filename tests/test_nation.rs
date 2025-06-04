use earthmc::nation::Nation;

#[test]
fn test_nation() {
    let raw_json = include_str!("inputs/nation.json");
    let parsed: Vec<Nation> = serde_json::from_str(raw_json).unwrap();
    insta::assert_debug_snapshot!(parsed);
}
