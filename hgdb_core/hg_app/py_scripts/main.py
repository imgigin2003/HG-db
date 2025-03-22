import streamlit as st
from hypergraph import create_hypergraph, draw_hypergraph
from layered_hypergraph import draw_layered_hypergraph
from dual_hypergraph import create_dual_hypergraph, draw_dual_hypergraph
import hypernetx as hnx
import pandas as pd
import json

def main():
    if "hyperedges" not in st.session_state:
        H, hyperedges = create_hypergraph()
        st.session_state.hyperedges = hyperedges if hyperedges else {}

    st.title("Interactive Hypergraph")
    tabs = st.tabs([
        "HyperGraph Visualization", 
        "Layered HyperGraph Visualization",
        "Dual HyperGraph Visualization",
        "Graph Properties", 
        "Graph Table", 
        "Graph Edit"
    ])

    with tabs[0]:
        st.write("### HyperGraph Visualization")
        H = hnx.Hypergraph({k: v["nodes"] for k, v in st.session_state.hyperedges.items()})
        visualize_mode = st.radio(
            "Visualize based on traversable:",
            options=["edges", "nodes"],
            format_func=lambda x: f"{'Edges' if x == 'edges' else 'Nodes'} (traversable: {x == 'edges'})"
        )
        if st.button("Visualize HyperGraph✨"):
            fig = draw_hypergraph(H, st.session_state.hyperedges, visualize_mode)
            if fig:
                st.pyplot(fig)

    with tabs[1]:
        st.write("### Layered HyperGraph Visualization")
        if st.button("Visualize Layered HyperGraph✨"):
            fig = draw_layered_hypergraph(st.session_state.hyperedges)  # Pass the full hyperedges dictionary
            if fig:
                st.pyplot(fig)

    with tabs[2]:
        st.write("### Dual HyperGraph Visualization")
        H_dual, _ = create_dual_hypergraph(st.session_state.hyperedges)
        if st.button("Visualize Dual HyperGraph✨"):
            fig = draw_dual_hypergraph(H_dual)
            if fig:
                st.pyplot(fig)

    with tabs[3]:
        st.write("### Graphs Properties")
        H = hnx.Hypergraph({k: v["nodes"] for k, v in st.session_state.hyperedges.items()})
        display_properties(H, "HyperGraph")
        H_dual, _ = create_dual_hypergraph(st.session_state.hyperedges)
        display_properties(H_dual, "Dual HyperGraph")

    with tabs[4]:
        st.write("### Graphs Tables")
        display_table(st.session_state.hyperedges, "HyperGraph")
        H_dual, dual_hyperedges = create_dual_hypergraph(st.session_state.hyperedges)
        display_table(dual_hyperedges, "Dual HyperGraph")

    with tabs[5]:
        st.write("### Graph Edit")
        edit_sub_tabs = st.tabs(["Add Hyperedge", "Edit Hyperedge", "Delete Hyperedge"])

        with edit_sub_tabs[0]:
            st.write("#### Add Hyperedge")
            new_edge_id = st.text_input("Enter new edge ID (e.g., e8):", key="new_edge_id")
            new_nodes = st.text_input("Enter nodes for the new edge (comma-separated):", key="new_nodes")
            new_layer = st.selectbox("Select layer for the new edge:", options=["Top", "Middle", "Lower"], key="new_layer")
            is_traverse = st.checkbox("Traversable", value=False, key="traverse")
            if st.button("Add Hyperedge"):
                if new_edge_id and new_nodes:
                    nodes_set = set(new_nodes.split(","))
                    if new_edge_id in st.session_state.hyperedges:
                        st.warning("Edge ID already exists!")
                    else:
                        layer_map = {"Top": 0, "Middle": 1, "Lower": 2}
                        st.session_state.hyperedges[new_edge_id] = {
                            "nodes": nodes_set,
                            "head": nodes_set,
                            "tail": set(),
                            "traversable": is_traverse,
                            "type": "linked",
                            "layer": layer_map[new_layer]  # Add layer attribute
                        }
                        st.success(f"Hyperedge '{new_edge_id}' added with nodes {nodes_set} in {new_layer} layer!")

        with edit_sub_tabs[1]:
            st.write("### Edit Hyperedge")
            edge_to_edit = st.selectbox("Select an edge to edit:", options=list(st.session_state.hyperedges.keys()))
            edited_nodes = st.text_input("Enter new nodes for the selected edge (comma-separated):", key="edit_nodes")
            edited_layer = st.selectbox("Select new layer for the selected edge:", options=["Top", "Middle", "Lower"], key="edit_layer")
            if st.button("Edit Hyperedge"):
                if edge_to_edit and edited_nodes:
                    new_nodes_set = set(edited_nodes.split(","))
                    layer_map = {"Top": 0, "Middle": 1, "Lower": 2}
                    st.session_state.hyperedges[edge_to_edit]["nodes"] = new_nodes_set
                    st.session_state.hyperedges[edge_to_edit]["head"] = new_nodes_set
                    st.session_state.hyperedges[edge_to_edit]["tail"] = set()
                    st.session_state.hyperedges[edge_to_edit]["layer"] = layer_map[edited_layer]  # Update layer
                    st.success(f"Hyperedge '{edge_to_edit}' updated with nodes '{new_nodes_set}' in {edited_layer} layer!")

        with edit_sub_tabs[2]:
            st.write("### Delete Hyperedge")
            edges_to_delete = st.selectbox("Select an edge to delete:", options=list(st.session_state.hyperedges.keys()))
            if st.button("Delete Hyperedge"):
                if edges_to_delete:
                    del st.session_state.hyperedges[edges_to_delete]
                    st.success(f"Hyperedge '{edges_to_delete}' deleted successfully.")

def display_properties(H, graph_type):
    st.write(f"### {graph_type} Properties")
    nodes_list = list(H.nodes())
    edges_list = list(H.edges())
    st.write(f"Nodes: {nodes_list}")
    st.write(f"Number of nodes: {len(nodes_list)}")
    st.write(f"Edges: {edges_list}")
    st.write(f"Number of edges: {len(edges_list)}")

def display_table(graph, graph_type):
    st.write(f"### {graph_type} Table")
    edge_data = []
    if graph_type == "HyperGraph":
        edge_data = [
            {"Edge": edge, "Nodes": ", ".join(sorted(data["nodes"])), 
            "Traversable": data["traversable"]}
            for edge, data in graph.items()
        ]
    else:  # Dual HyperGraph
        edge_data = [
            {"Node (Original Edge)": node, "Hyperedges (Original Nodes)": ", ".join(sorted(edges))}
            for node, edges in graph.items()
        ]
    df = pd.DataFrame(edge_data)
    st.dataframe(df)

if __name__ == "__main__":
    main()