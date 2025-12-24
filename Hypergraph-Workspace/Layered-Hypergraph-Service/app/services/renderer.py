def render_layered_hypergraph(hyperedges: dict):
    nodes = set()

    for edge_name, edge in hyperedges.items():
        for node in edge.nodes:
            nodes.add(node)

    return {
        "scene_type": "layered-hypergraph",
        "dimension": "3D",
        "nodes": list(nodes),
        "edges": {
            name: {
                "nodes": edge.nodes,
                "layer": edge.layer
            }
            for name, edge in hyperedges.items()
        }
    }
