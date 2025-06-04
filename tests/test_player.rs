use earthmc::player::Player;

#[test]
fn test_player() {
    let raw_json = include_str!("inputs/player.json");
    let parsed: Vec<Player> = serde_json::from_str(raw_json).unwrap();
    insta::assert_debug_snapshot!(parsed);
}
