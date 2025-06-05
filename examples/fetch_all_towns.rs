use earthmc::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::default()
        .build()
        .expect("Builder defaults are valid");

    let towns = client.get_all_towns().await?;

    for town in &towns {
        println!("{:?}", town);
    }

    println!("Total towns: {}", towns.len());

    Ok(())
}
