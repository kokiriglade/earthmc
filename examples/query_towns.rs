use earthmc::{ClientBuilder, query::TownQueryBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::default()
        .build()
        .expect("Builder defaults are valid");

    let query = TownQueryBuilder::default()
        .town("London".into())
        .town(String::from("Berlin").into())
        .build()
        .expect("Builder is valid");

    let towns = client.query_towns(query).await?;

    for town in &towns {
        println!(
            "{}'s town, {}, has {} gold, {} residents, and {} town blocks.",
            town.mayor.name,
            town.name,
            town.stats.balance,
            town.stats.num_residents,
            town.stats.num_town_blocks
        );
    }

    Ok(())
}
