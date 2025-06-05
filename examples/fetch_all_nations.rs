use earthmc::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::default().build()?;

    let nations = client.all_nations().await?;

    for nation in &nations {
        println!("{:?}", nation);
    }

    println!("Total nations: {}", nations.len());

    Ok(())
}
