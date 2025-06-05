use earthmc::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::default()
        .build()
        .expect("Builder defaults are valid");

    let players = client.get_all_players().await?;

    // stdout is slow :(
    // for player in &players {
    //     println!("{:?}", player);
    // }

    println!("Total players: {}", players.len());

    Ok(())
}
