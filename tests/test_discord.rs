use earthmc::discord_link::DiscordLink;

#[test]
fn test_location() {
    let raw_json = include_str!("inputs/discord.json");
    let parsed: Vec<DiscordLink> = serde_json::from_str(raw_json).unwrap();
    insta::assert_debug_snapshot!(parsed);
}
