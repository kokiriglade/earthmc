use earthmc::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();

    let towns = client.all_towns().await?;

    for town in &towns {
        println!("{:?}", town);
    }

    println!("Total towns: {}", towns.len());

    Ok(())
}
