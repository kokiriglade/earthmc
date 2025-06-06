use earthmc::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::default().build()?;

    let server = client.server().await?;

    println!(
        "{}/{} players are currently online. There are {} nomads, {} towns, and {} nations.",
        server.stats.num_online_players,
        server.stats.max_players,
        server.stats.num_nomads,
        server.stats.num_towns,
        server.stats.num_nations
    );

    Ok(())
}
