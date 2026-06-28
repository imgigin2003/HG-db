import streamlit as st
import hypernetx as hnx
import requests
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d.art3d import Poly3DCollection
import numpy as np
import networkx as nx
from matplotlib.patches import Ellipse, Circle
from pathlib import Path
import pandas as pd
import json
import streamlit.components.v1 as components
from data_loader import DataLoader
from hypergraph import HypergraphManager
from dual_hypergraph import DualHypergraphManager
from layered_hypergraph import LayeredHypergraphManager

# The FastAPI 3D-viewer service base URL – change once if the port ever moves
LAYERED_SERVICE_URL = "http://127.0.0.1:8000"
# The Rust hgdb_core service that normalizes uploaded JSON
RUST_CORE_URL = "http://127.0.0.1:8080"


class VisualizationManager:
    def __init__(self):
        self.data_loader = DataLoader()
        self.hypergraph = HypergraphManager()
        self.dual_hypergraph = DualHypergraphManager()
        self.layered_hypergraph = LayeredHypergraphManager()

    def _build_canonical_edges(self, hyperedges: dict) -> list:
        """
        Serialize the internal hyperedge dict into the canonical edge format
        (the same shape as testing/test.json) that the 3D service consumes.
        """
        edges = []
        for edge_id, data in hyperedges.items():
            head = sorted(data.get("head") or data.get("nodes", set()))
            tail = sorted(data.get("tail") or [])
            edges.append(
                {
                    "id": edge_id,
                    "name": edge_id,
                    "main_properties": [
                        {"key": "type", "p_type": "Simple",
                         "value": [data.get("type", "linked")]}
                    ],
                    "traversable": bool(data.get("traversable", True)),
                    "directed": bool(data.get("directed", False)),
                    "head_hyper_nodes": [{"id": n} for n in head],
                    "tail_hyper_nodes": [{"id": n} for n in tail] or None,
                    "layer": int(data.get("layer", 0)),
                }
            )
        return edges

    def send_to_layered_service(self, edges: list) -> dict:
        """
        POST the canonical edges to the FastAPI 3D service and return a normalised
        result dict:
            { "status": <int>, "scene_id": <str>, "viewer_url": <str>, "body": <json> }
        Raises RuntimeError on non-200 so the caller can fall back gracefully.
        """
        url = f"{LAYERED_SERVICE_URL}/render"
        try:
            response = requests.post(url, json={"edges": edges}, timeout=10)
        except requests.exceptions.ConnectionError:
            raise RuntimeError(
                f"Cannot reach the Layered service at {LAYERED_SERVICE_URL}. "
                "Make sure `uvicorn app.main:app --port 8000` is running."
            )

        if response.status_code != 200:
            raise RuntimeError(
                f"Layered service returned [{response.status_code}]: {response.text}"
            )

        body = response.json()
        return {
            "status": response.status_code,
            "scene_id": body.get("scene_id", ""),
            "viewer_url": body.get("viewer_url", f"{LAYERED_SERVICE_URL}/viewer"),
            "body": body,
        }

    # ──────────────────────────────────────────────────────────────────────────
    def display_hypergraph_visualization(
        self,
        hyperedges,
        graph_type,
        visualize_mode=None,
        is_layered=False,
        is_dual=False,
        tab_key="",
        highlighted_path=None,
        display_mode="default",
    ):
        try:
            if is_dual:
                H, _ = self.dual_hypergraph.create_dual_hypergraph(hyperedges)
            else:
                H = hnx.Hypergraph({k: v["nodes"] for k, v in hyperedges.items()})

            if not st.button(
                f"Visualize {graph_type} ✨", key=f"viz_{tab_key}_{graph_type}"
            ):
                return  # nothing to do until user clicks

            # ── Layered path ──────────────────────────────────────────────────
            if is_layered:
                st.info("⏳ Sending Layered Hypergraph to 3D Service…")

                edges = self._build_canonical_edges(hyperedges)

                try:
                    result = self.send_to_layered_service(edges)
                    viewer_url = result["viewer_url"]

                    st.success("✅ 3D Scene Generated Successfully!")

                    # ── Embed the Three.js viewer inline ──────────────────────
                    # Streamlit's iframe needs an absolute URL that the browser
                    # can reach.  The viewer is served by FastAPI on port 8000.
                    components.iframe(src=viewer_url, height=800, scrolling=True)

                    # ── Debug expanders ───────────────────────────────────────
                    col1, col2 = st.columns(2)
                    with col1:
                        with st.expander("📤 Request payload", expanded=False):
                            st.code(
                                json.dumps({"edges": edges}, indent=2),
                                language="json",
                            )
                    with col2:
                        with st.expander("📥 Service response", expanded=False):
                            st.json(result["body"])

                    # ── Quick-open link as backup ─────────────────────────────
                    st.markdown(
                        f'<a href="{viewer_url}" target="_blank">'
                        '<button style="background:#6a00ff;color:white;border:none;'
                        'padding:10px 20px;border-radius:8px;cursor:pointer;margin-top:10px;">'
                        "🔮 Open in new tab</button></a>",
                        unsafe_allow_html=True,
                    )

                except Exception as e:
                    st.error(f"3D render failed: {e}")
                    st.warning("⬇️ Falling back to static 3D (matplotlib) view…")
                    fig = self.layered_hypergraph.draw_layered_hypergraph(hyperedges)
                    if fig:
                        st.pyplot(fig)

                return  # don't fall through to the 2D path

            # ── Regular / dual path ───────────────────────────────────────────
            if is_dual:
                fig = self.dual_hypergraph.draw_dual_hypergraph(H)
            else:
                fig = self.hypergraph.draw_hypergraph(
                    H,
                    hyperedges,
                    visualize_mode,
                    highlighted_path=highlighted_path,
                    display_mode=display_mode,
                )

            if fig:
                st.pyplot(fig)
            else:
                st.error(f"Failed to visualize {graph_type}")

        except Exception as e:
            st.error(f"Visualization error: {e}")

    # ──────────────────────────────────────────────────────────────────────────
    def display_data_properties(self, data, data_type):
        """Display properties for atoms, molecules, genes, hypergraphs, or dual hypergraphs."""
        st.write(f"### {data_type} Properties")

        try:
            if data_type in ["Hypergraph", "Dual Hypergraph"]:
                if isinstance(data, hnx.Hypergraph):
                    nodes_list = list(data.nodes())
                    edges_list = list(data.edges())
                    st.write(f"Nodes: {nodes_list}")
                    st.write(f"Number of nodes: {len(nodes_list)}")
                    st.write(f"Edges: {edges_list}")
                    st.write(f"Number of edges: {len(edges_list)}")
                else:
                    st.error(
                        f"Invalid {data_type} data: Expected hypernetx.Hypergraph object"
                    )
                    return

            else:
                FILE_PATHS = {
                    "Atoms": self.data_loader.get_resources_root()
                    / "resources"
                    / "Atoms.json",
                    "Molecules": self.data_loader.get_resources_root()
                    / "resources"
                    / "Mols.json",
                    "Genes": self.data_loader.get_resources_root()
                    / "resources"
                    / "Genes.json",
                }

                if data_type not in FILE_PATHS:
                    st.error(f"Unsupported data type: {data_type}")
                    return

                with open(FILE_PATHS[data_type], "r") as f:
                    json_data = json.load(f)
                    if data_type == "Genes":
                        json_data = json_data["Genes"]

                properties = []
                if data_type == "Atoms":
                    for atom in json_data:
                        props = atom["properties"][0]
                        properties.append(
                            {
                                "Name": atom["name"],
                                "Symbol": props["symbol"],
                                "Atomic Number": props["atomic_number"],
                                "Weight": props["atomic_weight"],
                                "Electron Config": props["electron_configuration"],
                                "Layer": props.get("layer", 0),
                            }
                        )
                    df = pd.DataFrame(properties)
                    st.dataframe(df)
                    col1, col2, col3 = st.columns(3)
                    col1.metric("Total Atoms", len(df))
                    col2.metric("Average Weight", f"{df['Weight'].mean():.2f}")
                    col3.metric("Unique Layers", df["Layer"].nunique())

                elif data_type == "Molecules":
                    for mol in json_data:
                        props = mol["properties"][0]
                        properties.append(
                            {
                                "Name": mol["name"],
                                "Formula": props["molecular_formula"],
                                "Weight": props["molecular_weight"],
                                "IUPAC Name": props["iupac_name"],
                                "Roles": ", ".join(props["roles"]),
                                "Layer": props.get("layer", 0),
                            }
                        )
                    df = pd.DataFrame(properties)
                    st.dataframe(df)
                    col1, col2, col3 = st.columns(3)
                    col1.metric("Total Molecules", len(properties))
                    avg_w = sum(
                        float(m["properties"][0]["molecular_weight"]) for m in json_data
                    ) / len(json_data)
                    col2.metric("Avg Weight", f"{avg_w:.2f}")
                    col3.metric(
                        "Layers Used",
                        len({m["properties"][0].get("layer", 0) for m in json_data}),
                    )

                elif data_type == "Genes":
                    for gene in json_data:
                        props = gene["properties"]
                        properties.append(
                            {
                                "Gene Name": gene["name"],
                                "Chromosome": props["chromosome_location"],
                                "Protein": props["encoded_protein"],
                                "Function": props["function"],
                                "Biological Processes": ", ".join(
                                    props["biological_processes"]
                                ),
                            }
                        )
                    df = pd.DataFrame(properties)
                    st.dataframe(df)
                    col1, col2, col3 = st.columns(3)
                    col1.metric("Total Genes", len(json_data))
                    col2.metric(
                        "Unique Chromosomes",
                        len(
                            {g["properties"]["chromosome_location"] for g in json_data}
                        ),
                    )
                    counts = [
                        len(g["properties"]["biological_processes"]) for g in json_data
                    ]
                    col3.metric(
                        "Avg Processes",
                        f"{sum(counts)/len(counts):.1f}" if counts else "0",
                    )

        except Exception as e:
            st.error(f"Error loading {data_type.lower()} properties: {e}")

    # ──────────────────────────────────────────────────────────────────────────
    def display_table(self, graph, graph_type):
        st.write(f"### {graph_type} Table")
        if graph_type == "Hypergraph":
            edge_data = [
                {
                    "Edge": edge,
                    "Nodes": ", ".join(sorted(data["nodes"])),
                    "Traversable": data["traversable"],
                }
                for edge, data in graph.items()
            ]
        else:
            edge_data = [
                {
                    "Node (Original Edge)": node,
                    "Hyperedges (Original Nodes)": ", ".join(sorted(edges)),
                }
                for node, edges in graph.items()
            ]
        st.dataframe(pd.DataFrame(edge_data))
