use earthmc::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();

    let mystery_master = client.mystery_master().await?;

    for master in &mystery_master {
        println!("{:?} - {:?}", master.name, master.change);
    }

    Ok(())
}
