import hypernetx as hnx
import matplotlib.pyplot as plt
import json
import os

JSON_FILE_PATH = "/Users/gigin/Documents/Github/HG-DB/hgdb_core/hg_app/py_scripts/json-data/test_dual.json"

def load_dual_hyperedges_from_json(json_file_path):
    """Load dual hyperedges from a JSON file and format them for HyperNetX."""
    if not os.path.exists(json_file_path):
        print(f"Error: JSON file not found at {json_file_path}")
        return {}

    with open(json_file_path, 'r') as file:
        try:
            data = json.load(file)
        except json.JSONDecodeError as e:
            print(f"Error: Invalid JSON format - {e}")
            return {}

    dual_hyperedges = {}  # Node (dual edge name) -> set of hyperedges (original edges)
    # Iterate over each dual hypergraph
    for hypergraph_key in data:
        for edge in data[hypergraph_key]:
            edge_name = edge["name"]  # Use the 'name' field (e.g., "v1_v2")
            # Get all nodes (original edges) this dual edge connects to
            head_nodes = edge.get("head_hyper_nodes", []) if edge.get("head_hyper_nodes") is not None else []
            tail_nodes = edge.get("tail_hyper_nodes", []) if edge.get("tail_hyper_nodes") is not None else []
            nodes = head_nodes + tail_nodes  # Combine head and tail nodes
            if nodes:  # Only add if there are nodes to avoid empty sets
                dual_hyperedges[edge_name] = set(nodes)
    return dual_hyperedges

def compute_dual_hypergraph(hyperedges=None):
    """Compute the dual hypergraph from either JSON or provided hyperedges."""
    if hyperedges is None:
        # Load from JSON if no hyperedges provided
        dual_hyperedges = load_dual_hyperedges_from_json(JSON_FILE_PATH)
    else:
        # Compute dual from provided hyperedges (original edges -> nodes)
        dual_hyperedges = {}
        all_nodes = set().union(*hyperedges.values())  # All nodes in original hypergraph
        for node in all_nodes:
            dual_hyperedges[node] = {edge_name for edge_name, nodes in hyperedges.items() if node in nodes}
    return dual_hyperedges

def create_dual_hypergraph(hyperedges=None):
    """Create a dual hypergraph using dynamically loaded data or provided hyperedges."""
    dual_hyperedges = compute_dual_hypergraph(hyperedges)
    if not dual_hyperedges:
        return None, None
    H_dual = hnx.Hypergraph(dual_hyperedges)
    return H_dual, dual_hyperedges

def draw_dual_hypergraph(H_dual):
    """Draw the dual hypergraph using HyperNetX and return the figure."""
    if H_dual is None:
        return None
    
    fig, ax = plt.subplots(figsize=(12, 10))
    hnx.draw(H_dual, with_node_labels=True, with_edge_labels=True, ax=ax)
    plt.title("Dual Hypergraph Visualization")
    return fig