use earthmc::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::default().build()?;

    let mystery_master = client.mystery_master().await?;

    for master in &mystery_master {
        println!("{:?} - {:?}", master.name, master.change);
    }

    Ok(())
}
