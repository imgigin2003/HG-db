import hypernetx as hnx
from pathlib import Path
import matplotlib.pyplot as plt
import json
import numpy as np
import networkx as nx
from matplotlib.patches import Ellipse, Circle

def data_root():
    """Helper Function to get the project root relative to main.py's location"""
    return Path(__file__).parent

JSON_FILE_PATH = data_root() / "json-data" / "test_edge.json"

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

def draw_hypergraph(H, hyperedges, visualize_mode="edges", highlighted_path=None, display_mode="default"):
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
    pos = nx.spring_layout(G, k=0.3, iterations=50, scale=2.0)
    node_pos = {node: coord for node, coord in pos.items() if node in H.nodes}

    # Generate colors using a cyclic colormap for edges
    custom_colors = ['#c49ffc', '#f294d9', '#6fc5ed', '#a5f0a8', '#fcc09f']
    edge_colors = {
        edge_name: custom_colors[i % len(custom_colors)]
        for i, edge_name in enumerate(hyperedges.keys())
    }

    # Override colors for highlighted path
    if highlighted_path:
        path_color = highlighted_path.get("color", "gray")
        for edge_name in highlighted_path.get("hyperedges", []):
            if edge_name in edge_colors:
                edge_colors[edge_name] = path_color

    if display_mode == "default":
        # Draw hyperedges as colored regions in default mode
        for edge_name, edge_data in hyperedges.items():
            traversable = edge_data["traversable"]
            all_nodes = edge_data["nodes"]

            should_draw = (visualize_mode == "edges" and traversable) or (visualize_mode == "nodes" and not traversable)

            if should_draw:
                node_list = list(all_nodes)
                if node_list and all(n in node_pos for n in node_list):
                    points = np.array([node_pos[n] for n in node_list])
                    if len(points) == 1:
                        circle = Circle(points[0], 0.3, color=edge_colors.get(edge_name, "gray"), alpha=0.6)
                        ax.add_patch(circle)
                    else:
                        center = np.mean(points, axis=0)
                        max_dist = max(np.linalg.norm(p - center) for p in points)
                        width = max_dist * 2.5
                        height = width * 0.6
                        if len(points) >= 2:
                            p1, p2 = points[0], points[1]
                            angle = np.degrees(np.arctan2(p2[1] - p1[1], p2[0] - p1[0]))
                        else:
                            angle = 0
                        ellipse = Ellipse(center, width, height, angle=angle,
                                         facecolor=edge_colors.get(edge_name, "gray"), alpha=0.6, edgecolor="none")
                        ax.add_patch(ellipse)

        # Draw nodes (always)
        for node, (x, y) in node_pos.items():
            ax.scatter(x, y, s=100, color="black")
            ax.text(x, y + 0.05, node, fontsize=10, ha="center", va="bottom")

        # Adding legend patch top right corner for default mode
        legend_patches = []
        for edge_name, color in edge_colors.items():
            legend_patches.append(plt.Line2D([0], [0], 
                                            marker='o', 
                                            color='w', 
                                            label=edge_name,
                                            markerfacecolor=color, 
                                            markersize=10,
                                            alpha=0.5))

        ax.legend(handles=legend_patches, 
            title="Hyperedges",
            loc='upper right',
            bbox_to_anchor=(1.35, 1),  
            prop={'size': 15},          
            title_fontsize='12',         
            framealpha=0.7,
            markerscale=1.5)
        
    
    elif display_mode == "simple":
        # Simple mode: Draw an ellipse outline (no fill) around nodes of each hyperedge
        for edge_name, edge_data in hyperedges.items():
            traversable = edge_data["traversable"]
            all_nodes = edge_data["nodes"]

            should_draw = (visualize_mode == "edges" and traversable) or (visualize_mode == "nodes" and not traversable)

            if should_draw:
                node_list = list(all_nodes)
                if node_list and all(n in node_pos for n in node_list):
                    points = np.array([node_pos[n] for n in node_list])
                    if len(points) >= 2:  # Need at least 2 nodes to draw an ellipse
                        # Compute the centroid of the nodes
                        center = np.mean(points, axis=0)
                        # Calculate the maximum distance from the centroid to any node
                        max_dist = max(np.linalg.norm(p - center) for p in points)
                        # Set ellipse dimensions to enclose all nodes
                        width = max_dist * 2.5
                        height = width * 0.6
                        # Calculate the angle of the ellipse based on the first two nodes
                        if len(points) >= 2:
                            p1, p2 = points[0], points[1]
                            angle = np.degrees(np.arctan2(p2[1] - p1[1], p2[0] - p1[0]))
                        else:
                            angle = 0
                        # Draw the ellipse outline (no fill)
                        ellipse = Ellipse(center, width, height, angle=angle,
                                         fill=False, edgecolor=edge_colors.get(edge_name, "gray"), linewidth=2)
                        ax.add_patch(ellipse)
                        # Label the hyperedge at the centroid
                        ax.text(center[0], center[1], edge_name, fontsize=10, color=edge_colors.get(edge_name, "gray"),
                                ha="center", va="center", bbox=dict(facecolor="white", alpha=0.7, edgecolor="none"))
                    elif len(points) == 1:
                        # For a single node, draw a small circle outline
                        circle = Circle(points[0], 0.3, fill=False, color=edge_colors.get(edge_name, "gray"), linewidth=2)
                        ax.add_patch(circle)
                        # Label at the center
                        ax.text(points[0][0], points[0][1], edge_name, fontsize=10, color=edge_colors.get(edge_name, "gray"),
                                ha="center", va="center", bbox=dict(facecolor="white", alpha=0.7, edgecolor="none"))
                        
        # Draw nodes (always)
        for node, (x, y) in node_pos.items():
            ax.scatter(x, y, s=100, color="black")
            ax.text(x, y + 0.05, node, fontsize=10, ha="center", va="bottom")


    elif display_mode == "minimal":
        # Minimal mode: Use hypernetx default drawing with node and edge labels, reading from JSON
        hnx.draw(H, ax=ax, with_node_labels=True, with_edge_labels=True)
        # Apply custom colors to edges if possible (limited by hypernetx)
        for edge_name, edge_data in hyperedges.items():
            traversable = edge_data["traversable"]
            all_nodes = edge_data["nodes"]
            should_draw = (visualize_mode == "edges" and traversable) or (visualize_mode == "nodes" and not traversable)
            if should_draw:
                node_list = list(all_nodes)
                if node_list and all(n in node_pos for n in node_list):
                    points = np.array([node_pos[n] for n in node_list])
                    center = np.mean(points, axis=0)
 

    ax.set_title("Hypergraph")
    ax.axis("off")
    
    plt.tight_layout()
    return fig