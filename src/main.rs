use tokio;
use serde_json::json;
use qdrant_client::prelude::*;
use qdrant_client::client::QdrantClient;
use qdrant_client::qdrant::{CreateCollection, PointStruct, SearchPoints};
use qdrant_client::qdrant::{vectors_config::Config, VectorParams, VectorsConfig};
use anyhow::Result;


#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the client
    let client = QdrantClient::from_url("http://localhost:6334").build()?;

    // The Rust client uses Qdrant's GRPC interface to initialize
    //let mut client = QdrantClient::from_url("http://localhost:6334").build()?;

    // Create a collection
    let collection_name = "test_collection";

    client
        .create_collection(&CreateCollection {
            collection_name: collection_name.into(),
            vectors_config: Some(VectorsConfig {
                config: Some(Config::Params(VectorParams {
                    size: 4,
                    distance: Distance::Dot.into(),
                    ..Default::default()
                })),
            }),
            ..Default::default()
        })
        .await?;

    // add a few vectors with a payload
    let points = vec![
        PointStruct::new(
            1,
            vec![0.05, 0.61, 0.76, 0.74],
            json!(
                {"city": "Berlin"}
            )
            .try_into()
            .unwrap(),
        ),
        PointStruct::new(
            2,
            vec![0.19, 0.81, 0.75, 0.11],
            json!(
                {"city": "London"}
            )
            .try_into()
            .unwrap(),
        ),
        PointStruct::new(
            3,
            vec![0.1, 0.90, 0.72, 0.22],
            json!(
                {"city": "Hong Kong"}
            )
            .try_into()
            .unwrap(),
        ),
        PointStruct::new(
            3,
            vec![0.23, 0.04, 0.74, 0.55],
            json!(
                {"city": "San Diego"}
            )
            .try_into()
            .unwrap(),
        ),
    ];
    let operation_info = client
        .upsert_points_blocking("test_collection".to_string(), None, points, None)
        .await?;

    println!("Data ingest successful...");

    dbg!(operation_info);

    //run a query
    let search_result = client
        .search_points(&SearchPoints {
            collection_name: "test_collection".to_string(),
            vector: vec![0.2, 0.1, 0.9, 0.7],
            limit: 3,
            with_payload: Some(true.into()),
            ..Default::default()
        })
        .await?;

    //dbg!(search_result);

    // Visualize the output for each found point
    for (index, point) in search_result.result.iter().enumerate() {
        println!("Point {} Payload: {:?} Score: {}", index + 1, serde_json::to_string_pretty(&point.payload).unwrap(), point.score);
    }


    // Ingest data into Qdrant
    //ingest_data(&mut client, collection_name).await?;

    // Perform a search query
    //search_query(&mut client, collection_name).await?;

    Ok(())
}
