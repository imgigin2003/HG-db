import hypernetx as hnx
import matplotlib.pyplot as plt

def compute_dual_hypergraph(hyperedges):
    """Compute the dual hypergraph from the provided hyperedges."""
    if not hyperedges:
        return {}

    # In the dual, original edges become nodes, and original nodes become hyperedges
    dual_hyperedges = {}
    all_nodes = set().union(*[data["nodes"] for data in hyperedges.values()])  # All unique nodes from original hypergraph

    # For each node in the original hypergraph, create a hyperedge in the dual
    # containing all original edges that include this node
    for node in all_nodes:
        dual_edges = set()
        for edge_name, edge_data in hyperedges.items():
            if node in edge_data["nodes"]:
                dual_edges.add(edge_name)
        dual_hyperedges[node] = dual_edges
        
    return dual_hyperedges

def create_dual_hypergraph(hyperedges=None):
    """Create a dual hypergraph using provided hyperedges (or None for default behavior)."""
    if hyperedges is None:
        return None, None

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
    plt.title("DualHypergraph Visualization")
    return fig