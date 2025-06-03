use earthmc::shared::NamedId;

#[test]
fn test_named_id() {
    let raw_json = include_str!("inputs/named_id.json");
    let parsed: NamedId = serde_json::from_str(raw_json).unwrap();
    insta::assert_debug_snapshot!(parsed);
}
