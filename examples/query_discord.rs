use std::str::FromStr;

use earthmc::{
    Client,
    query::{DiscordQueryBuilder, DiscordQueryItem},
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();

    let query = DiscordQueryBuilder::default()
        .insert(DiscordQueryItem::Minecraft {
            target: Uuid::from_str("f17d77ab-aed4-44e7-96ef-ec9cd473eda3")?,
        })
        .insert(DiscordQueryItem::Discord {
            target: "160374716928884736".into(),
        })
        .build()?;

    let linked_accounts = client.discord(query).await?;

    for account in &linked_accounts {
        println!("{} = {}", account.id, account.uuid);
    }

    Ok(())
}
