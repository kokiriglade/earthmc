use earthmc::{ClientBuilder, query::SimpleQueryBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::default()
        .build()
        .expect("Builder defaults are valid");

    let query = SimpleQueryBuilder::default()
        .add("London")
        .add("Berlin")
        .add("THIS_TOWN_PROBABLY_DOES_NOT_EXIST")
        .build()
        .expect("Builder is valid");

    let towns = client.query_towns(query).await?;

    // non-existent towns are simply omitted
    assert_eq!(towns.len(), 2);

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
