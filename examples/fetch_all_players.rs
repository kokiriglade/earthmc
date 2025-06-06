use earthmc::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();

    let players = client.all_players().await?;

    // stdout is slow :(
    // for player in &players {
    //     println!("{:?}", player);
    // }

    println!("Total players: {}", players.len());

    Ok(())
}
