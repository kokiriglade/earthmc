use earthmc::town::Town;

#[test]
fn test_town() {
    let raw_json = include_str!("inputs/town.json");
    let parsed: Vec<Town> = serde_json::from_str(raw_json).unwrap();
    insta::assert_debug_snapshot!(parsed);
}
