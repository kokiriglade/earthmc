use std::str::FromStr;

use earthmc::{Client, query::UuidQueryBuilder};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();

    let query = UuidQueryBuilder::default()
        .insert(Uuid::from_str("971c5315-12c2-4c15-a8c3-d65121fa4e07")?)
        .build()?;

    // note that if the Quarter with that UUID stops existing, this example will
    // fail.

    let quarters = client.quarters(query).await?;

    for quarter in &quarters {
        println!(
            "Quarter {} (of town {}) has {} cuboids.",
            quarter.name, quarter.town.name, quarter.stats.num_cuboids
        )
    }

    Ok(())
}
