use std::str::FromStr;

use earthmc::{
    ClientBuilder,
    query::{DiscordQueryBuilder, DiscordQueryItem},
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::default().build()?;

    let query = DiscordQueryBuilder::default()
        .add(DiscordQueryItem::Minecraft {
            target: Uuid::from_str("f17d77ab-aed4-44e7-96ef-ec9cd473eda3")?,
        })
        .add(DiscordQueryItem::Discord {
            target: "160374716928884736".into(),
        })
        .build()?;

    let linked_accounts = client.discord(query).await?;

    for account in &linked_accounts {
        println!("{} = {}", account.id, account.uuid);
    }

    Ok(())
}
