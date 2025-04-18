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
    """Draw the dual hypergraph with a layout matching the Hypergraph style (colored ellipses)."""
    if H_dual is None:
        return None

    fig, ax = plt.subplots(figsize=(10, 8))

    # Extract nodes (original edges) and hyperedges (original nodes)
        # Extract nodes (original edges) and hyperedges (original nodes)
    nodes = list(H_dual.nodes)  # These are the original edges (e1, e2, ...)
    hyperedges = {hyperedge: set(H_dual.incidence_dict[hyperedge]) for hyperedge in H_dual.edges}  # Original nodes (v3, v4, ...)

    # Create a NetworkX graph for layout purposes
    import networkx as nx
    G = nx.Graph()
    for node in nodes:
        G.add_node(node)
    for node_set in hyperedges.values():
        node_list = list(node_set)
        for i in range(len(node_list)):
            for j in range(i + 1, len(node_list)):
                G.add_edge(node_list[i], node_list[j])

    # Use NetworkX spring_layout with parameters matching the Hypergraph
    pos = nx.kamada_kawai_layout(G, scale=2.0)

    # Use your custom pastel colors
    custom_colors = ['#c49ffc', '#f294d9', '#6fc5ed', '#a5f0a8', '#fcc09f']
    # Assign colors to dual hyperedges (original nodes)
    hyperedge_colors = {
        hyperedge: custom_colors[i % len(custom_colors)]
        for i, hyperedge in enumerate(H_dual.edges)
    }

    # Draw hyperedges as colored ellipses (representing original nodes)
    from matplotlib.patches import Circle, Ellipse
    import numpy as np

    for hyperedge, node_set in hyperedges.items():
        if not node_set:
            continue
        # Calculate points for the ellipse
        points = np.array([pos[node] for node in node_set])
        if len(points) == 1:
            # Single node: Draw a circle
            circle = Circle(points[0], 0.2, color=hyperedge_colors.get(hyperedge, "gray"), alpha=0.7)
            ax.add_patch(circle)
        else:
            # Multiple nodes: Draw an ellipse encompassing all nodes
            center = np.mean(points, axis=0)
            max_dist = max(np.linalg.norm(p - center) for p in points)
            width = max_dist * 2.8  # Scale to make the ellipse larger
            height = width * 0.8  # Adjust height to make it more elliptical
            if len(points) >= 2:
                p1, p2 = points[0], points[1]  # Use first two points to determine angle
                angle = np.degrees(np.arctan2(p2[1] - p1[1], p2[0] - p1[0]))
            else:
                angle = 0
            ellipse = Ellipse(center, width, height, angle=angle,
                             facecolor=hyperedge_colors.get(hyperedge, "gray"), alpha=0.6, edgecolor="none")
            ax.add_patch(ellipse)
    # Draw nodes (original edges)
    for node in nodes:
        x, y = pos[node]
        ax.scatter(x, y, s=50, color="black")
        ax.text(x, y + 0.05, node, fontsize=10, ha="center", va="bottom")

    ax.set_title("DualHypergraph Visualization")
    ax.axis("off")
    
    # Create legend patches
    legend_patches = []
    for hyperedge, color in hyperedge_colors.items():
        legend_patches.append(plt.Line2D([0], [0], 
                                      marker='o', 
                                      color='w', 
                                      label=hyperedge,
                                      markerfacecolor=color, 
                                      markersize=10,
                                      alpha=0.5))

    # Add legend to the plot
    ax.legend(handles=legend_patches, 
         title="Dual Hyperedges",
         loc='upper right',
         bbox_to_anchor=(1.4, 1),    
         prop={'size': 15},           
         title_fontsize='13',         
         framealpha=0.7,
         markerscale=1.8)   
    
    plt.tight_layout()
    return fig


# def draw_dual_hypergraph(H_dual):
#     """Draw the dual hypergraph using HyperNetX and return the figure."""
#     if H_dual is None:
#         return None
    
#     fig, ax = plt.subplots(figsize=(12, 10))
#     hnx.draw(H_dual, with_node_labels=True, with_edge_labels=True, ax=ax)
#     plt.title("DualHypergraph Visualization")
#     return fig