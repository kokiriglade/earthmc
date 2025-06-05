use earthmc::{ClientBuilder, query::SimpleQueryBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::default().build()?;

    let query = SimpleQueryBuilder::default()
        .add("Fix")
        .add("CorruptedGreed")
        .add("THIS_PLAYER_DEFINITELY_DOES_NOT_EXIST")
        .build()?;

    let players = client.players(query).await?;

    // non-existent players are simply omitted
    // NOTE: this assertion might not work if either Fix or CorruptedGreed stop
    //  logging in.
    assert_eq!(players.len(), 2);

    for player in &players {
        println!(
            "{} has {} gold and is {}. {}.",
            player.name,
            player.stats.balance,
            if player.status.is_online {
                "online"
            } else {
                "offline"
            },
            player.town.name.clone().map_or_else(
                || "They are not in a town".to_string(),
                |v| format!("Their town is called '{}'", v)
            )
        );
    }

    Ok(())
}
