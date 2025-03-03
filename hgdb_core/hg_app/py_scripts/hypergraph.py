import hypernetx as hnx
import matplotlib.pyplot as plt
import json
import os

JSON_FILE_PATH = "/Users/gigin/Documents/Github/HG-DB/hgdb_core/hg_app/py_scripts/json-data/test_simple.json"

def load_hyperedges_from_json(json_file_path):
    """Load hyperedges from a JSON file and format them for HyperNetX."""
    if not os.path.exists(json_file_path):
        print(f"Error: JSON file not found at {json_file_path}")
        return {}

    with open(json_file_path, 'r') as file:
        try:
            data = json.load(file)
        except json.JSONDecodeError as e:
            print(f"Error: Invalid JSON format - {e}")
            return {}

    hyperedges = {}
    for hypergraph_key in data:
        for edge in data[hypergraph_key]:
            edge_name = edge["name"]
            head_nodes = edge["head_hyper_nodes"]
            tail_nodes = edge["tail_hyper_nodes"] if edge["tail_hyper_nodes"] is not None else []
            nodes = head_nodes + tail_nodes
            hyperedges[edge_name] = set(nodes)

    return hyperedges

def create_hypergraph():
    """Create a hypergraph using dynamically loaded data from JSON."""
    hyperedges = load_hyperedges_from_json(JSON_FILE_PATH)
    if not hyperedges:
        return None, None
    H = hnx.Hypergraph(hyperedges)
    return H, hyperedges

def draw_hypergraph(H):
    """Draw the hypergraph using HyperNetX and return the figure."""
    if H is None:
        return None
    
    fig, ax = plt.subplots(figsize=(12, 10))
    hnx.draw(H, with_node_labels=True, with_edge_labels=True, ax=ax)
    plt.title("Hypergraph Visualization")
    return fig