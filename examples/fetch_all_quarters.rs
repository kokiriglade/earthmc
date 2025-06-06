use earthmc::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::default().build()?;

    let quarters = client.all_quarters().await?;

    for quarter in &quarters {
        println!("{:?}", quarter);
    }

    println!("Total quarters: {}", quarters.len());

    Ok(())
}
