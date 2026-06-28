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

# The FastAPI service base URL – change once if the port ever moves
LAYERED_SERVICE_URL = "http://127.0.0.1:8000"


class VisualizationManager:
    def __init__(self):
        self.project_root = DataLoader()
        self.data_loader = DataLoader()
        self.hypergraph = HypergraphManager()
        self.dual_hypergraph = DualHypergraphManager()
        self.layered_hypergraph = LayeredHypergraphManager()

    def serialize_hyperedges(self, hyperedges):
<<<<<<< Updated upstream
        """Convert internal hyperedge dict to a JSON-safe format."""
        clean = {}
=======
        clean = {}

>>>>>>> Stashed changes
        for edge_name, data in hyperedges.items():
            clean[edge_name] = {
                "nodes": list(data["nodes"]),
                "layer": data.get("layer", 0),
            }
<<<<<<< Updated upstream
        return clean

    def _build_layers_payload(self, hyperedges: dict) -> dict:
        """
        Group hyperedges by layer number and return the payload dict that
        /render-layered expects:
            { "0": { "edgeA": ["n1", "n2"], ... }, "1": { ... } }
        """
        layers: dict[str, dict] = {}
        for edge_id, data in hyperedges.items():
            layer_key = str(data.get("layer", 0))
            layers.setdefault(layer_key, {})
            layers[layer_key][edge_id] = list(data["nodes"])
        return layers

    def send_to_layered_service(self, layers: dict) -> dict:
        """
        POST the layers payload to FastAPI and return a normalised result dict:
            {
                "status": <int HTTP code>,
                "scene_id": <str>,
                "viewer_url": <str>,
                "body": <full JSON response>,
            }
        Raises RuntimeError on non-200 so the caller can fall back gracefully.
        """
        url = f"{LAYERED_SERVICE_URL}/render-layered"
        try:
            response = requests.post(url, json={"layers": layers}, timeout=10)
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
=======

        return clean

    def send_to_rust_layered(self, edges_payload):
        url = "http://127.0.0.1:8080/api/visualize"

        response = requests.post(url, json=edges_payload)

        if response.status_code != 200:
            raise RuntimeError(
                f"Rust backend failed [{response.status_code}]: {response.text}"
            )

        return response.json()

>>>>>>> Stashed changes
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

<<<<<<< Updated upstream
            if not st.button(
                f"Visualize {graph_type} ✨", key=f"viz_{tab_key}_{graph_type}"
            ):
                return  # nothing to do until user clicks

            # ── Layered path ──────────────────────────────────────────────────
            if is_layered:
                st.info("⏳ Sending Layered Hypergraph to 3D Service…")

                layers = self._build_layers_payload(hyperedges)

                try:
                    result = self.send_to_layered_service(layers)
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
                                json.dumps({"layers": layers}, indent=2),
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
                    st.warning("⬇️ Falling back to interactive 2D visualization…")
                    self.render_interactive_visualization()

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
=======
            if st.button(
                f"Visualize {graph_type} ✨", key=f"viz_{tab_key}_{graph_type}"
            ):
                if is_layered:
                    st.info("Sending Layered Hypergraph to 3D Service...")

                    # --------- Build request payload (Layered format) ----------
                    edges_array = []

                    for edge_id, data in hyperedges.items():
                        edge_obj = {
                            "id": edge_id,
                            "name": edge_id,
                            "main_properties": [
                                {"key": "type", "p_type": "Simple", "value": ["linked"]}
                            ],
                            "traversable": True,
                            "directed": data.get("directed", False),
                            "head_hyper_nodes": [
                                {"id": node_id} for node_id in data["nodes"]
                            ],
                            "tail_hyper_nodes": None,
                            "layer": data.get("layer", 0),
                        }

                        edges_array.append(edge_obj)

                    request_payload = {
                        "edges": edges_array,
                        "hypergraph_id": "",
                        "hypergraph_name": "",
                    }

                    # --------- Send to microservice ----------
                    result = self.send_to_rust_layered(request_payload)

                    # --------- Layout: 2 columns ----------
                    viewer_url = "http://127.0.0.1:3000/viewer"

                    colA, colB, colC = st.columns([1, 2, 1])
                    with colB:
                        st.markdown(
                            f"""
                                <div style="text-align:center; margin-bottom:15px;">
                                    <a href="{viewer_url}" target="_blank">
                                        <button style="
                                            background:#6a00ff;
                                            color:white;
                                            border:none;
                                            padding:12px 20px;
                                            font-size:18px;
                                            border-radius:10px;
                                            cursor:pointer;
                                        ">
                                            🔮 Open 3D Viewer
                                        </button>
                                    </a>
                                </div>
                                """,
                            unsafe_allow_html=True,
                        )

                    # -------- Request / Response --------
                    col1, col2 = st.columns(2)

                    with col1:
                        with st.expander("📤 Request → FastAPI", expanded=False):
                            st.code(
                                json.dumps(request_payload, indent=2), language="json"
                            )

                    with col2:
                        with st.expander("📥 Response ← Microservice", expanded=False):
                            st.json(result)

                    # -------- Status --------
                    if result.get("status") == "success":
                        st.success("3D Scene Generated Successfully 🚀")
                    else:
                        st.error("Layered service failed")

                    return

                elif is_dual:
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
>>>>>>> Stashed changes

        except Exception as e:
            st.error(f"Visualization error: {e}")

    # ──────────────────────────────────────────────────────────────────────────
    def render_interactive_visualization(self):
        html_path = self.data_loader.get_statics_root() / "statics" / "interactive.html"
        json_path = self.data_loader.get_statics_root() / "statics" / "test_edge.json"

        try:
<<<<<<< Updated upstream
=======
            # Load HTML template
>>>>>>> Stashed changes
            with open(html_path, "r", encoding="utf-8") as f:
                html_content = f.read()

            with open(json_path, "r", encoding="utf-8") as f:
                json_data = json.load(f)

<<<<<<< Updated upstream
=======
            # Inject JSON directly into the HTML as a JS variable
>>>>>>> Stashed changes
            injected_script = (
                f"<script>const injectedData = {json.dumps(json_data)};</script>"
            )

<<<<<<< Updated upstream
            # Patch fetch() so the HTML works without a separate file server
=======
            # Replace the fetch line in HTML with use of the JS variable
>>>>>>> Stashed changes
            html_content = html_content.replace(
                'fetch("test_edge.json")',
                "Promise.resolve({ ok: true, status: 200, json: () => Promise.resolve(injectedData) })",
            )
            html_content = html_content.replace(
                "</head>", f"{injected_script}\n</head>"
            )

            components.html(html_content, height=700, scrolling=True)

        except Exception as e:
            st.error(f"Error loading interactive visualization: {e}")

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
<<<<<<< Updated upstream
                    col1.metric("Total Atoms", len(df))
                    col2.metric("Average Weight", f"{df['Weight'].mean():.2f}")
                    col3.metric("Unique Layers", df["Layer"].nunique())
=======
                    with col1:
                        st.metric("Total Atoms", len(df))
                    with col2:
                        st.metric("Average Weight", f"{df['Weight'].mean():.2f}")
                    with col3:
                        st.metric("Unique Layers", df["Layer"].nunique())
>>>>>>> Stashed changes

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
