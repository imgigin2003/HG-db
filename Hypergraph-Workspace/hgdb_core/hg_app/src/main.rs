mod layered_client;

use layered_client::{
    Hyperedge,
    LayeredHypergraphRequest,
    send_layered_hypergraph,
};

use std::collections::HashMap;

#[tokio::main]
async fn main() {
    println!("Sending layered hypergraph to microservice...");

    let mut edges = HashMap::new();

    edges.insert(
        "e1".to_string(),
        Hyperedge {
            nodes: vec!["A".into(), "B".into(), "C".into()],
            layer: 0,
        },
    );

    edges.insert(
        "e2".to_string(),
        Hyperedge {
            nodes: vec!["C".into(), "D".into()],
            layer: 1,
        },
    );

    let request = LayeredHypergraphRequest {
        hyperedges: edges,
    };

    match send_layered_hypergraph(request).await {
        Ok(response) => {
            println!("✅ Response from layered service:");
            println!("{}", response);
        }
        Err(err) => {
            eprintln!("❌ Error calling layered service: {:?}", err);
        }
    }
}
