import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d.art3d import Poly3DCollection
import numpy as np
from matplotlib.patches import Ellipse

def draw_layered_hypergraph(hyperedges):
    """Draw a layered hypergraph in 3D with grouped hyperedges and dynamic inter-layer connections."""
    if not hyperedges:
        return None

    fig = plt.figure(figsize=(16, 12))
    ax = fig.add_subplot(111, projection='3d')

    # Dynamically determine all layers present in the data
    all_layers = set()
    for edge_data in hyperedges.values():
        all_layers.add(edge_data.get("layer", 0)) # Default layer set to 0 (lowest)

    if not all_layers: # if no layers specified at all
        all_layers = {0}

    num_layers = len(all_layers)
    sorted_layers = sorted(all_layers)

    # Create layer groups
    layer_groups = {layer: [] for layer in sorted_layers}
    for edge_name, edge_data in hyperedges.items():
        layer = edge_data.get("layer", sorted_layers[-1]) # Default to last layer if not specified

        if layer in layer_groups:
            layer_groups[layer].append(edge_name)

    # Node positions: (node, layer) -> (x, y, z)
    node_positions = {}
    layer_nodes = {layer: set() for layer in sorted_layers}

    layer_names = [f'Layer {i+1}' for i in range(num_layers)]
    custom_colors = ['#c49ffc', '#f294d9', '#6fc5ed', '#a5f0a8', '#fcc09f']
    layer_colors = [custom_colors[i % len(custom_colors)] for i in range(num_layers)]
    
    # Handle single-layer case to avoid division by zero
    if num_layers == 1:
        layer_z_mapping = {layer: 0.0 for layer in sorted_layers}
        max_z = 0.0
    else:
        max_z = (num_layers - 1) * 2.0
        layer_z_mapping = {layer: (num_layers - 1 - idx) * (max_z / (num_layers - 1)) 
                          for idx, layer in enumerate(sorted_layers)}

    # Position nodes in each layer
    for layer, edges in layer_groups.items():
        if not edges:
            continue
        # Collect all nodes in this layer
        nodes_in_layer = set()
        for edge in edges:
            nodes_in_layer.update(hyperedges[edge]["nodes"])
        layer_nodes[layer] = nodes_in_layer

        # Create a NetworkX graph for this layer to compute a 2D layout
        import networkx as nx
        G = nx.Graph()
        for node in nodes_in_layer:
            G.add_node(node)
        for edge in edges:
            nodes = list(hyperedges[edge]["nodes"])
            for i in range(len(nodes)):
                for j in range(i + 1, len(nodes)):
                    G.add_edge(nodes[i], nodes[j])

        # Compute 2D positions using spring layout with adjusted parameters
        pos_2d = nx.kamada_kawai_layout(G, scale=1.5)

        # Manually adjust positions for single-node hyperedges to avoid overlap
        edge_positions = {}  # Track the center position of each hyperedge
        for edge in edges:
            nodes = list(hyperedges[edge]["nodes"])
            if len(nodes) == 1:
                # Single-node hyperedge: Adjust position to avoid overlap
                node = nodes[0]
                base_pos = pos_2d[node]
                # Check for overlap with other single-node hyperedges
                min_dist = 0.5  # Minimum distance between single-node hyperedges
                for other_edge, other_pos in edge_positions.items():
                    dist = np.linalg.norm(np.array(base_pos) - np.array(other_pos))
                    if dist < min_dist:
                        # Offset the position
                        offset = (min_dist - dist) * np.array([1, 0])  # Offset along x-axis
                        base_pos = (base_pos[0] + offset[0], base_pos[1] + offset[1])
                        pos_2d[node] = base_pos
                edge_positions[edge] = base_pos
            else:
                # Multi-node hyperedge: Compute centroid for reference
                edge_pos = np.mean([pos_2d[node] for node in nodes], axis=0)
                edge_positions[edge] = edge_pos

        # Map 2D positions to 3D by adding the z-coordinate for the layer (reversed)
        z = layer_z_mapping[layer]
        for node in nodes_in_layer:
            x, y = pos_2d[node]
            node_positions[(node, layer)] = (x, y, z)

    # Draw layer planes
    for layer in sorted_layers:
        z = layer_z_mapping[layer]
        vertices = [
            (-3, -3, z),
            (3, -3, z),
            (3, 3, z),
            (-3, 3, z)
        ]
        ax.add_collection3d(Poly3DCollection([vertices], alpha=0.4, color=layer_colors[sorted_layers.index(layer)]))

    # Draw hyperedges as ellipses within each layer
    for edge_name, edge_data in hyperedges.items():
        layer = edge_data.get("layer", 2)
        z = layer_z_mapping[layer]
        node_list = list(edge_data["nodes"])
        points = np.array([node_positions[(node, layer)][:2] for node in node_list])  # 2D points (x, y)
        
        # Define size parameters
        size_multiplier = 1.0  # Consistent multiplier across all layers
        min_size = 1.0  # Increased minimum size to match default ellipse size
        min_max_dist = 0.3  # Minimum max_dist to ensure reasonable ellipse size
        base_size = 0.3  # Base size for single-node hyperedges
        
        if len(points) == 1:
            # Single node: Draw a circle with a minimum size
            width = max(base_size * size_multiplier, min_size)
            height = width
            circle = Ellipse(points[0], width, height, zorder=25, color=layer_colors[sorted_layers.index(layer)], alpha=0.7)
            ax.add_patch(circle)
            from mpl_toolkits.mplot3d import art3d
            art3d.pathpatch_2d_to_3d(circle, z=z, zdir="z")
        else:
            # Multiple nodes: Draw an ellipse with a minimum size
            center = np.mean(points, axis=0)
            max_dist = max(np.linalg.norm(p - center) for p in points)
            # Apply minimum max_dist
            max_dist = max(max_dist, min_max_dist)
            width = max(max_dist * 2.6 * size_multiplier, min_size)
            height = max(width * 1.2, min_size)
            angle = np.degrees(np.arctan2(points[1][1] - points[0][1], points[1][0] - points[0][0])) if len(points) >= 2 else 0
            ellipse = Ellipse(center, width, height, angle=angle, zorder=25, color=layer_colors[sorted_layers.index(layer)], alpha=0.7)
            ax.add_patch(ellipse)
            from mpl_toolkits.mplot3d import art3d
            art3d.pathpatch_2d_to_3d(ellipse, z=z, zdir="z")

    # Draw nodes and labels with highest zorder
    for (node, layer), (x, y, z) in node_positions.items():
        ax.scatter(x, y, z, color='black', s=60, zorder=10002)
        ax.text(x, y + 0.1, z + 0.15, node, 
                fontsize=10, ha='center', va='bottom',
                bbox=dict(facecolor='none', alpha=0.7, edgecolor='none', pad=1),
                zorder=1000)

    # Draw inter-layer dashed lines between all layers where a node appears
    all_nodes = set()
    for edge_data in hyperedges.values():
        all_nodes.update(edge_data["nodes"])

    for node in all_nodes:
        # Find the layers this node appears in
        node_layers = []
        for layer, edges in layer_groups.items():
            for edge in edges:
                if node in hyperedges[edge]["nodes"]:
                    node_layers.append((layer, edge))
        node_layers.sort()  # Sort by layer

        # Connect the node between all layers where it appears
        for i in range(len(node_layers) - 1):
            layer1, edge1 = node_layers[i]
            layer2, edge2 = node_layers[i + 1]
            pos1 = node_positions[(node, layer1)]
            pos2 = node_positions[(node, layer2)]
            ax.plot([pos1[0], pos2[0]], [pos1[1], pos2[1]], [pos1[2], pos2[2]], 
                    linestyle='--', color='black', alpha=1.0, lw=0.8, zorder=50)

    # Configure axes
    ax.set_xlabel("X-axis")
    ax.set_ylabel("Y-axis")
    ax.set_zlabel("Z-axis (Layers)")
    ax.set_xlim(-3, 3)
    ax.set_ylim(-3, 3)
    ax.set_zlim(-0.5, 5)
    ax.view_init(elev=20, azim=60)
    
    # Create legend patches - works for any number of layers
    layer_names = [f'Layer {i+1}' for i in range(num_layers)]
    legend_patches = []
    for layer_idx in range(num_layers):
        legend_patches.append(plt.Line2D(
            [0], [0],
            marker='o',
            color='w',
            label=layer_names[layer_idx],
            markerfacecolor=layer_colors[layer_idx % len(layer_colors)],  # Cycle through colors
            markersize=10
        ))


    # Add legend to the plot
    ax.legend(handles=legend_patches, 
            title="Layers",
            loc='upper right',
            bbox_to_anchor=(1.15, 1),   
            prop={'size': 15},           
            title_fontsize='14',         
            framealpha=0.7,
            markerscale=2.0)            
    
    plt.tight_layout()
    return fig