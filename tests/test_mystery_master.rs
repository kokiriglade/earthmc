use earthmc::mystery_master::MysteryMaster;

#[test]
fn test_mystery_master() {
    let raw_json = include_str!("inputs/mystery_master.json");
    let parsed: Vec<MysteryMaster> = serde_json::from_str(raw_json).unwrap();
    insta::assert_debug_snapshot!(parsed);
}
