import hypernetx as hnx
import matplotlib
import matplotlib.pyplot as plt
import json
import numpy as np
import networkx as nx
from matplotlib.patches import PathPatch, Path, Ellipse, Circle
from matplotlib.path import Path

JSON_FILE_PATH = "/Users/gigin/Documents/GitHub/HG-db/hgdb_core/hg_app/py_scripts/json-data/test_edge.json"

def load_hyperedges_from_json(json_file_path):
    with open(json_file_path, 'r') as file:
        data = json.load(file)
    
    # Sort edges by size for default layer assignment
    edge_sizes = [(edge["id"], len(edge["head_hyper_nodes"]) + (len(edge["tail_hyper_nodes"]) if edge["tail_hyper_nodes"] else 0)) for edge in data]
    edge_sizes.sort(key=lambda x: (-x[1], x[0]))  # Sort by size (descending), then by ID
    num_edges = len(edge_sizes)
    layer_thresholds = [num_edges // 3, 2 * num_edges // 3]  # Split into three groups

    hyperedges = {}
    for idx, edge in enumerate(data):
        edge_id = edge["id"]
        head_nodes = [node["id"] for node in edge["head_hyper_nodes"]]
        tail_nodes = [node["id"] for node in edge["tail_hyper_nodes"]] if edge["tail_hyper_nodes"] else []
        
        # Assign layer based on the 'layer' key, or fall back to size-based grouping
        if "layer" in edge:
            layer = edge["layer"]
        else:
            # Find the edge's position in the sorted list
            edge_pos = next(i for i, (eid, _) in enumerate(edge_sizes) if eid == edge_id)
            if edge_pos < layer_thresholds[0]:
                layer = 0  # Top layer
            elif edge_pos < layer_thresholds[1]:
                layer = 1  # Middle layer
            else:
                layer = 2  # Lower layer

        hyperedges[edge_id] = {
            "nodes": set(head_nodes + tail_nodes),
            "head": set(head_nodes),
            "tail": set(tail_nodes),
            "traversable": edge["traversable"],
            "directed": edge["directed"],
            "type": edge["main_properties"][0]["value"][0],
            "layer": layer  # Add layer attribute
        }
    return hyperedges

def create_hypergraph():
    hyperedges = load_hyperedges_from_json(JSON_FILE_PATH)
    if not hyperedges:
        return None, None
    H = hnx.Hypergraph({k: v["nodes"] for k, v in hyperedges.items()})
    return H, hyperedges

def draw_hypergraph(H, hyperedges, visualize_mode="edges"):
    if H is None or not hyperedges:
        return None

    fig, ax = plt.subplots(figsize=(10, 8))
    
    # Convert Hypergraph to bipartite NetworkX graph for layout
    G = nx.Graph()
    for node in H.nodes:
        G.add_node(node, bipartite=0)
    for edge_name in H.edges:
        G.add_node(edge_name, bipartite=1)
        for node in hyperedges[edge_name]["nodes"]:
            G.add_edge(node, edge_name)
    
    # Use NetworkX spring_layout with adjusted parameters to spread nodes
    pos = nx.spring_layout(G, scale=2.0, k=0.5, iterations=50, seed=42)  # Added seed for consistent layout
    node_pos = {node: coord for node, coord in pos.items() if node in H.nodes}
    
    # Generate colors dynamically using a continuous colormap
    num_edges = len(hyperedges)
    colormap = matplotlib.colormaps.get_cmap('rainbow')  # Use 'rainbow' for a continuous range of colors
    edge_colors = {edge_name: colormap(i / num_edges) for i, edge_name in enumerate(hyperedges.keys())}

    # Draw hyperedges as colored regions (always ellipses or circles)
    for edge_name, edge_data in hyperedges.items():
        traversable = edge_data["traversable"]
        all_nodes = edge_data["nodes"]

        should_draw = (visualize_mode == "edges" and traversable) or (visualize_mode == "nodes" and not traversable)

        if should_draw:
            # Draw a blob around the nodes (ignoring directed field)
            node_list = list(all_nodes)
            if node_list and all(n in node_pos for n in node_list):
                # Calculate points for the blob
                points = np.array([node_pos[n] for n in node_list])
                if len(points) == 1:
                    # Single node: Draw a circle
                    circle = Circle(points[0], 0.3, color=edge_colors.get(edge_name, "gray"), alpha=0.5)
                    ax.add_patch(circle)
                else:
                    # Multiple nodes: Draw an ellipse encompassing all nodes
                    center = np.mean(points, axis=0)
                    # Calculate the width and height of the ellipse
                    max_dist = max(np.linalg.norm(p - center) for p in points)
                    width = max_dist * 2.5  # Scale to make the ellipse larger
                    height = width * 0.6  # Adjust height to make it more elliptical
                    # Calculate the angle of the ellipse based on the principal direction
                    if len(points) >= 2:
                        p1, p2 = points[0], points[1]  # Use first two points to determine angle
                        angle = np.degrees(np.arctan2(p2[1] - p1[1], p2[0] - p1[0]))
                    else:
                        angle = 0
                    ellipse = Ellipse(center, width, height, angle=angle,
                                     facecolor=edge_colors.get(edge_name, "gray"), alpha=0.5, edgecolor="none")
                    ax.add_patch(ellipse)
                # Label the edge near the centroid
                centroid_x = np.mean([node_pos[n][0] for n in node_list])
                centroid_y = np.mean([node_pos[n][1] for n in node_list])
                ax.text(centroid_x, centroid_y - 0.2, edge_name, fontsize=10, color="black", ha="center")

    # Draw nodes
    for node, (x, y) in node_pos.items():
        ax.scatter(x, y, s=100, color="black")
        ax.text(x, y + 0.05, f"x{node}", fontsize=10, ha="center", va="bottom")

    ax.set_title("Hypergraph")
    ax.axis("off")
    plt.tight_layout()
    return fig

# -----------------------------------------------------------------------------------------------------
# import hypernetx as hnx
# import matplotlib.pyplot as plt
# import json
# import os
# import numpy as np
# import networkx as nx

# JSON_FILE_PATH = "py_scripts/json-data/test_edge.json"

# def load_hyperedges_from_json(json_file_path):
#     with open(json_file_path, 'r') as file:
#         data = json.load(file)
    
#     hyperedges = {}  # This will store your hyperedges data
#     for edge in data:  # Directly iterate over the list
#         edge_id = edge["id"]
#         # You can then extract relevant information from each edge
#         hyperedges[edge_id] = {
#             "nodes": set(edge["head_hyper_nodes"] + (edge.get("tail_hyper_nodes") or [])),  # Collect head and tail nodes
#             "head": set(edge["head_hyper_nodes"]),  # Add head nodes
#             "tail": set(edge.get("tail_hyper_nodes") or []),  # Add tail nodes
#             "traversable": edge["traversable"],
#             "directed": edge["directed"],
#             "type": edge["main_properties"][0]["value"][0],  # Assuming the first property is the type
#         }

#     return hyperedges

# def create_hypergraph():
#     """Create a hypergraph using dynamically loaded data from JSON."""
#     hyperedges = load_hyperedges_from_json(JSON_FILE_PATH)
#     if not hyperedges:
#         return None, None
#     H = hnx.Hypergraph({k: v["nodes"] for k, v in hyperedges.items()})
#     return H, hyperedges


# def draw_hypergraph(H, hyperedges, visualize_mode="edges"):
#     """Draw the hypergraph with custom handling for directed edges and traversable filter."""
#     if H is None or not hyperedges:
#         return None

#     fig, ax = plt.subplots(figsize=(12, 10))
    
#     # Convert Hypergraph to bipartite NetworkX graph for layout
#     G = nx.Graph()
#     for node in H.nodes:
#         G.add_node(node, bipartite=0)  # Nodes
#     for edge_name in H.edges:
#         G.add_node(edge_name, bipartite=1)  # Edges
#         for node in hyperedges[edge_name]["nodes"]:
#             G.add_edge(node, edge_name)
    
#     # Use NetworkX spring_layout with adjusted parameters
#     pos = nx.spring_layout(G, scale=2.0, k=0.5, iterations=50)
#     # Extract only node positions
#     node_pos = {node: coord for node, coord in pos.items() if node in H.nodes}
    
#     # Draw nodes (always visible)
#     for node, (x, y) in node_pos.items():
#         ax.scatter(x, y, s=100, color="black")
#         ax.text(x, y + 0.05, node, fontsize=10, ha="center", va="bottom")

#     # Filter and draw based on visualize_mode
#     for edge_name, edge_data in hyperedges.items():
#         traversable = edge_data["traversable"]
#         directed = edge_data["directed"]
#         head_nodes = edge_data["head"]
#         tail_nodes = edge_data["tail"]
#         all_nodes = edge_data["nodes"]

#         should_draw = (visualize_mode == "edges" and traversable) or (visualize_mode == "nodes" and not traversable)

#         # Show edges where traversable matches the mode
#         if should_draw:
#             if directed and head_nodes and tail_nodes:
#                 # Directed: Draw arrows from head to tail
#                 for head in head_nodes:
#                     for tail in tail_nodes:
#                         if head in node_pos and tail in node_pos:
#                             ax.annotate("", 
#                                        xy=node_pos[tail], xytext=node_pos[head],
#                                        arrowprops=dict(arrowstyle="->", color="blue", lw=2))
#                             mid_x = (node_pos[head][0] + node_pos[tail][0]) / 2
#                             mid_y = (node_pos[head][1] + node_pos[tail][1]) / 2
#                             ax.text(mid_x, mid_y, edge_name, fontsize=10, color="blue", ha="center")
#             else:
#                 # Undirected: Draw lines between all node pairs in the edge
#                 node_list = list(all_nodes)
#                 for i in range(len(node_list)):
#                     for j in range(i + 1, len(node_list)):
#                         n1, n2 = node_list[i], node_list[j]
#                         if n1 in node_pos and n2 in node_pos:
#                             ax.plot([node_pos[n1][0], node_pos[n2][0]], 
#                                     [node_pos[n1][1], node_pos[n2][1]], 
#                                     color="gray", linestyle="-", lw=2)
#                 # Label edge near centroid
#                 if node_list and all(n in node_pos for n in node_list):
#                     centroid_x = np.mean([node_pos[n][0] for n in node_list])
#                     centroid_y = np.mean([node_pos[n][1] for n in node_list])
#                     ax.text(centroid_x, centroid_y, edge_name, fontsize=10, color="gray", ha="center")

#     ax.set_title(f"Hypergraph Visualization ({visualize_mode.capitalize()})")
#     ax.axis("off")
#     plt.tight_layout()
#     return fig