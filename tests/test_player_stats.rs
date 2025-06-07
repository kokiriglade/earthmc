use earthmc::player_stats::StatMap;

#[test]
fn test_player_stats() {
    let raw_json = include_str!("inputs/player_stats.json");
    let parsed: StatMap = serde_json::from_str(raw_json).unwrap();
    insta::assert_debug_snapshot!(parsed);
}
