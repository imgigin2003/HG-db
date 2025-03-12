import hypernetx as hnx
import matplotlib.pyplot as plt
import json
import os
import numpy as np
import networkx as nx

JSON_FILE_PATH = "py_scripts/json-data/test_edge.json"

def load_hyperedges_from_json(json_file_path):
    with open(json_file_path, 'r') as file:
        data = json.load(file)
    
    hyperedges = {}  # This will store your hyperedges data
    for edge in data:  # Directly iterate over the list
        edge_id = edge["id"]
        # You can then extract relevant information from each edge
        hyperedges[edge_id] = {
            "nodes": set(edge["head_hyper_nodes"] + (edge.get("tail_hyper_nodes") or [])),  # Collect head and tail nodes
            "head": set(edge["head_hyper_nodes"]),  # Add head nodes
            "tail": set(edge.get("tail_hyper_nodes") or []),  # Add tail nodes
            "traversable": edge["traversable"],
            "directed": edge["directed"],
            "type": edge["main_properties"][0]["value"][0],  # Assuming the first property is the type
        }

    return hyperedges

def create_hypergraph():
    """Create a hypergraph using dynamically loaded data from JSON."""
    hyperedges = load_hyperedges_from_json(JSON_FILE_PATH)
    if not hyperedges:
        return None, None
    H = hnx.Hypergraph({k: v["nodes"] for k, v in hyperedges.items()})
    return H, hyperedges


def draw_hypergraph(H, hyperedges, visualize_mode="edges"):
    """Draw the hypergraph with custom handling for directed edges and traversable filter."""
    if H is None or not hyperedges:
        return None

    fig, ax = plt.subplots(figsize=(12, 10))
    
    # Convert Hypergraph to bipartite NetworkX graph for layout
    G = nx.Graph()
    for node in H.nodes:
        G.add_node(node, bipartite=0)  # Nodes
    for edge_name in H.edges:
        G.add_node(edge_name, bipartite=1)  # Edges
        for node in hyperedges[edge_name]["nodes"]:
            G.add_edge(node, edge_name)
    
    # Use NetworkX spring_layout with adjusted parameters
    pos = nx.spring_layout(G, scale=2.0, k=0.5, iterations=50)
    # Extract only node positions
    node_pos = {node: coord for node, coord in pos.items() if node in H.nodes}
    
    # Draw nodes (always visible)
    for node, (x, y) in node_pos.items():
        ax.scatter(x, y, s=100, color="black")
        ax.text(x, y + 0.05, node, fontsize=10, ha="center", va="bottom")

    # Filter and draw based on visualize_mode
    for edge_name, edge_data in hyperedges.items():
        traversable = edge_data["traversable"]
        directed = edge_data["directed"]
        head_nodes = edge_data["head"]
        tail_nodes = edge_data["tail"]
        all_nodes = edge_data["nodes"]

        should_draw = (visualize_mode == "edges" and traversable) or (visualize_mode == "nodes" and not traversable)

        # Show edges where traversable matches the mode
        if should_draw:
            if directed and head_nodes and tail_nodes:
                # Directed: Draw arrows from head to tail
                for head in head_nodes:
                    for tail in tail_nodes:
                        if head in node_pos and tail in node_pos:
                            ax.annotate("", 
                                       xy=node_pos[tail], xytext=node_pos[head],
                                       arrowprops=dict(arrowstyle="->", color="blue", lw=2))
                            mid_x = (node_pos[head][0] + node_pos[tail][0]) / 2
                            mid_y = (node_pos[head][1] + node_pos[tail][1]) / 2
                            ax.text(mid_x, mid_y, edge_name, fontsize=10, color="blue", ha="center")
            else:
                # Undirected: Draw lines between all node pairs in the edge
                node_list = list(all_nodes)
                for i in range(len(node_list)):
                    for j in range(i + 1, len(node_list)):
                        n1, n2 = node_list[i], node_list[j]
                        if n1 in node_pos and n2 in node_pos:
                            ax.plot([node_pos[n1][0], node_pos[n2][0]], 
                                    [node_pos[n1][1], node_pos[n2][1]], 
                                    color="gray", linestyle="-", lw=2)
                # Label edge near centroid
                if node_list and all(n in node_pos for n in node_list):
                    centroid_x = np.mean([node_pos[n][0] for n in node_list])
                    centroid_y = np.mean([node_pos[n][1] for n in node_list])
                    ax.text(centroid_x, centroid_y, edge_name, fontsize=10, color="gray", ha="center")

    ax.set_title(f"Hypergraph Visualization ({visualize_mode.capitalize()})")
    ax.axis("off")
    plt.tight_layout()
    return fig