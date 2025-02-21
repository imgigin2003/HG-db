use hgdb_core::hyper_edge::repository::simple_h_edge_repository::SimpleHyperEdgeRepository;
use hgdb_core::hyper_edge::entity::simple_h_edge::{SimpleHyperEdge, Property};

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fs::remove_dir_all;

    const DB_PATH: &str = "/users/gigin/documents/mydbs/rocksdb/simple-h-edge"; // RocksDB path

    #[test]
    fn test_simple_h_edge_crud_operation() -> Result<(), Box<dyn Error>> {
        //delete the database folder before running the test
        let _ = remove_dir_all(DB_PATH);

        //initialize repository
        let repository = SimpleHyperEdgeRepository::new(DB_PATH)?;

        //define test data for directed graphs
        let test_key_directed = "test_edge_directed";
        let test_edge_directed = SimpleHyperEdge {
            id: test_key_directed.to_string(),
            name: "Friendship Directed".to_string(),
            main_properties: vec![
                Property {
                    key: "relationship-type".to_string(),
                    value: vec!["friends".to_string()]
                }
            ],
            traversable: true,
            directed: true,
            head_hyper_nodes: Box::new(vec!["v1".to_string(), "v2".to_string()]),
            tail_hyper_nodes: Some(Box::new(vec!["v3".to_string(), "v4".to_string()]))
        };

        // define test data for undirected graph
        let test_key_undirected = "test_edge_undirected";
        let test_edge_undirected = SimpleHyperEdge {
            id: test_key_undirected.to_string(),
            name: "Friendship Undirected".to_string(),
            main_properties: vec![
                Property {
                    key: "relationship-type".to_string(),
                    value: vec!["friends".to_string()]
                }
            ],
            traversable: false,
            directed: false,
            head_hyper_nodes: Box::new(vec!["v1".to_string(), "v2".to_string()]),
            tail_hyper_nodes: None // no tail nodes for undirected graph
        };

        // create directed graph
        repository.create(test_key_directed, &test_edge_directed)?;
        // create undirected graph
        repository.create(test_key_undirected, &test_edge_undirected)?;

        // retrieve all edges and assert there's atleast one edge
        let all_edges = repository.get_all()?;
        assert!(all_edges.len() >= 1, "❌ Not all edges were retrieved");

        // retrieve by key again and verify for directed graph
        let retrieve_edge_directed = repository.get_by_key(test_key_directed)?;
        assert!(retrieve_edge_directed.is_some(), "❌ Directed edge was not found in database");
        assert_eq!(retrieve_edge_directed.unwrap().name, "Friendship Directed", "❌ Retrieved edge name mismatch");
        // retrieve by key again and verify for undirected graph
        let retrieve_edge_undirected = repository.get_by_key(test_key_undirected)?;
        assert!(retrieve_edge_undirected.is_some(), "❌ Undirected edge was not found in database");
        assert_eq!(retrieve_edge_undirected.unwrap().name, "Friendship Undirected", "❌ Retrieve edge name mismatch");

        // log retrieved edge for debugging (directed)
        let retrieved_edge = repository.get_by_key(test_key_directed)?;
        assert!(retrieved_edge.is_some(), "❌ Edge was not found in database after create");
        println!("✅ Retrieved Directed Edge");
        // log retrieved edge for debugging (undirected)
        let retrieved_edge = repository.get_by_key(test_key_undirected)?;
        assert!(retrieved_edge.is_some(), "❌ Edge was not found in database after create");
        println!("✅ Retrieved Directed Edge");

        // test delete function
        repository.delete(test_key_directed)?;
        repository.delete(test_key_undirected)?;

        //ensure all edges are deleted after calling the delete_all function
        let all_edges_after_delete = repository.get_all()?;
        assert_eq!(all_edges_after_delete.len(), 0, "❌ Database should be empty after delete_all function");

        // Try retrieving individual edges (should return None)
        let deleted_directed_edge = repository.get_by_key(test_key_directed)?;
        let deleted_undirected_edge = repository.get_by_key(test_key_undirected)?;
        assert!(deleted_directed_edge.is_none(), "Directed edge was not deleted by delete_all");
        assert!(deleted_undirected_edge.is_none(), "Undirected edge was not deleted by delete_all");

    Ok(())
    }
}
