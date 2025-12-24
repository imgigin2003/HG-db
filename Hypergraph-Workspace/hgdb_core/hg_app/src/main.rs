mod layered_client;

use layered_client::{
    Hyperedge,
    LayeredHypergraphRequest,
    send_layered_hypergraph
};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    println!("Sending Layered Hypergraph to Service...");

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
            layer: 0,
        },
    );

    let req = LayeredHypergraphRequest { hyperedges: edges };

    match send_layered_hypergraph(req).await {
        Ok(resp) => println!("Response from service:\n{}", resp),
        Err(e) => eprintln!("Error{:?}", e)
    }


}