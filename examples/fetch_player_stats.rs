use earthmc::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();

    // you can get it as a map like this
    // let stats: std::collections::BTreeMap<String, i64> =
    //     (client.player_stats().await?).into();

    // for stat in &stats {
    //     println!("{}: {}", stat.0, stat.1);
    // }

    let stats = client.player_stats().await?;

    for stat in &stats.0 {
        println!("{:?}", stat);
    }

    Ok(())
}
