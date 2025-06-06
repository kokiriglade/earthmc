use earthmc::{Client, query::LocationQueryBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();

    let query = LocationQueryBuilder::default()
        .add([0, 0])
        .add([100, 100])
        .build()?;

    let locations = client.locations(query).await?;

    for location in &locations {
        println!("{:?}", location);
    }

    Ok(())
}
