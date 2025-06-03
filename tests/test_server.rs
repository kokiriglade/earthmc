use earthmc::server::Server;

#[test]
fn test_server() {
    let raw_json = include_str!("inputs/server.json");
    let parsed: Server = serde_json::from_str(raw_json).unwrap();
    insta::assert_debug_snapshot!(parsed);
}
