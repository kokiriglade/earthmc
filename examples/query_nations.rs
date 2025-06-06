use earthmc::{Client, query::SimpleQueryBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();

    let query = SimpleQueryBuilder::default()
        .add("Germany")
        .add("France")
        .add("THIS_NATION_PROBABLY_DOES_NOT_EXIST")
        .build()?;

    let nations = client.nations(query).await?;

    // non-existent nations are simply omitted
    assert_eq!(nations.len(), 2);

    for nation in &nations {
        println!(
            "{}'s nation, {}, has {} gold, {} residents, and {} town blocks.",
            nation.leader.name,
            nation.name,
            nation.stats.balance,
            nation.stats.num_residents,
            nation.stats.num_town_blocks
        );
    }

    Ok(())
}
