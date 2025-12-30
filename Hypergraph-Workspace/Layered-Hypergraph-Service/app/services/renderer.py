from collections import defaultdict

def render_layered_hypergraph(layers):
    """
    Converts layered edges into a 3D hypergraph structure.
    Stateless: does NOT store anything.
    """

    edges = {}
    nodes = set()
    node_to_layers = defaultdict(set)

    for layer_id, layer_edges in layers.items():
        layer_int = int(layer_id)
        for edge_id, edge_nodes in layer_edges.items():
            edges[edge_id] = {
                "nodes": edge_nodes,
                "layer": layer_int
            }
            nodes.update(edge_nodes)
            for node in edge_nodes:
                node_to_layers[node].add(layer_int)

    node_to_layers_dict = {
        node: list(layers_set)
        for node, layers_set in node_to_layers.items()
    }

    layered = defaultdict(dict)
    for edge_id, info in edges.items():
        layer_id = str(info["layer"])
        layered[layer_id][edge_id] = info["nodes"]

    return {
        "scene_type": "layered-hypergraph",
        "dimension": "3D",
        "nodes": list(nodes),
        "edges": edges,
        "layers": dict(layered),
        "node_to_layers": node_to_layers_dict
    }
