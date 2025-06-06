use earthmc::{
    Client,
    query::{
        NearbyQueryBuilder, NearbyQueryItemBuilder, NearbySearchType,
        NearbyTarget, NearbyTargetType,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();

    let query = NearbyQueryBuilder::default()
        .add(
            NearbyQueryItemBuilder::default()
                .target_type(NearbyTargetType::Town)
                .target(NearbyTarget::Town("Berlin".to_string()))
                .search_type(NearbySearchType::Town)
                .radius(500)
                .build()?,
        )
        .add(
            NearbyQueryItemBuilder::default()
                .target_type(NearbyTargetType::Coordinate)
                .target(NearbyTarget::Coordinates([0, 0]))
                .search_type(NearbySearchType::Town)
                .radius(1500)
                .build()?,
        )
        .build()?;

    let nearby_targets = client.nearby(query).await?;

    let towns_nearby_berlin = &nearby_targets[0];
    println!("Towns nearby Berlin ({}):", towns_nearby_berlin.len());
    for result in towns_nearby_berlin {
        println!("{:?}", result);
    }

    let towns_nearby_coordinate = &nearby_targets[1];
    println!(
        "\n\nTowns nearby [0,0] ({}):",
        towns_nearby_coordinate.len()
    );
    for result in towns_nearby_coordinate {
        println!("{:?}", result);
    }

    Ok(())
}
